use napi::bindgen_prelude::*;
use napi_derive::napi;
use stegjs::run;

#[napi]
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    run(args).await?;
    Ok(())
}