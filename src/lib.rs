#![deny(clippy::all)]

mod checker;
mod converters;
mod decode;
mod encode;
mod get_file;

pub use checker::*;
pub use converters::*;
pub use decode::*;
pub use encode::*;
pub use get_file::*;

#[macro_use]
extern crate napi_derive;

use napi::{Error as NapiError, Result};

/// Exposes the `encode_rs` function to Node.js via N-API.
#[napi]
pub async fn encode(image: String, message: String, step: String, output: String) -> Result<()> {
  encode_rs(&image, &message, &step, &output)
    .await
    .map_err(|e| NapiError::from_reason(e.to_string()))
}

/// Exposes the `decode_rs` function to Node.js via N-API.
#[napi]
pub async fn decode(image: String) -> Result<()> {
  decode_rs(&image)
    .await
    .map_err(|e| NapiError::from_reason(e.to_string()))
}
