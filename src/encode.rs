use std::error::Error;

use crate::checker::{is_png, is_right_step, is_uri};
use crate::converters::string_to_bits;
use crate::get_file::{by_path, by_uri};

/// Encodes a message into a PNG image using steganography.
///
/// # Parameters
/// - `img`: Path to the input PNG image or a URI.
/// - `msg`: The message to encode into the image.
/// - `step`: The step pattern for encoding the bits into the image.
/// - `out`: Path to the output PNG image.
///
/// # Errors
/// Returns an error if the input image is not a valid PNG, if the message does not fit,
/// or if there are issues reading or writing the files.
pub async fn encode_rs(
    img: &str,
    msg: &str,
    step: &str,
    out: &str,
) -> Result<(), Box<dyn Error>> {
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

    // Proceed to encode the message into the image.
    encode_image(&file_path, msg, step, out)?;

    Ok(())
}

/// Helper function to encode the message into the image.
///
/// # Parameters
/// - `file_path`: Path to the input PNG image.
/// - `msg`: The message to encode.
/// - `step`: The step pattern for encoding.
/// - `out`: Path to the output PNG image.
///
/// # Errors
/// Returns an error if encoding fails or if the message does not fit within the image.
fn encode_image(
    file_path: &str,
    msg: &str,
    step: &str,
    out: &str,
) -> Result<(), Box<dyn Error>> {
    // Load the image from the specified path.
    let img = image::open(file_path)?.to_rgba8();
    let (width, _height) = img.dimensions();
    let mut img = img.clone();

    // Validate and parse the step pattern.
    let (width_step, height_step) = match is_right_step(step) {
        Ok((w, h)) => (w.parse::<u32>()?, h.parse::<u32>()?),
        Err(e) => return Err(e.into()),
    };

    // Prepare meta-information.
    let text_length = msg.len();
    // Fixed-length meta string: e.g., "10|10" (8 characters)
    let meta = format!("{:02}|{:02}|{:02}", text_length, width_step, height_step);

    let meta_bits = string_to_bits(&meta);

    let msg_bits = string_to_bits(msg);

    // Initialize bit indices.
    let mut meta_index = 0;
    let mut msg_index = 0;

    // Flag to indicate if all bits have been encoded.
    let mut encoding_complete = false;

    // Iterate through each pixel to encode meta bits and message bits.
    for (y, x, pixel) in img.enumerate_pixels_mut() {
        let _idx = (y * width + x) as usize;

        // Encode meta bits into the alpha channel.
        if meta_index < meta_bits.len() {
            let bit = meta_bits[meta_index];
            let alpha = pixel[3];
            let new_alpha = if bit {
                alpha | 1 // Set LSB to 1.
            } else {
                alpha & 254 // Set LSB to 0.
            };
            pixel[3] = new_alpha;
            meta_index += 1;
            continue; // Continue to encode meta bits first.
        }

        // Encode message bits based on the step pattern.
        if y % height_step == 0 && x % width_step == 0 && msg_index < msg_bits.len() {
            let bit = msg_bits[msg_index];
            let alpha = pixel[3];
            let new_alpha = if bit {
                alpha | 1 // Set LSB to 1.
            } else {
                alpha & 254 // Set LSB to 0.
            };
            pixel[3] = new_alpha;
            msg_index += 1;
        }

        // Check if all bits have been encoded.
        if meta_index >= meta_bits.len() && msg_index >= msg_bits.len() {
            encoding_complete = true;
            break;
        }
    }

    // Ensure that all bits were encoded.
    if !encoding_complete {
        return Err("A very long message or a wide step! This amount of text does not fit in this picture.".into());
    }

    // Save the modified image to the output path.
    img.save(out)?;

    println!(
        "{} has been encoded\nmessage: {}\npattern: {}",
        out, msg, step
    );

    Ok(())
}