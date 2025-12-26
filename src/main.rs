/* src/main.rs */

pub(crate) mod modules {
    pub(crate) mod stack {
        pub(crate) mod protocol {
            pub(crate) mod application {
                pub(crate) mod http {
                    // Keep mock_hyper separate to preserve some structure,
                    // but minimize it to just the IO types.
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
                    }

                    pub(crate) mod httpx {
                        use super::mock_hyper::{OnUpgrade, Upgraded};
                        use std::future::Future;
                        use std::pin::Pin;

                        // INLINED: VaneBody from wrapper
                        pub(crate) enum VaneBody {
                            SwitchingProtocols(OnUpgrade),
                            UpgradeBridge {
                                tunnel_task:
                                    Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
                            },
                            Empty,
                        }

                        // INLINED: Container/PayloadState from container
                        pub(crate) enum PayloadState {
                            Http(VaneBody),
                            Empty,
                        }

                        pub(crate) struct Container {
                            pub(crate) response_body: PayloadState,
                            pub(crate) client_upgrade: Option<OnUpgrade>,
                        }

                        pub(crate) async fn handle_connection() {
                            let service = || async move { serve_request().await };
                            let _ = service().await;
                        }

                        async fn serve_request() {
                            // 1. Setup Client Upgrade
                            let client_upgrade_handle = if true {
                                Some(Box::pin(async {
                                    Ok(Upgraded {
                                        io: Box::new(tokio::io::empty()),
                                    })
                                }) as OnUpgrade)
                            } else {
                                None
                            };

                            let mut container = Container {
                                response_body: PayloadState::Empty, // Simplified initial state
                                client_upgrade: client_upgrade_handle,
                            };

                            tokio::task::yield_now().await;

                            // 2. Set state to SwitchingProtocols
                            container.response_body =
                                PayloadState::Http(VaneBody::SwitchingProtocols(Box::pin(async {
                                    Ok(Upgraded {
                                        io: Box::new(tokio::io::empty()),
                                    })
                                })));

                            let mut payload = std::mem::replace(
                                &mut container.response_body,
                                PayloadState::Empty,
                            );

                            // 3. The Crash Logic
                            if let PayloadState::Http(VaneBody::SwitchingProtocols(
                                upstream_upgrade,
                            )) = payload
                            {
                                if let Some(client_upgrade) = container.client_upgrade.take() {
                                    let tunnel_future = Box::pin(async move {
                                        tokio::task::yield_now().await;
                                        match tokio::try_join!(client_upgrade, upstream_upgrade) {
                                            Ok((mut client_io, mut upstream_io)) => {
                                                // tokio::io::copy_bidirectional is the key here
                                                let _ = tokio::io::copy_bidirectional(
                                                    &mut client_io,
                                                    &mut upstream_io,
                                                )
                                                .await;
                                            }
                                            Err(_) => {}
                                        }
                                    });

                                    // ICE happens here during coercion
                                    payload = PayloadState::Http(VaneBody::UpgradeBridge {
                                        tunnel_task: Some(tunnel_future),
                                    });
                                } else {
                                    payload = PayloadState::Empty;
                                }
                            }
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
