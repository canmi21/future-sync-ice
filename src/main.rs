/* src/main.rs */

pub(crate) mod repro {
    use std::pin::Pin;
    use std::task::Context;
    use std::task::Poll;
    use tokio::io::AsyncRead;
    use tokio::io::AsyncWrite;
    use tokio::io::ReadBuf;

    pub(crate) struct Upgraded {
        pub(crate) io: Box<dyn AsyncReadWrite + Send>,
    }

    pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin {}

    impl<T: AsyncRead + AsyncWrite + Unpin> AsyncReadWrite for T {}

    impl AsyncRead for Upgraded {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            _: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            loop {}
        }
    }

    impl AsyncWrite for Upgraded {
        fn poll_write(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            _: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            loop {}
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            loop {}
        }

        fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            loop {}
        }
    }

    pub(crate) type OnUpgrade = Pin<Box<dyn Future<Output = Result<Upgraded, ()>> + Send>>;

    pub(crate) enum VaneBody {
        SwitchingProtocols(OnUpgrade),
        UpgradeBridge {
            tunnel_task: Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
        },
        Empty,
    }

    pub(crate) struct Container {
        pub(crate) body: VaneBody,
        pub(crate) upgrade: Option<OnUpgrade>,
    }

    pub(crate) async fn handle_connection() {
        let service = || async move { serve_request().await };
        let _ = service().await;
    }

    async fn serve_request() {
        let mut container = Container {
            body: VaneBody::Empty,
            upgrade: Some(Box::pin(async {
                Ok(Upgraded {
                    io: Box::new(tokio::io::empty()),
                })
            }) as OnUpgrade),
        };
        tokio::task::yield_now().await;
        container.body = VaneBody::SwitchingProtocols(Box::pin(async {
            Ok(Upgraded {
                io: Box::new(tokio::io::empty()),
            })
        }));
        let payload = std::mem::replace(&mut container.body, VaneBody::Empty);
        if let VaneBody::SwitchingProtocols(upstream) = payload {
            if let Some(client) = container.upgrade.take() {
                let tunnel_future = Box::pin(async move {
                    tokio::task::yield_now().await;
                    match tokio::try_join!(client, upstream) {
                        Ok((mut c, mut u)) => {
                            let _ = tokio::io::copy_bidirectional(&mut c, &mut u).await;
                        }
                        Err(_) => {}
                    }
                });
                container.body = VaneBody::UpgradeBridge {
                    tunnel_task: Some(tunnel_future),
                };
            }
        }
        loop {}
    }
}

#[tokio::main]
async fn main() {
    loop {}
}
