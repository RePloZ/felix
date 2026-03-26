use tokio::{io::AsyncWriteExt, net::TcpListener};

use bytes::{BufMut, BytesMut};

use crate::request::header::RequestHeader;

pub mod request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                tokio::spawn(async move {
                    let mut buf = BytesMut::with_capacity(8);

                    let req_header = RequestHeader::parse_request(&mut stream).await;
                    buf.put_i32(0);
                    buf.put_i32(req_header.correlation_id);
                    buf.put_i16(req_header.check_version());

                    stream.write_all_buf(&mut buf).await.unwrap()
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
