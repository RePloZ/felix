use std::io::{Read, Write};
use std::net::TcpListener;

use crate::request::header::RequestHeader;

pub mod request;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer);

                let req_header = RequestHeader::from(&buffer);

                let _ = stream.write_all(&(0i32).to_be_bytes());
                let _ = stream.write(&req_header.correlation_id.to_be_bytes());
                let _ = stream.flush();

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
