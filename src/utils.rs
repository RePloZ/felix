use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResponseApiError {
    #[error("Response Error: ")]
    UnsupportedKey(u16),
    #[error("Response Error: ")]
    UnsupportedVersion(u16),
}

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("unexpected end of buffer")]
    UnexpectedEof,
    #[error("invalid data: {0}")]
    InvalidData(#[from] std::io::Error),
}

impl Into<u16> for ResponseApiError {
    fn into(self) -> u16 {
        match self {
            ResponseApiError::UnsupportedKey(_) => 36,
            ResponseApiError::UnsupportedVersion(_) => 35,
        }
    }
}
