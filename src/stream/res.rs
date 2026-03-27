use bytes::{BufMut, BytesMut};

use crate::stream::req::KafkaRequest;
use crate::utils::ResponseApiError;

pub struct ResponseApiVersion {
    pub header: ResponseHeader,
    pub body: ResponseBody,
}

pub struct ResponseHeader {
    correlation_id: u32,
}

pub struct ResponseBody {
    error_code: u16,
    versions: Vec<ResponseVersion>,
    throttle_time_ms: u32,
    tag_buffer: u8,
}

pub struct ResponseVersion {
    api_key: u16,
    version_range: (u16, u16),
    tag_buffer: u8,
}

impl From<KafkaRequest> for ResponseApiVersion {
    fn from(req: KafkaRequest) -> Self {
        let error_code: u16 = match Self::check(&req) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{err}");
                err.into()
            }
        };

        let mut versions = Vec::new();

        versions.push(ResponseVersion {
            api_key: req.header.key,
            version_range: (0, 4),
            tag_buffer: req.header.tag_buffer,
        });

        Self {
            header: ResponseHeader {
                correlation_id: req.header.correlation_id,
            },
            body: ResponseBody {
                error_code,
                versions: versions,
                throttle_time_ms: 0,
                tag_buffer: req.header.tag_buffer,
            },
        }
    }
}

impl ResponseApiVersion {
    pub fn size(&self) -> u32 {
        12 + (self.body.versions.len() as u32) * 7
    }

    fn check(req: &KafkaRequest) -> Result<(), ResponseApiError> {
        Self::check_api_key(&req)?;
        Self::check_version(&req)?;
        Ok(())
    }

    fn check_api_key(req: &KafkaRequest) -> Result<(), ResponseApiError> {
        if req.header.key == 18 {
            return Ok(());
        }
        Err(ResponseApiError::UnsupportedKey(req.header.key))
    }

    fn check_version(req: &KafkaRequest) -> Result<(), ResponseApiError> {
        if (0..=4).contains(&req.header.version) {
            return Ok(());
        }
        Err(ResponseApiError::UnsupportedVersion(req.header.version))
    }
}

impl Into<BytesMut> for ResponseApiVersion {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(8);

        buf.put_u32(self.size()); //4
        buf.put_u32(self.header.correlation_id); //8
        buf.put_u16(self.body.error_code); //10
        buf.put_u8(self.body.versions.len() as u8 + 1);
        for version in self.body.versions {
            buf.put_u16(version.api_key);
            buf.put_u16(version.version_range.0);
            buf.put_u16(version.version_range.1);
            buf.put_u8(version.tag_buffer);
        }
        buf.put_u32(self.body.throttle_time_ms);
        buf.put_u8(self.body.tag_buffer);

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_response(versions: Vec<ResponseVersion>) -> ResponseApiVersion {
        ResponseApiVersion {
            header: ResponseHeader { correlation_id: 1 },
            body: ResponseBody {
                error_code: 0,
                versions,
                throttle_time_ms: 0,
                tag_buffer: 0,
            },
        }
    }

    fn make_version(api_key: u16) -> ResponseVersion {
        ResponseVersion {
            api_key,
            version_range: (0, 4),
            tag_buffer: 0,
        }
    }

    #[test]
    fn size_with_one_version() {
        let resp = make_response(vec![make_version(18)]);
        // 4 correlation_id + 2 error_code + 1 array_len + 7 version + 4 throttle + 1 tag = 19
        assert_eq!(resp.size(), 19);
    }

    #[test]
    fn size_with_no_versions() {
        let resp = make_response(vec![]);
        // 4 + 2 + 1 + 0 + 4 + 1 = 12
        assert_eq!(resp.size(), 12);
    }

    #[test]
    fn size_with_multiple_versions() {
        let resp = make_response(vec![make_version(1), make_version(18), make_version(75)]);
        // 12 + 3 * 7 = 33
        assert_eq!(resp.size(), 33);
    }

    #[test]
    fn size_matches_serialized_bytes() {
        let resp = make_response(vec![make_version(18)]);
        let expected_size = resp.size();
        let buf: BytesMut = resp.into();
        // buf includes the 4-byte size prefix itself, so actual payload = buf.len() - 4
        assert_eq!((buf.len() - 4) as u32, expected_size);
    }
}
