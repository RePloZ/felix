use super::head::ApiReqHeader;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ApiVersionsReq {
    pub size: u32,
    pub header: ApiReqHeader,
    pub content: ApiReqBody,
}

#[derive(Debug)]
pub struct ApiReqBody {
    pub client: ApiReqContent,
    pub version: ApiReqContent,
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiReqContent(pub u8, pub String);

impl ApiVersionsReq {
    pub async fn from_socket(socket: &mut TcpStream) -> anyhow::Result<Self> {
        let size = socket.read_u32().await?;
        println!("[Debug] - API Versions Request: Size {size}");
        let header = ApiReqHeader::from_socket(socket).await?;
        println!("[Debug] - API Versions Request: Header {header:?}");
        let content = ApiReqBody::from_socket(socket).await?;
        println!("[Debug] - API Versions Request: Body {content:?}");

        Ok(Self {
            size,
            header,
            content,
        })
    }
}

impl ApiReqBody {
    pub async fn from_socket(socket: &mut TcpStream) -> anyhow::Result<Self> {
        let client = ApiReqContent::from_socket(socket).await?;
        let version = ApiReqContent::from_socket(socket).await?;
        let tag_buffer = socket.read_u8().await?;
        println!("Tag Buffer : {tag_buffer}");
        Ok(Self {
            client,
            version,
            tag_buffer,
        })
    }
}

impl ApiReqContent {
    pub async fn from_socket(socket: &mut TcpStream) -> anyhow::Result<Self> {
        let length = socket.read_u8().await?;
        let mut content: Vec<u8> = vec![0; (length as usize) - 1];
        socket.read_exact(&mut content).await?;
        let text = String::from_utf8(content)?;

        Ok(Self(length, text))
    }
}
