/* src/main.rs */

pub(crate) mod common {
    pub(crate) mod requirements {
        #[derive(Debug)]
        pub(crate) enum Error {
            System(),
            Anyhow(),
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                loop {}
            }
        }
        impl std::error::Error for Error {}
        pub(crate) type Result<T> = std::result::Result<T, Error>;
    }
}

pub(crate) mod modules {
    pub(crate) mod stack {
        pub(crate) mod protocol {
            pub(crate) mod application {
                pub(crate) mod http {
                    // MOCKED HYPER TYPES
                    pub(crate) mod mock_hyper {
                        use std::future::Future;
                        use std::pin::Pin;
                        use std::task::{Context, Poll};
                        use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

                        // The culprit: Upgraded is Send but !Sync
                        pub struct Upgraded {
                            // Box<dyn Trait + Send> is NOT Sync
                            pub io: Box<dyn AsyncReadWrite + Send>,
                        }

                        pub trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin {}
                        impl<T: AsyncRead + AsyncWrite + Unpin> AsyncReadWrite for T {}

                        impl AsyncRead for Upgraded {
                            fn poll_read(
                                mut self: Pin<&mut Self>,
                                cx: &mut Context<'_>,
                                buf: &mut ReadBuf<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                Pin::new(&mut self.io).poll_read(cx, buf)
                            }
                        }

                        impl AsyncWrite for Upgraded {
                            fn poll_write(
                                mut self: Pin<&mut Self>,
                                cx: &mut Context<'_>,
                                buf: &[u8],
                            ) -> Poll<std::io::Result<usize>> {
                                Pin::new(&mut self.io).poll_write(cx, buf)
                            }
                            fn poll_flush(
                                mut self: Pin<&mut Self>,
                                cx: &mut Context<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                Pin::new(&mut self.io).poll_flush(cx)
                            }
                            fn poll_shutdown(
                                mut self: Pin<&mut Self>,
                                cx: &mut Context<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                Pin::new(&mut self.io).poll_shutdown(cx)
                            }
                        }

                        pub type OnUpgrade =
                            Pin<Box<dyn Future<Output = Result<Upgraded, ()>> + Send>>;

                        pub struct Incoming; // Dummy
                    }

                    pub(crate) mod wrapper {
                        use super::mock_hyper::{Incoming, OnUpgrade};
                        use std::future::Future;
                        use std::pin::Pin;

                        pub(crate) enum VaneBody {
                            Hyper(Incoming),
                            SwitchingProtocols(OnUpgrade),
                            UpgradeBridge {
                                tunnel_task:
                                    Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
                            },
                            Empty,
                        }
                    }

                    pub(crate) mod httpx {
                        use super::mock_hyper::{Incoming, Upgraded};
                        use super::wrapper::VaneBody;
                        use crate::common::requirements::Error;
                        use crate::common::requirements::Result;
                        use crate::modules::stack::protocol::application::container::{
                            Container, PayloadState,
                        };
                        use http::{Request, Response};
                        use tokio::io::{AsyncRead, AsyncWrite};
                        use tokio::sync::oneshot;

                        pub(crate) trait ByteStream:
                            AsyncRead + AsyncWrite + Unpin + Send + Sync
                        {
                        }
                        impl<T: AsyncRead + AsyncWrite + Unpin + Send + Sync> ByteStream for T {}

                        // The outer future structure
                        pub(crate) async fn handle_connection(
                            _conn: Box<dyn ByteStream>,
                            protocol_id: String,
                        ) -> Result<()> {
                            // Mocking the service_fn structure
                            let service = move |req: Request<Incoming>| {
                                let proto = protocol_id.clone();
                                async move { serve_request(req, proto).await }
                            };

                            // Mocking the serve loop
                            let _ = service(Request::new(Incoming)).await;
                            Ok(())
                        }

                        async fn dummy_flow_step(_container: &mut Container) {
                            tokio::task::yield_now().await;
                        }

                        async fn serve_request(
                            mut req: Request<Incoming>,
                            _protocol_id: String,
                        ) -> std::result::Result<Response<Incoming>, Error>
                        {
                            let client_upgrade_handle =
                                if req.headers().contains_key(http::header::UPGRADE) {
                                    // Mocking hyper::upgrade::on
                                    Some(Box::pin(async {
                                        Ok(Upgraded {
                                            io: Box::new(tokio::io::empty()),
                                        })
                                    })
                                        as super::mock_hyper::OnUpgrade)
                                } else {
                                    None
                                };

                            let (mut parts, _body) = req.into_parts();
                            let request_payload = PayloadState::Http(VaneBody::Hyper(Incoming));
                            let response_payload = PayloadState::Empty;
                            let (res_tx, res_rx) = oneshot::channel::<Response<()>>();

                            let mut container =
                                Container::new(request_payload, response_payload, Some(res_tx));
                            container.client_upgrade = client_upgrade_handle;
                            let _ = parts.headers;

                            dummy_flow_step(&mut container).await;

                            match res_rx.await {
                                Ok(_response_parts) => {
                                    let mut payload = std::mem::replace(
                                        &mut container.response_body,
                                        PayloadState::Empty,
                                    );
                                    if let PayloadState::Http(VaneBody::SwitchingProtocols(
                                        upstream_upgrade,
                                    )) = payload
                                    {
                                        if let Some(client_upgrade) =
                                            container.client_upgrade.take()
                                        {
                                            // ICE Trigger Zone
                                            let tunnel_future = Box::pin(async move {
                                                tokio::task::yield_now().await;
                                                // Upgraded is !Sync.
                                                // try_join awaits both.
                                                // The resulting future holds !Sync state across yield points.
                                                match tokio::try_join!(
                                                    client_upgrade,
                                                    upstream_upgrade
                                                ) {
                                                    Ok((mut client_io, mut upstream_io)) => {
                                                        // copy_bidirectional requires AsyncRead+AsyncWrite
                                                        // Our Upgraded implements it.
                                                        let _ = tokio::io::copy_bidirectional(
                                                            &mut client_io,
                                                            &mut upstream_io,
                                                        )
                                                        .await;
                                                    }
                                                    Err(_) => {}
                                                }
                                            });

                                            // ICE: coercing !Sync future to Sync trait object
                                            payload = PayloadState::Http(VaneBody::UpgradeBridge {
                                                tunnel_task: Some(tunnel_future),
                                            });
                                        } else {
                                            payload = PayloadState::Empty;
                                        }
                                    }
                                    loop {}
                                }
                                Err(_) => loop {},
                            }
                        }
                    }
                }

                pub(crate) mod container {
                    use crate::modules::stack::protocol::application::http::mock_hyper::OnUpgrade;
                    use crate::modules::stack::protocol::application::http::wrapper::VaneBody;
                    use http::Response;
                    use tokio::sync::oneshot;

                    pub(crate) enum PayloadState {
                        Http(VaneBody),
                        Empty,
                    }

                    pub(crate) struct Container {
                        pub(crate) response_body: PayloadState,
                        pub(crate) client_upgrade: Option<OnUpgrade>,
                    }

                    impl Container {
                        pub(crate) fn new(
                            _request_body: PayloadState,
                            response_body: PayloadState,
                            _response_tx: Option<oneshot::Sender<Response<()>>>,
                        ) -> Self {
                            loop {}
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    loop {}
}
