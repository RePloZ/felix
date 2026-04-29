use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::requests;
use crate::responses::ApiVersionsRes;

pub async fn connect_stream(mut socket: TcpStream) -> anyhow::Result<()> {
    loop {
        let req = match requests::ApiVersionsReq::from_socket(&mut socket).await {
            Ok(req) => req,
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        };

        let res: ApiVersionsRes = req.into();
        let mut buf: BytesMut = res.into();
        socket.write_all_buf(&mut buf).await?;
    }

    Ok(())
}
