use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResponseApiError {
    #[error("Response Error: ")]
    UnsupportedKey(u16),
    #[error("Response Error: ")]
    UnsupportedVersion(u16),
}

impl Into<u16> for ResponseApiError {
    fn into(self) -> u16 {
        match self {
            ResponseApiError::UnsupportedKey(_) => 36,
            ResponseApiError::UnsupportedVersion(_) => 35,
        }
    }
}
