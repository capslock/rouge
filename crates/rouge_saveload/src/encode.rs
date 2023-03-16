use base64::{engine::general_purpose, Engine as _};

use crate::SaveloadError as Error;

/// Base64-encode a byte slice. Useful for storing a binary save file as text.
pub fn encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(bytes)
}

/// Base64-decode a byte slice. Useful when storing a binary save file as text.
pub fn decode(string: &str) -> Result<Vec<u8>, Error> {
    general_purpose::STANDARD_NO_PAD
        .decode(string)
        .map_err(Error::from)
}
