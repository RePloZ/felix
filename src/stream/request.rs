use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::utils::CodecError;

#[derive(Debug)]
pub struct RequestApiVersion {
    pub msg_size: u32,
    pub header: RequestHeader,
    pub api_version: ApiVersion,
}

#[derive(Debug)]
pub struct RequestHeader {
    pub key: u16,
    pub version: u16,
    pub correlation_id: u32,
    pub client: ClientId,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ClientId {
    pub length: u16,
    pub content: String,
}

#[derive(Debug)]
pub struct ClientSoftwareVersion {
    pub length: u8,
    pub content: String,
}

#[derive(Debug)]
pub struct ApiVersion {
    pub client: ApiVersionClientId,
    pub software_version: ClientSoftwareVersion,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiVersionClientId {
    pub length: u8,
    pub content: String,
}

pub struct Header {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub msg_size: u32,
    pub client_id: Option<Vec<u8>>,
    pub tag_buffer: Option<Vec<u8>>,
}

impl Header {
    pub fn check_version(&self) -> i16 {
        match self.api_version {
            0..=4 => 0,
            _ => 35,
        }
    }
}

impl ApiVersionClientId {
    pub async fn read_client(stream: &mut TcpStream) -> Result<Self, CodecError> {
        let length = stream.read_u8().await?;
        let mut content: Vec<u8> = vec![0; (length as usize) - 1];
        stream.read_exact(&mut content).await?;
        let content = String::from_utf8(content).map_err(|_| CodecError::UnexpectedEof)?;

        Ok(Self { length, content })
    }
}

impl ClientId {
    pub async fn read_client(stream: &mut TcpStream) -> Result<Self, CodecError> {
        let length = stream.read_u16().await?;
        let mut content: Vec<u8> = vec![0; length.into()];
        stream.read_exact(&mut content).await?;
        let content = String::from_utf8(content).map_err(|_| CodecError::UnexpectedEof)?;

        Ok(Self { length, content })
    }
}

impl ClientSoftwareVersion {
    pub async fn read_client(stream: &mut TcpStream) -> Result<Self, CodecError> {
        let length = stream.read_u8().await?;
        let mut content = vec![0; (length as usize) - 1];
        stream.read_exact(&mut content).await?;
        Ok(Self {
            length,
            content: String::from_utf8(content).map_err(|_| CodecError::UnexpectedEof)?,
        })
    }
}

impl RequestApiVersion {
    pub async fn from_socket(socket: &mut TcpStream) -> Result<Self, CodecError> {
        let msg_size = socket.read_u32().await?;
        let api_key = socket.read_u16().await?;
        let api_version = socket.read_u16().await?;
        let correlation_id = socket.read_u32().await?;
        let header_client = ClientId::read_client(socket).await?;

        let tag_buffer = socket.read_u8().await?;

        let api_version_client = ApiVersionClientId::read_client(socket).await?;
        let software_version = ClientSoftwareVersion::read_client(socket).await?;

        let api_tag_buffer = socket.read_u8().await?;

        Ok(RequestApiVersion {
            msg_size,
            header: RequestHeader {
                key: api_key,
                version: api_version,
                correlation_id,
                client: header_client,
                tag_buffer,
            },
            api_version: ApiVersion {
                client: api_version_client,
                software_version,
                tag_buffer: api_tag_buffer,
            },
        })
    }
}
