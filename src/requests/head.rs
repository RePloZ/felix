use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ApiReqHeader {
    pub key: u16,
    pub version: u16,
    pub correlation_id: u32,
    pub client: ApiReqClient,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiReqClient(pub u16, pub String);

impl ApiReqHeader {
    pub async fn from_socket(socket: &mut TcpStream) -> anyhow::Result<Self> {
        let key = socket.read_u16().await?;
        let version = socket.read_u16().await?;
        let correlation_id = socket.read_u32().await?;
        let client = ApiReqClient::from_socket(socket).await?;
        let tag_buffer = socket.read_u8().await?;

        Ok(Self {
            key,
            version,
            correlation_id,
            client,
            tag_buffer,
        })
    }
}

impl ApiReqClient {
    pub async fn from_socket(socket: &mut TcpStream) -> anyhow::Result<Self> {
        let length = socket.read_u16().await?;
        let mut content: Vec<u8> = vec![0; length as usize];
        socket.read_exact(&mut content).await?;
        let text = String::from_utf8(content)?;

        Ok(Self(length, text))
    }
}
