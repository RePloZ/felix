use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use crate::stream::{parse_request, response::ResponseApiVersion};

pub mod stream;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                tokio::spawn(async move {
                    let req = parse_request(&mut stream).await;
                    let mut buf: BytesMut = ResponseApiVersion::from(req).into();

                    stream.write_all_buf(&mut buf).await.unwrap()
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
