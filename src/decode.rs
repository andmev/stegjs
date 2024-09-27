use crate::checker::{is_png, is_uri};
use crate::converters::{bits_to_string, meta_to_obj};
use crate::get_file::{by_path, by_uri};
use anyhow::{anyhow, Result};
use image::GenericImageView;

/// Function to decode a message from an image
pub async fn decode(img: &str) -> Result<()> {
  if !is_png(img) {
    return Err(anyhow!("Only *.png images are supported."));
  }

  let img_path = if is_uri(img) {
    by_uri(img).await?
  } else {
    by_path(img).await?
  };

  decode_image(&img_path)
}

fn decode_image(img_path: &str) -> Result<()> {
  let img = image::open(img_path)?;
  let (width, height) = img.dimensions();

  let mut secret_text = Vec::new();
  let mut index = 0;

  // First get meta-information
  for y in 0..height {
    for x in 0..width {
      let pixel = img.get_pixel(x, y);
      secret_text.push(pixel[0] & 1 == 1);
      index += 1;

      if index == 32 {
        break;
      }
    }
    if index == 32 {
      break;
    }
  }

  let meta_string = bits_to_string(&secret_text)?;
  println!("Meta-information: {}", meta_string);

  // Add more detailed error handling
  match meta_to_obj(&meta_string) {
      Ok((string_length, width_step, height_step)) => {
          println!("Parsed meta-information: length={}, width_step={}, height_step={}", 
                   string_length, width_step, height_step);
          
          // Continue with the rest of the function
          let string_length: usize = string_length.parse()?;
          let width_step: u32 = width_step.parse()?;
          let height_step: u32 = height_step.parse()?;

          secret_text.clear();

          // Get message from the image
          'outer: for y in (0..height).step_by(height_step as usize) {
            for x in (0..width).step_by(width_step as usize) {
              let pixel = img.get_pixel(x, y);
              secret_text.push(pixel[3] & 1 == 1);

              if secret_text.len() == string_length * 8 {
                break 'outer;
              }
            }
          }

          let message = bits_to_string(&secret_text)?;
          let message = message.chars().take(string_length).collect::<String>();

          println!("{} was decoded!", img_path);
          println!("message: {}", message);
          println!("pattern: {}x{}", width_step, height_step);

          Ok(())
      },
      Err(e) => {
          println!("Failed to parse meta-information: {:?}", e);
          Err(e)
      }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::encode::encode;
  use image::{Rgba, RgbaImage};
  use tempfile::NamedTempFile;

  #[tokio::test]
  async fn test_decode() {
    let temp_in = NamedTempFile::new().unwrap();
    let temp_out = NamedTempFile::new().unwrap();

    // Create a test image
    let img = RgbaImage::from_fn(100, 100, |_, _| Rgba([255, 255, 255, 255]));
    img.save(temp_in.path().with_extension("png"))
        .expect("Failed to save input image");

    let test_message = "Test message";

    // Encode the message
    encode(
      temp_in.path().with_extension("png").to_str().unwrap(),
      test_message,
      "1x1",
      temp_out.path().with_extension("png").to_str().unwrap(),
    )
    .await
    .expect("Failed to encode message");

    // Print the contents of the encoded image
    let encoded_img = image::open(temp_out.path().with_extension("png")).unwrap();
    println!("Encoded image dimensions: {:?}", encoded_img.dimensions());
    
    // Decode the message
    let result = decode(temp_out.path().with_extension("png").to_str().unwrap()).await;

    match result {
      Ok(_) => println!("Decoding successful"),
      Err(e) => {
        println!("Decoding failed: {:?}", e);
        panic!("Decoding failed: {:?}", e);
      }
    }
  }
}
