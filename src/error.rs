use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RequestError {
    #[error("Invalid compression value: {0}")]
    InvalidCompression(u8),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Number conversion is impossible {0}")]
    NumConversion(#[from] std::convert::Infallible),

    #[error("Number conversion is impossible {0}")]
    Kafka(#[from] KafkaError),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Request failed for api_key={api_key}, correlation_id={correlation_id}")]
    Request {
        api_key: u16,
        correlation_id: u32,
        #[source]
        source: Box<Self>,
    },
}

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Unsupported API key: {0}")]
    UnsupportedKey(u16),

    #[error("Unsupported version: {1} for API key {0}")]
    UnsupportedVersion(u16, u16),

    #[error("Unsupported Topic Or Partition: {0}")]
    UnknownTopicOrPartition(u16),
}

impl RequestError {
    pub fn with_request_context(self, api_key: u16, correlation_id: u32) -> Self {
        Self::Request {
            api_key,
            correlation_id,
            source: Box::new(self),
        }
    }
}

impl KafkaError {
    pub fn into_error_code(&self) -> u16 {
        match self {
            Self::UnsupportedKey(_) => 36,
            Self::UnsupportedVersion(_, _) => 35,
            Self::UnknownTopicOrPartition(_) => 3,
        }
    }
}

pub type Result<T> = std::result::Result<T, RequestError>;
