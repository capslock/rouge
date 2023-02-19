mod sys;
pub use sys::*;
mod compress;
#[cfg(target_arch = "wasm32")]
mod encode;
mod serialize;

#[cfg(target_arch = "wasm32")]
use base64::DecodeError;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum SaveloadError {
    #[error("Failed to serialize save data")]
    Serialize(#[source] anyhow::Error),

    #[error("Failed to deserialize save data")]
    Deserialize(#[source] anyhow::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("Failed to decode save data")]
    Decode(#[from] DecodeError),

    #[error("IO error occurred")]
    IO(#[from] std::io::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("JS error occurred: {message:?}")]
    JS { message: JsValue },
}
