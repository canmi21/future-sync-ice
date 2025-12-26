/* src/main.rs */

pub(crate) mod common {
    pub(crate) mod requirements {
        #[derive(Debug)]
        pub(crate) enum Error {
            System(),
            Anyhow(),
        }

        pub(crate) type Result<T> = std::result::Result<T, Error>;
    }
}

pub(crate) mod modules {
    pub(crate) mod stack {
        pub(crate) mod protocol {
            pub(crate) mod application {
                pub(crate) mod http {
                    pub(crate) mod mock_hyper {
                        use std::pin::Pin;
                        use std::task::Context;
                        use std::task::Poll;
                        use tokio::io::AsyncRead;
                        use tokio::io::AsyncWrite;
                        use tokio::io::ReadBuf;

                        pub(crate) struct Upgraded {
                            pub(crate) io: Box<dyn AsyncReadWrite + Send>,
                        }

                        pub(crate) trait AsyncReadWrite:
                            AsyncRead + AsyncWrite + Unpin
                        {
                        }

                        impl<T: AsyncRead + AsyncWrite + Unpin> AsyncReadWrite for T {}

                        impl AsyncRead for Upgraded {
                            fn poll_read(
                                self: Pin<&mut Self>,
                                _cx: &mut Context<'_>,
                                _buf: &mut ReadBuf<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                loop {}
                            }
                        }

                        impl AsyncWrite for Upgraded {
                            fn poll_write(
                                self: Pin<&mut Self>,
                                _cx: &mut Context<'_>,
                                _buf: &[u8],
                            ) -> Poll<std::io::Result<usize>> {
                                loop {}
                            }

                            fn poll_flush(
                                self: Pin<&mut Self>,
                                _cx: &mut Context<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                loop {}
                            }

                            fn poll_shutdown(
                                self: Pin<&mut Self>,
                                _cx: &mut Context<'_>,
                            ) -> Poll<std::io::Result<()>> {
                                loop {}
                            }
                        }

                        pub(crate) type OnUpgrade =
                            Pin<Box<dyn Future<Output = Result<Upgraded, ()>> + Send>>;

                        pub(crate) struct Incoming;
                    }

                    pub(crate) mod wrapper {
                        use super::mock_hyper::Incoming;
                        use super::mock_hyper::OnUpgrade;
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
                        use super::mock_hyper::Incoming;
                        use super::mock_hyper::Upgraded;
                        use super::wrapper::VaneBody;
                        use crate::common::requirements::Error;
                        use crate::common::requirements::Result;
                        use crate::modules::stack::protocol::application::container::Container;
                        use crate::modules::stack::protocol::application::container::PayloadState;
                        use tokio::io::AsyncRead;
                        use tokio::io::AsyncWrite;
                        use tokio::sync::oneshot;

                        pub(crate) struct Request<T>(pub T);

                        pub(crate) struct Response<T>(pub T);

                        pub(crate) trait ByteStream:
                            AsyncRead + AsyncWrite + Unpin + Send + Sync
                        {
                        }

                        pub(crate) async fn handle_connection(
                            _conn: Box<dyn ByteStream>,
                            protocol_id: String,
                        ) -> Result<()> {
                            let service = move |req: Request<Incoming>| {
                                let proto = protocol_id.clone();
                                async move { serve_request(req, proto).await }
                            };
                            let _ = service(Request(Incoming)).await;
                            Ok(())
                        }

                        async fn dummy_flow_step(_container: &mut Container) {
                            loop {}
                        }

                        async fn serve_request(
                            req: Request<Incoming>,
                            _protocol_id: String,
                        ) -> std::result::Result<Response<Incoming>, Error>
                        {
                            let client_upgrade_handle = if true {
                                Some(Box::pin(async {
                                    Ok(Upgraded {
                                        io: Box::new(tokio::io::empty()),
                                    })
                                })
                                    as super::mock_hyper::OnUpgrade)
                            } else {
                                None
                            };
                            let _body = req.0;
                            let request_payload = PayloadState::Http(VaneBody::Hyper(Incoming));
                            let response_payload = PayloadState::Empty;
                            let (res_tx, res_rx) = oneshot::channel::<Response<()>>();
                            let mut container =
                                Container::new(request_payload, response_payload, Some(res_tx));
                            container.client_upgrade = client_upgrade_handle;
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
                                            let tunnel_future = Box::pin(async move {
                                                tokio::task::yield_now().await;
                                                match tokio::try_join!(
                                                    client_upgrade,
                                                    upstream_upgrade
                                                ) {
                                                    Ok((mut client_io, mut upstream_io)) => {
                                                        let _ = tokio::io::copy_bidirectional(
                                                            &mut client_io,
                                                            &mut upstream_io,
                                                        )
                                                        .await;
                                                    }
                                                    Err(_) => {}
                                                }
                                            });
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
                    use crate::modules::stack::protocol::application::http::httpx::Response;
                    use crate::modules::stack::protocol::application::http::mock_hyper::OnUpgrade;
                    use crate::modules::stack::protocol::application::http::wrapper::VaneBody;
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
