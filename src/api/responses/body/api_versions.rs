use std::ops::RangeInclusive;

use bytes::{BufMut, BytesMut};

use crate::api::capabilites::{APIS, ApiInfo};
use crate::api::requests::{ApiVersionsRequest, RequestHeader};
use crate::api::responses::traits::{ResponseBody, ResponseBytes};

#[derive(Debug)]
pub struct ApiVersionsBody {
    pub error_code: u16,
    pub versions: Vec<ApiVersionsVer>,
    pub throttle_time: u32,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiVersionsVer {
    pub api_key: u16,
    pub version_range: RangeInclusive<u16>,
    pub tag_buffer: u8,
}

impl ResponseBytes for ApiVersionsBody {
    fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(13);
        buf.put_u16(self.error_code);
        buf.put_u8(self.versions.len() as u8 + 1);
        let version_bytes = self
            .versions
            .iter()
            .fold(BytesMut::new(), |mut acc, version| {
                acc.extend(version.to_bytes());
                acc
            });
        buf.extend(version_bytes);
        buf.put_u32(self.throttle_time);
        buf.put_u8(self.tag_buffer);

        buf
    }
}

impl ResponseBytes for ApiVersionsVer {
    fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(7);
        buf.put_u16(self.api_key);
        buf.put_u16(*self.version_range.start());
        buf.put_u16(*self.version_range.end());
        buf.put_u8(self.tag_buffer);

        buf
    }
}

impl ResponseBody<ApiVersionsRequest> for ApiVersionsBody {
    fn from_request(req_header: &RequestHeader, req_body: &ApiVersionsRequest) -> Self {
        let mut versions = Vec::new();
        let error_code = match req_header.check_version() {
            Ok(_) => 0,
            Err(kafka_error) => kafka_error.into_error_code(),
        };

        if error_code == 0 {
            for api in APIS {
                versions.push(ApiVersionsVer::from(api));
            }
        }

        Self {
            error_code,
            versions,
            throttle_time: 0,
            tag_buffer: req_body.tag_buffer.0,
        }
    }
}

impl From<&ApiInfo> for ApiVersionsVer {
    fn from(info: &ApiInfo) -> Self {
        Self {
            api_key: info.key,
            version_range: info.version_range.clone(),
            tag_buffer: 0,
        }
    }
}
