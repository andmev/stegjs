use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    crate::run(args).await
}