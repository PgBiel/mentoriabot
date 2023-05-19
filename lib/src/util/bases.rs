use base64::{engine::general_purpose::STANDARD, Engine};

pub fn base64_encode_bytes(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}
