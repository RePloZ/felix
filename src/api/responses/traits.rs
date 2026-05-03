use bytes::BytesMut;

use crate::api::requests::RequestHeader;

pub trait ResponseBytes {
    fn to_bytes(&self) -> BytesMut;
}

pub trait ResponseHeader: ResponseBytes {
    fn from_request(header: &RequestHeader) -> Self;
}

pub trait ResponseBody<RequestBody>: ResponseBytes {
    fn from_request(header: &RequestHeader, body: &RequestBody) -> Self;
}
