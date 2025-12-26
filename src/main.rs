/* src/main.rs */

pub(crate) mod modules {
    pub(crate) mod stack {
        pub(crate) mod protocol {
            pub(crate) mod application {
                pub(crate) mod http {
                    pub(crate) mod mock_hyper {
                        use std::future::Future;
                        use std::pin::Pin;
                        use std::task::{Context, Poll};
                        use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

                        // The wrapper that makes things !Sync
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
                        use super::mock_hyper::{Incoming, OnUpgrade};
                        use std::future::Future;
                        use std::pin::Pin;

                        pub(crate) enum VaneBody {
                            Hyper(Incoming),
                            SwitchingProtocols(OnUpgrade),
                            UpgradeBridge {
                                // The Unsize Coercion target
                                tunnel_task:
                                    Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
                            },
                            Empty,
                        }
                    }

                    pub(crate) mod httpx {
                        use super::mock_hyper::{Incoming, Upgraded};
                        use super::wrapper::VaneBody;
                        use crate::modules::stack::protocol::application::container::{
                            Container, PayloadState,
                        };

                        // Minimal wrapper to maintain the async closure structure
                        pub(crate) async fn handle_connection() {
                            let service = || async move { serve_request().await };
                            let _ = service().await;
                        }

                        async fn serve_request() {
                            // 1. Setup Client Upgrade (Mocked)
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

                            // 2. Setup Container
                            let mut container = Container {
                                response_body: PayloadState::Http(VaneBody::Hyper(Incoming)),
                                client_upgrade: client_upgrade_handle,
                            };

                            // 3. Simulate "Logic" execution gap
                            tokio::task::yield_now().await;

                            // 4. Manually set state to "SwitchingProtocols" (Simulating what flow/oneshot did)
                            container.response_body =
                                PayloadState::Http(VaneBody::SwitchingProtocols(Box::pin(async {
                                    Ok(Upgraded {
                                        io: Box::new(tokio::io::empty()),
                                    })
                                })));

                            // 5. The Crash Site
                            let mut payload = std::mem::replace(
                                &mut container.response_body,
                                PayloadState::Empty,
                            );

                            if let PayloadState::Http(VaneBody::SwitchingProtocols(
                                upstream_upgrade,
                            )) = payload
                            {
                                if let Some(client_upgrade) = container.client_upgrade.take() {
                                    // Creating the !Sync future
                                    let tunnel_future = Box::pin(async move {
                                        tokio::task::yield_now().await;
                                        match tokio::try_join!(client_upgrade, upstream_upgrade) {
                                            Ok((mut client_io, mut upstream_io)) => {
                                                // copy_bidirectional holds &mut Upgraded across await
                                                let _ = tokio::io::copy_bidirectional(
                                                    &mut client_io,
                                                    &mut upstream_io,
                                                )
                                                .await;
                                            }
                                            Err(_) => {}
                                        }
                                    });

                                    // Coercion to Send + Sync triggers ICE
                                    payload = PayloadState::Http(VaneBody::UpgradeBridge {
                                        tunnel_task: Some(tunnel_future),
                                    });
                                } else {
                                    payload = PayloadState::Empty;
                                }
                            }

                            // Prevent optimization
                            loop {}
                        }
                    }
                }

                pub(crate) mod container {
                    use crate::modules::stack::protocol::application::http::mock_hyper::OnUpgrade;
                    use crate::modules::stack::protocol::application::http::wrapper::VaneBody;

                    pub(crate) enum PayloadState {
                        Http(VaneBody),
                        Empty,
                    }

                    pub(crate) struct Container {
                        pub(crate) response_body: PayloadState,
                        pub(crate) client_upgrade: Option<OnUpgrade>,
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
