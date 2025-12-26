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

    type OnUpgrade = Pin<Box<dyn Future<Output = Result<Upgraded, ()>> + Send>>;

    pub(crate) async fn handle_connection() {
        let service = || async move { serve_request().await };
        let _ = service().await;
    }

    async fn serve_request() {
        let mut client: Option<OnUpgrade> = Some(Box::pin(async {
            Ok(Upgraded {
                io: Box::new(tokio::io::empty()),
            })
        }));
        let upstream: OnUpgrade = Box::pin(async {
            Ok(Upgraded {
                io: Box::new(tokio::io::empty()),
            })
        });
        tokio::task::yield_now().await;
        if let Some(c) = client.take() {
            let tunnel_future = Box::pin(async move {
                tokio::task::yield_now().await;
                match tokio::try_join!(c, upstream) {
                    Ok((mut c_io, mut u_io)) => {
                        let _ = tokio::io::copy_bidirectional(&mut c_io, &mut u_io).await;
                    }
                    Err(_) => {}
                }
            });
            let _target: Pin<Box<dyn Future<Output = ()> + Send + Sync>> = tunnel_future;
        }
        loop {}
    }
}

#[tokio::main]
async fn main() {
    loop {}
}
