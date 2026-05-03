use crate::error::Result;
use tokio::io::AsyncReadExt;

pub trait StreamReader: Sized {
    fn from_stream<R: AsyncReadExt + Unpin>(
        stream: &mut R,
    ) -> impl std::future::Future<Output = Result<Self>>;
}

#[derive(Debug)]
pub struct Bytes8(pub u8);

#[derive(Debug)]
pub struct Bytes16(pub u16);

#[derive(Debug)]
pub struct Bytes32(pub u32);

#[derive(Debug)]
pub struct Bytes64(pub u64);

#[derive(Debug)]
pub struct Bytes128(pub u128);

#[derive(Debug)]
pub struct BytesUUID(pub [u8; 16]);

#[derive(Debug)]
pub struct BytesString(pub u16, pub String);

#[derive(Debug)]
pub struct BytesCompactString(pub u8, pub String);

#[derive(Debug)]
pub struct BytesCompactArray<T>(pub Bytes8, pub Vec<T>);

impl StreamReader for Bytes8 {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        Ok(Self(stream.read_u8().await?))
    }
}

impl StreamReader for Bytes16 {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        Ok(Self(stream.read_u16().await?))
    }
}

impl StreamReader for Bytes32 {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        Ok(Self(stream.read_u32().await?))
    }
}

impl StreamReader for Bytes64 {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        Ok(Self(stream.read_u64().await?))
    }
}

impl StreamReader for Bytes128 {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        Ok(Self(stream.read_u128().await?))
    }
}

impl StreamReader for BytesUUID {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let mut buf = [0u8; 16];
        stream.read_exact(&mut buf).await?;
        Ok(Self(buf))
    }
}

impl StreamReader for BytesString {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let txt_len = Bytes16::from_stream(stream).await?.0;
        let mut buf: Vec<u8> = vec![0u8; txt_len as usize];
        stream.read_exact(&mut buf).await?;
        let txt = String::from_utf8(buf)?;
        Ok(Self(txt_len, txt))
    }
}

impl StreamReader for BytesCompactString {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let txt_len = Bytes8::from_stream(stream).await?.0;
        let mut buf: Vec<u8> = vec![0u8; (txt_len as usize) - 1];
        stream.read_exact(&mut buf).await?;
        let txt = String::from_utf8(buf)?;
        Ok(Self(txt_len, txt))
    }
}

impl<T: StreamReader + 'static> StreamReader for BytesCompactArray<T> {
    async fn from_stream<R: AsyncReadExt + Unpin>(stream: &mut R) -> Result<Self> {
        let array_len = Bytes8::from_stream(stream).await?;
        let mut arr = Vec::new();
        for _ in 0..(array_len.0 - 1) {
            let elem = T::from_stream(stream).await?;
            arr.push(elem);
        }
        Ok(Self(array_len, arr))
    }
}

/*
impl<T: StreamReader> StreamReader for BytesCompactArray<T> {
    async fn from_stream<'a, R: AsyncReadExt + Unpin + 'a>(stream: &'a mut R) -> Result<Self>
    where
        T: 'a,
    {

    }
}
    */
