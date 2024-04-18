pub fn encode_base64(data: &[u8]) -> String {
    base64::encode(data)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(encoded)
}
