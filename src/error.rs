#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Backend(crate::message::Notice),
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Connect(String),
    #[error("another command is already in progress")]
    InQuery,
    #[error("{0}")]
    InvalidState(String),
    #[error("Invalid backend response. Type: {0}, payload: {1:?}")]
    InvalidResponse(char, Vec<u8>),
    #[error("{0}")]
    Parse(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Unable to lock TCP stream")]
    RwLock,
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknow")]
    Unknow,
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::RwLock
    }
}
