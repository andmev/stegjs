use std::error::Error;

use crate::checker::{is_png, is_uri};
use crate::converters::{bits_to_string, meta_to_obj};
use crate::get_file::{by_path, by_uri};
use crate::DecodeResult; // Add this import

/// Decodes a hidden message from a PNG image using steganography.
///
/// # Parameters
/// - `img`: Path to the input PNG image or a URI.
///
/// # Errors
/// Returns an error if the input image is not a valid PNG, if decoding fails,
/// or if there are issues reading the files.
pub async fn decode_rs(img: &str) -> Result<DecodeResult, Box<dyn Error>> {
  // Check that the input file is in PNG format.
  if !is_png(img) {
    return Err("Only *.png images are supported.".into());
  }

  // Determine if the input is a URI or a local path and retrieve the file accordingly.
  let file_path = if is_uri(img) {
    by_uri(img).await?
  } else {
    by_path(img).await?
  };

  // Proceed to decode the message from the image.
  let (message, pattern) = decode_image(&file_path)?;

  Ok(DecodeResult { message, pattern })
}

/// Helper function to decode the message from the image.
///
/// # Parameters
/// - `file_path`: Path to the input PNG image.
///
/// # Errors
/// Returns an error if decoding fails or if the message cannot be reconstructed.
fn decode_image(file_path: &str) -> Result<(String, String), Box<dyn Error>> {
  // Load the image from the specified path.
  let img = image::open(file_path)?.to_rgba8();
  let (width, height) = img.dimensions();

  // Define the number of bits to read for meta-information.
  // Assuming meta string is 8 characters: "10|XX"
  let meta_str_length = 8; // Adjusted to match the meta format in encode.rs
  let meta_bits_len = meta_str_length * 8;

  // Extract meta bits
  let mut meta_bits: Vec<bool> = Vec::with_capacity(meta_bits_len);

  for y in 0..height {
    for x in 0..width {
      let pixel = img.get_pixel(x, y);
      let bit = (pixel[3] & 1) != 0;
      meta_bits.push(bit);

      if meta_bits.len() == meta_bits_len {
        break;
      }
    }
    if meta_bits.len() == meta_bits_len {
      break;
    }
  }

  // Convert meta bits to string
  let meta_str = bits_to_string(&meta_bits)?;

  // Parse meta information
  let (string_length, width_step, height_step) = meta_to_obj(&meta_str)?;

  // Parse step values into numeric types
  let width_step_num: u32 = width_step
    .parse()
    .map_err(|_| "Invalid width_step format")?;
  let height_step_num: u32 = height_step
    .parse()
    .map_err(|_| "Invalid height_step format")?;
  let string_length_num: usize = string_length
    .parse()
    .map_err(|_| "Invalid string_length format")?;

  // Calculate the number of bits to extract for the message
  let total_bits = string_length_num * 8;

  // Initialize message_bits
  let mut message_bits: Vec<bool> = Vec::with_capacity(total_bits);

  // Iterate through each pixel to extract message bits based on the step pattern,
  // skipping the meta bits.
  for y in (0..height).step_by(height_step_num as usize) {
    for x in (0..width).step_by(width_step_num as usize) {
      // Calculate the current bit position
      let current_bit = (y * width + x) as usize;

      // Skip meta bits
      if current_bit < meta_bits_len {
        continue;
      }

      let pixel = img.get_pixel(x, y);
      let bit = (pixel[3] & 1) != 0;
      message_bits.push(bit);

      if message_bits.len() == total_bits {
        break;
      }
    }
    if message_bits.len() == total_bits {
      break;
    }
  }

  // Convert message bits to string
  let message = bits_to_string(&message_bits)?;

  // Construct the pattern string
  let pattern = format!("{}x{}", width_step_num, height_step_num);

  // Output the decoded message and pattern
  println!(
    "{} was decoded!\nmessage: {}\npattern: {}",
    file_path, message, pattern
  );

  Ok((message, pattern))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::encode_rs;
  use image::{Rgba, RgbaImage};
  use tempfile::NamedTempFile;

  #[tokio::test]
  async fn test_decode() {
    let temp_in = NamedTempFile::new().unwrap();
    let temp_out = NamedTempFile::new().unwrap();

    // Create a test image
    let img = RgbaImage::from_fn(100, 100, |_, _| Rgba([255, 255, 255, 255]));
    img
      .save(temp_in.path().with_extension("png"))
      .expect("Failed to save input image");

    let test_message = "Test message";

    // Encode the message
    let encode_result = encode_rs(
      temp_in.path().with_extension("png").to_str().unwrap(),
      test_message,
      "1x1",
      temp_out.path().with_extension("png").to_str().unwrap(),
    )
    .await
    .expect("Failed to encode message");

    // Verify the encode result
    assert_eq!(
      encode_result.output,
      temp_out.path().with_extension("png").to_str().unwrap()
    );
    assert_eq!(encode_result.message, test_message);
    assert_eq!(encode_result.pattern, "1x1");

    // Decode the message
    let decode_result = decode_rs(temp_out.path().with_extension("png").to_str().unwrap())
      .await
      .expect("Failed to decode message");

    // Verify the decode result
    assert_eq!(decode_result.message, test_message);
    assert_eq!(decode_result.pattern, "1x1");

    println!("Decoding test successful.");
  }
}
