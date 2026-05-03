use tokio::io::AsyncReadExt;

use crate::api::capabilites::APIS;
use crate::error::KafkaError;
use crate::protocol::reader::{Bytes8, Bytes16, Bytes32, BytesString, StreamReader};

#[derive(Debug)]
pub struct RequestHeader {
    pub key: Bytes16,
    pub version: Bytes16,
    pub correlation_id: Bytes32,
    pub client: BytesString,
    pub tag_buffer: Bytes8,
}

impl StreamReader for RequestHeader {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> crate::error::Result<Self> {
        let key = Bytes16::from_stream(stream).await?;
        let version = Bytes16::from_stream(stream).await?;
        let correlation_id = Bytes32::from_stream(stream).await?;
        let client = BytesString::from_stream(stream).await?;
        let tag_buffer = Bytes8::from_stream(stream).await?;

        Ok(Self {
            key,
            version,
            correlation_id,
            client,
            tag_buffer,
        })
    }
}

impl RequestHeader {
    pub fn check_version(&self) -> Result<(), KafkaError> {
        let (key, version) = (self.key.0, self.version.0);
        let api = APIS
            .iter()
            .find(|api| api.key == key)
            .ok_or_else(|| KafkaError::UnsupportedKey(key))?;
        match api.version_range.contains(&version) {
            true => Ok(()),
            false => Err(KafkaError::UnsupportedVersion(key, version)),
        }
    }
}
