#![deny(clippy::all)]

use colored::*;
use napi::{Env, JsObject, Result};
use napi_derive::napi;

mod checker;
mod converters;
mod decode;
mod encode;
mod get_file;

#[napi]
pub fn encode(
  env: Env,
  image: String,
  message: String,
  step: Option<String>,
  output: Option<String>,
) -> Result<JsObject> {
  let step = step.unwrap_or_else(|| "1x1".to_string());
  let output = output.unwrap_or_else(|| "out.png".to_string());

  env.execute_tokio_future(
    async move {
      match encode::encode(&image, &message, &step, &output).await {
        Ok(_) => {
          println!("{} has been encoded", output.green());
          println!("message: {}", message.yellow());
          println!("pattern: {}", step);
          Ok(())
        }
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
      }
    },
    |env, _| env.get_undefined(),
  )
}

#[napi]
pub fn decode(env: Env, image: String) -> Result<JsObject> {
  env.execute_tokio_future(
    async move {
      match decode::decode(&image).await {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
      }
    },
    |env, _| env.create_string("Decoding completed"),
  )
}
