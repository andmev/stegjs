#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
fn main() -> Result {
  let args: Vec<String> = std::env::args().collect();
  crate::run(args).await
}
