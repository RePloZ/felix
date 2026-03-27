pub mod request;
pub mod response;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::stream::request::{ApiVersion, ApiVersionClientId, ClientId, ClientSoftwareVersion, RequestApiVersion, RequestHeader};

pub async fn xparse_request(stream: &mut TcpStream) -> request::Header {
    let msg_size = stream.read_u32().await.unwrap();
    let api_key = stream.read_i16().await.unwrap();
    let api_version = stream.read_i16().await.unwrap();
    let correlation_id = stream.read_i32().await.unwrap();

    request::Header {
        api_key,
        msg_size,
        api_version,
        correlation_id,
        client_id: None,
        tag_buffer: None,
    }
}

pub async fn parse_request(stream: &mut TcpStream) -> RequestApiVersion {
    let msg_size = stream.read_u32().await.unwrap();
    let api_key = stream.read_u16().await.unwrap();
    let api_version = stream.read_u16().await.unwrap();
    let correlation_id = stream.read_u32().await.unwrap();
    let header_client = ClientId::read_client(stream).await;

    let tag_buffer = stream.read_u8().await.unwrap();

    let api_version_client = ApiVersionClientId::read_client(stream).await;
    let software_version = ClientSoftwareVersion::read_client(stream).await;

    let api_tag_buffer = stream.read_u8().await.unwrap();

    RequestApiVersion {
        msg_size,
        header: RequestHeader {
            key: api_key,
            version: api_version,
            correlation_id,
            client: header_client,
            tag_buffer: tag_buffer,
        },
        api_version: ApiVersion {
            client: api_version_client,
            software_version: software_version,
            tag_buffer: api_tag_buffer,
        },
    }
}
