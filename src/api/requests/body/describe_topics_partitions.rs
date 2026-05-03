use tokio::io::AsyncReadExt;

use crate::error::Result;
use crate::protocol::reader::{
    Bytes8, Bytes32, BytesCompactArray, BytesCompactString, StreamReader,
};

#[derive(Debug)]
pub struct ReqDescribeTopicPartitions {
    pub topics: BytesCompactArray<ReqTopicParition>,
    pub partition_limit: Bytes32,
    pub cursor: Bytes8,
    pub tag_buffer: Bytes8,
}

#[derive(Debug)]
pub struct ReqTopicParition {
    pub name: BytesCompactString,
    pub tag_buffer: Bytes8,
}

impl StreamReader for ReqDescribeTopicPartitions {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let topics = BytesCompactArray::from_stream(stream).await?;
        let partition_limit = Bytes32::from_stream(stream).await?;
        let cursor = Bytes8::from_stream(stream).await?;
        let tag_buffer = Bytes8::from_stream(stream).await?;

        Ok(Self {
            topics,
            partition_limit,
            cursor,
            tag_buffer,
        })
    }
}

impl StreamReader for ReqTopicParition {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let name = BytesCompactString::from_stream(stream).await?;
        let tag_buffer = Bytes8::from_stream(stream).await?;

        Ok(Self { name, tag_buffer })
    }
}
