pub mod api;
pub mod error;
pub mod protocol;
pub mod stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::net::TcpListener;
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    loop {
        let socket = listener.accept().await?.0;

        tokio::spawn(async move {
            if let Err(error) = stream::connect_stream(socket).await {
                eprintln!("connection error: {:?}", error);
            }
        });
    }
}
