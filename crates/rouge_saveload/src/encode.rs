use base64::{engine::general_purpose, Engine as _};

pub fn encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(bytes)
}

pub fn decode(string: &str) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD.decode(string).unwrap()
}
