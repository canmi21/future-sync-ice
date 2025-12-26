#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("IO Error: {0}")]
    Io(String),
    #[error("TLS Error: {0}")]
    Tls(String),
    #[error("Configuration Error: {0}")]
    Configuration(String),
    #[error("System Error: {0}")]
    System(String),
    #[error("Not Implemented: {0}")]
    NotImplemented(String),
    #[error("Anyhow: {0}")]
    Anyhow(
        #[from]
        anyhow::Error,
    ),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
