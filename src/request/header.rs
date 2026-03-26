use thiserror::Error;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub msg_size: i32,
    pub client_id: Option<Vec<u8>>,
    pub tag_buffer: Option<Vec<u8>>,
}

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("the data for key `{0}` is not available")]
    ReadElement(#[from] tokio::io::Error),
}

impl From<(i32, i16, i16, i32)> for RequestHeader {
    fn from(header: (i32, i16, i16, i32)) -> Self {
        let (msg_size, api_key, api_version, correlation_id) = header;
        Self {
            msg_size,
            api_key,
            api_version,
            correlation_id,
            client_id: None,
            tag_buffer: None,
        }
    }
}

impl RequestHeader {
    pub async fn parse_request(stream: &mut TcpStream) -> Self {
        let msg_size = stream.read_i32().await.unwrap();
        let api_key = stream.read_i16().await.unwrap();
        let api_ver = stream.read_i16().await.unwrap();
        let correlation_id = stream.read_i32().await.unwrap();

        Self::from((msg_size, api_key, api_ver, correlation_id))
    }

    pub fn check_version(&self) -> i16 {
        match self.api_version {
            0..=4 => 0,
            _ => 35,
        }
    }
}
