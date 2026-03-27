use crate::{stream::request::RequestApiVersion, utils::ResponseApiError};
use bytes::{BufMut, BytesMut};

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

impl From<RequestApiVersion> for ResponseApiVersion {
    fn from(req: RequestApiVersion) -> Self {
        let error_code: u16 = match Self::check(&req) {
            Ok(_) => 0,
            Err(err) => err.into(),
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
        let mut size = 4; // correlation_id
        size += 2; // error_code
        size += 1; // api_keys array length (compact array, 1 element = 0x02 which is 1 byte)
        for _version in &self.body.versions {
            size += 2; // api_key
            size += 2; // min_version
            size += 2; // max_version
            size += 1; // tag_buffer
        }
        size += 4; // throttle_time_ms
        size += 1; // tag_buffer

        size
    }

    fn check(req: &RequestApiVersion) -> Result<(), ResponseApiError> {
        Self::check_api_key(&req)?;
        Self::check_version(&req)?;
        Ok(())
    }

    fn check_api_key(req: &RequestApiVersion) -> Result<(), ResponseApiError> {
        if req.header.key == 18 {
            return Ok(());
        }
        Err(ResponseApiError::UnsupportedKey(req.header.key))
    }

    fn check_version(req: &RequestApiVersion) -> Result<(), ResponseApiError> {
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
