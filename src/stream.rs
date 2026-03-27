mod req;
mod res;

use anyhow::Result;
use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::utils::CodecError;
use req::KafkaRequest;

pub async fn handle_connection(mut socket: TcpStream) -> Result<(), CodecError> {
    use crate::stream::res::ResponseApiVersion;

    loop {
        let req = match KafkaRequest::from_socket(&mut socket).await {
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
