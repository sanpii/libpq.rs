pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    NulError(#[from] std::ffi::NulError),
    #[error("{0}")]
    Backend(String),
    #[error("Large object error")]
    LargeObject,
    #[error("Invalid SSL attribute: '{0}'")]
    InvalidSslAttribute(String),
    #[error("Timeout")]
    Timeout,
    #[error("Unknow error")]
    Unknow,
    #[error("{0}")]
    Utf8(#[from] std::str::Utf8Error),
}
