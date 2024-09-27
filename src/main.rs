use clap::{Arg, Command};
use colored::*;

mod decode;
mod encode;

pub fn main(args: Vec<String>) -> Result<(), anyhow::__private> {
  let matches = Command::new("stegjs")
      .version(env!("CARGO_PKG_VERSION"))
      .about("Encrypt your message to PNG image.")
      .arg(
        Arg::new("image")
            .help("Input image file or URL")
            .required(true)
            .index(1),
      )
      .subcommand(
        Command::new("encode")
            .about("Encode a message into an image")
            .arg(Arg::new("message").help("Message to encode").required(true))
            .arg(
              Arg::new("step")
                  .help("Step pattern (e.g., '1x1')")
                  .default_value("1x1"),
            )
            .arg(
              Arg::new("output")
                  .help("Output image file")
                  .default_value("out.png"),
            ),
      )
      .subcommand(Command::new("decode").about("Decode a message from an image"))
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
      }
      Err(e) => eprintln!("{}", format!("Encoding error: {}", e).red()),
    }
  } else if matches.subcommand_matches("decode").is_some() {
    match decode::decode(image) {
      Ok(_) => {} // The decode function already prints the results
      Err(e) => eprintln!("{}", format!("Decoding error: {}", e).red()),
    }
  } else {
    eprintln!("{}", "No mode specified. Use 'encode' or 'decode'.".red());
  }

  Ok(())
}
