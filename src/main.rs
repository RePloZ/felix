use tokio::net::TcpListener;

use crate::stream::connect_stream;

pub mod requests;
pub mod responses;
pub mod stream;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    loop {
        let socket = listener.accept().await?.0;

        tokio::spawn(async move {
            if let Err(error) = connect_stream(socket).await {
                eprintln!("connection error: {:?}", error);
            }
        });
    }
}
