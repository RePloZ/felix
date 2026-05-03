use tokio::io::AsyncReadExt;

use crate::error::Result;
use crate::protocol::reader::{Bytes8, BytesCompactString, StreamReader};

#[derive(Debug)]
pub struct ApiVersionsRequest {
    pub client: BytesCompactString,
    pub version: BytesCompactString,
    pub tag_buffer: Bytes8,
}

impl StreamReader for ApiVersionsRequest {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let client = BytesCompactString::from_stream(stream).await?;
        let version = BytesCompactString::from_stream(stream).await?;
        let tag_buffer = Bytes8::from_stream(stream).await?;

        Ok(Self {
            client,
            version,
            tag_buffer,
        })
    }
}
