use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::{self, Result};
use crate::protocol::reader::{Bytes8, Bytes16, Bytes32, Bytes64, Bytes128, StreamReader};

pub struct RecordBatch {
    pub base_offset: Bytes64,
    pub batch_len: Bytes32,
    pub partition_leader_epoch: Bytes32,
    pub magic_byte: Bytes8,
    pub crc: Bytes32,
    pub attributes: RecordAttributes,
    pub last_offset_delta: Bytes32,
    pub base_timestamp: Bytes64,
    pub max_timestamp: Bytes64,
    pub producer_id: Bytes64,
    pub producer_epoch: Bytes16,
    pub base_sequence: Bytes32,
    pub records_length: Bytes32,
    pub records: Vec<RecordElement>,
}

pub struct RecordElement {
    pub len: Bytes8,
    pub attributes: Bytes8,
    pub delta_timestamp: Bytes8,
    pub offset_delta: Bytes8,
    pub key_len: Bytes8,
    pub value_len: Bytes8,
    pub value: RecordValue,
    pub headers_array_count: Bytes8,
}

pub struct RecordValue {
    pub frame_version: Bytes8,
    pub types: Bytes8,
    pub version: Bytes8,
    pub name_len: Bytes8,
    pub name: Bytes128,
    pub feature_level: Bytes16,
    pub tagged_fields_count: Bytes8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordAttributes {
    pub compression: Compression,
    pub timestamp_type: bool,
    pub is_transactional: bool,
    pub is_control_batch: bool,
    pub has_delete_horizon_ms: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

impl TryFrom<u8> for Compression {
    type Error = error::RequestError;

    fn try_from(val: u8) -> std::result::Result<Self, Self::Error> {
        match val {
            0 => Ok(Compression::None),
            1 => Ok(Compression::Gzip),
            2 => Ok(Compression::Snappy),
            3 => Ok(Compression::Lz4),
            4 => Ok(Compression::Zstd),
            _ => Err(error::RequestError::InvalidCompression(val)),
        }
    }
}

pub async fn read_files() -> io::Result<Vec<RecordBatch>> {
    let path = "/tmp/kraft-combined-logs/__cluster_metadata-0/00000000000000000000.log";
    let mut stream = File::open(path).await?;

    let mut records_bath = Vec::new();

    while let Ok(record_bath) = RecordBatch::from_stream(&mut stream).await {
        records_bath.push(record_bath);
    }

    Ok(records_bath)
}

impl StreamReader for RecordBatch {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let base_offset = Bytes64::from_stream(stream).await?;
        let batch_len = Bytes32::from_stream(stream).await?;
        let partition_leader_epoch = Bytes32::from_stream(stream).await?;
        let magic_byte = Bytes8::from_stream(stream).await?;
        let crc = Bytes32::from_stream(stream).await?;
        let attributes = RecordAttributes::from_stream(stream).await?;
        let last_offset_delta = Bytes32::from_stream(stream).await?;
        let base_timestamp = Bytes64::from_stream(stream).await?;
        let max_timestamp = Bytes64::from_stream(stream).await?;
        let producer_id = Bytes64::from_stream(stream).await?;
        let producer_epoch = Bytes16::from_stream(stream).await?;
        let base_sequence = Bytes32::from_stream(stream).await?;
        let records_length = Bytes32::from_stream(stream).await?;
        let mut records = Vec::new();

        for _ in 0..records_length.0 {
            let record = RecordElement::from_stream(stream).await?;
            records.push(record);
        }

        Ok(RecordBatch {
            base_offset,
            batch_len,
            partition_leader_epoch,
            magic_byte,
            crc,
            attributes,
            last_offset_delta,
            base_timestamp,
            max_timestamp,
            producer_id,
            producer_epoch,
            base_sequence,
            records_length,
            records,
        })
    }
}

impl StreamReader for RecordAttributes {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).await?;

        let val = u16::from_be_bytes(buf);

        let compression = (val & 0b0111) as u8;
        let timestamp_type = (val & 0b1000) != 0;
        let is_transactional = (val & 0b1_0000) != 0;
        let is_control_batch = (val & 0b10_0000) != 0;
        let has_delete_horizon_ms = (val & 0b100_0000) != 0;

        Ok(Self {
            compression: compression.try_into()?,
            timestamp_type,
            is_transactional,
            is_control_batch,
            has_delete_horizon_ms,
        })
    }
}

impl StreamReader for RecordElement {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let len = Bytes8::from_stream(stream).await?;
        let attributes = Bytes8::from_stream(stream).await?;
        let delta_timestamp = Bytes8::from_stream(stream).await?;
        let offset_delta = Bytes8::from_stream(stream).await?;
        let key_len = Bytes8::from_stream(stream).await?;
        let value_len = Bytes8::from_stream(stream).await?;
        let value = RecordValue::from_stream(stream).await?;
        let headers_array_count = Bytes8::from_stream(stream).await?;

        Ok(RecordElement {
            len,
            attributes,
            delta_timestamp,
            offset_delta,
            key_len,
            value_len,
            value,
            headers_array_count,
        })
    }
}

impl StreamReader for RecordValue {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let frame_version = Bytes8::from_stream(stream).await?;
        let types = Bytes8::from_stream(stream).await?;
        let version = Bytes8::from_stream(stream).await?;
        let name_len = Bytes8::from_stream(stream).await?;
        let name = Bytes128::from_stream(stream).await?;
        let feature_level = Bytes16::from_stream(stream).await?;
        let tagged_fields_count = Bytes8::from_stream(stream).await?;
        Ok(Self {
            frame_version,
            types,
            version,
            name_len,
            name,
            feature_level,
            tagged_fields_count,
        })
    }
}
