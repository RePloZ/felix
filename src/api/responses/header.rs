use bytes::{BufMut, BytesMut};

use crate::api::requests::RequestHeader;
use crate::api::responses::traits::{ResponseBytes, ResponseHeader};

#[derive(Debug)]
pub struct ResHeaderV0 {
    pub correalation_id: u32,
}

#[derive(Debug)]
pub struct ResHeaderV1 {
    pub correalation_id: u32,
    pub tag_buffer: u8,
}

impl ResponseBytes for ResHeaderV0 {
    fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(4);
        buf.put_u32(self.correalation_id);
        buf
    }
}

impl ResponseBytes for ResHeaderV1 {
    fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(5);
        buf.put_u32(self.correalation_id);
        buf.put_u8(self.tag_buffer);
        buf
    }
}

impl ResponseHeader for ResHeaderV0 {
    fn from_request(header: &RequestHeader) -> Self {
        Self {
            correalation_id: header.correlation_id.0,
        }
    }
}

impl ResponseHeader for ResHeaderV1 {
    fn from_request(header: &RequestHeader) -> Self {
        Self {
            correalation_id: header.correlation_id.0,
            tag_buffer: header.tag_buffer.0,
        }
    }
}
