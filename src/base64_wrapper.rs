use std::fmt;

#[derive(Debug)]
pub struct Base64Error {
    details: String
}

impl Base64Error {
    fn new(msg: &str) -> Base64Error {
        Base64Error{details: msg.to_string()}
    }
}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<base64::DecodeError> for Base64Error {
    fn from(err: base64::DecodeError) -> Base64Error {
        Base64Error::new(&err.to_string())
    }
}

pub fn encode_base64(data: &[u8]) -> String {
    base64::encode(data)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, Base64Error> {
    base64::decode(encoded).map_err(Base64Error::from)
}