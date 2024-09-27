use std::error::Error;

use crate::checker::{is_png, is_uri};
use crate::converters::{bits_to_string, meta_to_obj};
use crate::get_file::{by_path, by_uri};

/// Decodes a hidden message from a PNG image using steganography.
///
/// # Parameters
/// - `img`: Path to the input PNG image or a URI.
///
/// # Errors
/// Returns an error if the input image is not a valid PNG, if decoding fails,
/// or if there are issues reading the files.
pub async fn decode_rs(img: &str) -> Result<(), Box<dyn Error>> {
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
    decode_image(&file_path)?;

    Ok(())
}

/// Helper function to decode the message from the image.
///
/// # Parameters
/// - `file_path`: Path to the input PNG image.
///
/// # Errors
/// Returns an error if decoding fails or if the message cannot be reconstructed.
fn decode_image(file_path: &str) -> Result<(), Box<dyn Error>> {
    // Load the image from the specified path.
    let img = image::open(file_path)?.to_rgba8();
    let (width, height) = img.dimensions();

    // Define the number of bits to read for meta-information.
    // Assuming meta string is 8 characters: "10|10"
    let meta_str_length = 8;
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
    let width_step_num: u32 = width_step.parse().map_err(|_| "Invalid width_step format")?;
    let height_step_num: u32 = height_step.parse().map_err(|_| "Invalid height_step format")?;
    let string_length_num: usize = string_length.parse().map_err(|_| "Invalid string_length format")?;

    // Calculate the number of bits to extract for the message
    let total_bits = string_length_num * 8;

    // Initialize message_bits
    let mut message_bits: Vec<bool> = Vec::with_capacity(total_bits);

    // {{ edit_start }}
    // Initialize a counter to keep track of bits read
    let mut bit_counter = 0;

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
            bit_counter += 1;

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

    // Output the decoded message
    println!(
        "{} was decoded!\nmessage: {}\npattern: {}x{}",
        file_path, message, width_step_num, height_step_num
    );

    Ok(())
}