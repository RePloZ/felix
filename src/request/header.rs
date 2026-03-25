pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<Vec<u8>>,
    pub tag_buffer: Option<Vec<u8>>,
}

impl From<&[u8; 1024]> for RequestHeader {
    fn from(data: &[u8; 1024]) -> Self {
        let client_id = data[8..].to_vec();

        let tag_buffer = None;
        Self {
            api_key: i16::from_be_bytes([data[0], data[1]]),
            api_version: i16::from_be_bytes([data[2], data[3]]),
            correlation_id: i32::from_be_bytes([data[8], data[9], data[10], data[11]]),
            client_id: Some(client_id),
            tag_buffer: tag_buffer,
        }
    }
}
