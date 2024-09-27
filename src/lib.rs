#![deny(clippy::all)]

use napi_derive::napi;
use napi::{Result, Env, JsString, JsUnknown};
use clap::{Command, Arg};
use colored::*;

mod encode;
mod decode;
mod checker;
mod converters;
mod get_file;

#[napi]
pub async fn run(args: Vec<String>) -> Result<()> {
    let matches = Command::new("stegjs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Encrypt your message to PNG image.")
        .arg(Arg::new("image")
            .help("Input image file or URL")
            .required(true)
            .index(1))
        .subcommand(Command::new("encode")
            .about("Encode a message into an image")
            .arg(Arg::new("message")
                .help("Message to encode")
                .required(true))
            .arg(Arg::new("step")
                .help("Step pattern (e.g., '1x1')")
                .default_value("1x1"))
            .arg(Arg::new("output")
                .help("Output image file")
                .default_value("out.png")))
        .subcommand(Command::new("decode")
            .about("Decode a message from an image"))
        .get_matches_from(args);

    let image = matches.get_one::<String>("image").unwrap();

    if let Some(encode_matches) = matches.subcommand_matches("encode") {
        let message = encode_matches.get_one::<String>("message").unwrap();
        let step = encode_matches.get_one::<String>("step").unwrap();
        let output = encode_matches.get_one::<String>("output").unwrap();

        match encode::encode(image, message, step, output).await {
            Ok(_) => {
                println!("{} has been encoded", output.green());
                println!("message: {}", message.yellow());
                println!("pattern: {}", step);
            },
            Err(e) => eprintln!("{}", format!("Encoding error: {}", e).red()),
        }
    } else if matches.subcommand_matches("decode").is_some() {
        match decode::decode(image).await {
            Ok(_) => {},  // The decode function already prints the results
            Err(e) => eprintln!("{}", format!("Decoding error: {}", e).red()),
        }
    } else {
        eprintln!("{}", "No mode specified. Use 'encode' or 'decode'.".red());
    }

    Ok(())
}

#[napi]
pub async fn encode(env: Env, image: String, message: String, step: Option<String>, output: Option<String>) -> Result<JsUnknown> {
    let step = step.unwrap_or_else(|| "1x1".to_string());
    let output = output.unwrap_or_else(|| "out.png".to_string());

    match encode::encode(&image, &message, &step, &output).await {
        Ok(_) => {
            println!("{} has been encoded", output.green());
            println!("message: {}", message.yellow());
            println!("pattern: {}", step);
            Ok(env.get_undefined()?.into_unknown())
        },
        Err(e) => Err(napi::Error::from_reason(format!("Encoding error: {}", e))),
    }
}

#[napi]
pub async fn decode(env: Env, image: String) -> Result<JsString> {
    match decode::decode(&image).await {
        Ok(message) => Ok(env.create_string(&message)?),
        Err(e) => Err(napi::Error::from_reason(format!("Decoding error: {}", e))),
    }
}