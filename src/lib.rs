#[deny(clippy::manual_pattern_char_comparison)]
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

use napi::{Error as NapiError, Error};
use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
#[derive(Serialize)]
pub struct EncodeResult {
  pub output: String,
  pub message: String,
  pub pattern: String,
}

#[napi(object)]
#[derive(Serialize)]
pub struct DecodeResult {
  pub message: String,
  pub pattern: String,
}

/// Exposes the `encode_rs` function to Node.js via N-API.
#[napi]
pub async fn encode(
  image: String,
  message: String,
  step: String,
  output: String,
) -> std::result::Result<EncodeResult, Error> {
  encode_rs(&image, &message, &step, &output)
    .await
    .map_err(|e| NapiError::from_reason(e.to_string()))
}

/// Exposes the `decode_rs` function to Node.js via N-API.
#[napi]
pub async fn decode(image: String) -> std::result::Result<DecodeResult, Error> {
  decode_rs(&image)
    .await
    .map_err(|e| NapiError::from_reason(e.to_string()))
}
