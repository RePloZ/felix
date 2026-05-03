use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::api::requests::{IntoResponse, Request};
use crate::api::responses::ResponseBytes;
use crate::protocol::reader::{Bytes32, StreamReader};

pub async fn connect_stream(mut stream: TcpStream) -> anyhow::Result<()> {
    loop {
        match Bytes32::from_stream(&mut stream).await {
            Err(_) => break,
            Ok(_) => {
                let req = Request::from_stream(&mut stream).await?;
                let mut buf = req.into_response().to_bytes();
                println!("[Response] -> bytes: {buf:x}");
                stream.write_all_buf(&mut buf).await?;
            }
        }
    }
    Ok(())
}
