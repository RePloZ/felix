use bytes::{BufMut, BytesMut};
use std::ops::RangeInclusive;

use crate::requests::ApiVersionsReq;
use crate::utils::versions::{API_CAPABILITIES, ApiReqVersion};

#[derive(Debug)]
pub struct ApiVersionsRes {
    pub size: u32,
    pub header: ApiResHeader,
    pub body: ApiResBody,
}

#[derive(Debug)]
pub struct ApiResHeader {
    pub correlation_id: u32,
}

#[derive(Debug)]
pub struct ApiResBody {
    pub error_code: u16,
    pub versions: Vec<ApiVersion>,
    pub throttle_time: u32,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiVersion {
    pub api_key: u16,
    pub version_range: RangeInclusive<u16>,
    pub tag_buffer: u8,
}

impl Into<ApiVersionsRes> for ApiVersionsReq {
    fn into(self) -> ApiVersionsRes {
        let mut versions = Vec::new();

        let error_code = match ApiReqVersion::new(self.header.key, self.header.version).check() {
            Ok(_) => 0,
            Err(api_error) => api_error.into_error_code(),
        };

        if error_code == 0 {
            match self.header.key {
                18 => {
                    let api_capabilities = API_CAPABILITIES.lock().unwrap();
                    let capabilities = api_capabilities.iter();
                    for (api_key, (_, version_range)) in capabilities {
                        let version = ApiVersion {
                            api_key: api_key.to_owned(),
                            version_range: version_range.to_owned(),
                            tag_buffer: 0,
                        };
                        versions.push(version);
                    }
                }
                _ => {}
            };
        };

        let header = ApiResHeader {
            correlation_id: self.header.correlation_id,
        };

        let versions_len: u32 = versions.len().try_into().unwrap();
        let body = ApiResBody {
            error_code,
            versions: versions,
            throttle_time: 0,
            tag_buffer: self.header.tag_buffer,
        };

        ApiVersionsRes {
            size: 12 + versions_len * 7,
            header,
            body,
        }
    }
}

impl Into<BytesMut> for ApiVersionsRes {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(8);

        buf.put_u32(self.size);
        buf.put_u32(self.header.correlation_id);
        buf.put_u16(self.body.error_code);
        buf.put_u8(self.body.versions.len() as u8 + 1);
        for version in self.body.versions {
            buf.put_u16(version.api_key);
            buf.put_u16(*version.version_range.start());
            buf.put_u16(*version.version_range.end());
            buf.put_u8(version.tag_buffer);
        }
        buf.put_u32(self.body.throttle_time);
        buf.put_u8(0); //

        buf
    }
}
