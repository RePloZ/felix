use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiErrors {
    #[error("Key is not supported: {0}")]
    UnsupportedKey(u16),
    #[error("Unsupported version: {0} in api_key {1}")]
    UnsupportedVersion(u16, u16),
}

impl ApiErrors {
    pub fn into_error_code(&self) -> u16 {
        match self {
            ApiErrors::UnsupportedKey(_) => 36,
            ApiErrors::UnsupportedVersion(_, _) => 35,
        }
    }
}

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("unexpected end of buffer")]
    UnexpectedEof,
    #[error("invalid data: {0}")]
    InvalidData(#[from] std::io::Error),
    #[error("invalid utf8 data: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
