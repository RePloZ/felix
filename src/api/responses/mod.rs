//use body::describe_topics_partitions::;

mod body;
mod header;
mod traits;

use bytes::{BufMut, BytesMut};

pub use self::body::{
    api_versions::ApiVersionsBody, describe_topics_partitions::DescribeTopicBody,
};
pub use self::header::{ResHeaderV0, ResHeaderV1};
pub use self::traits::{ResponseBody, ResponseBytes, ResponseHeader};

pub enum Response {
    DescribeTopic(ResHeaderV1, DescribeTopicBody),
    ApiVersions(ResHeaderV0, ApiVersionsBody),
}

impl ResponseBytes for Response {
    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        let (header, body) = match self {
            Response::DescribeTopic(res_header, res_body) => {
                (res_header.to_bytes(), res_body.to_bytes())
            }
            Response::ApiVersions(res_header, res_body) => {
                (res_header.to_bytes(), res_body.to_bytes())
            }
        };
        bytes.put_u32(header.len() as u32 + body.len() as u32);
        bytes.extend(header.iter());
        bytes.extend(body.iter());
        bytes
    }
}
