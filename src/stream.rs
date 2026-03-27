pub mod request;
pub mod response;

use anyhow::Result;
use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::stream::request::RequestApiVersion;
use crate::utils::CodecError;

pub async fn handle_connection(mut socket: TcpStream) -> Result<(), CodecError> {
    use crate::stream::response::ResponseApiVersion;

    loop {
        let req = match RequestApiVersion::from_socket(&mut socket).await {
            Ok(req) => req,
            Err(_) => break,
        };

        let mut buf: BytesMut = ResponseApiVersion::from(req).into();

        socket
            .write_all_buf(&mut buf)
            .await
            .map_err(|err| CodecError::from(err))?;
    }

    Ok(())
}
