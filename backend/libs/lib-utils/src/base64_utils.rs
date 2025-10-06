use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encode(data: &[u8]) -> String {
    STANDARD.encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(data)
}

pub fn encode_string(data: &str) -> String {
    encode(data.as_bytes())
}

pub fn decode_string(data: &str) -> Result<String, String> {
    decode(data)
        .map_err(|e| e.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
}
