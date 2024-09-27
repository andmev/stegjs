use crate::checker::{is_png, is_right_step, is_uri};
use crate::converters::string_to_bits;
use crate::get_file::{by_path, by_uri};
use anyhow::{anyhow, Result};
use image::{GenericImage, GenericImageView};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Function to encode a message into an image
pub async fn encode(img: &str, msg: &str, step: &str, out: &str) -> Result<()> {
    println!("Encoding started");
    println!("Input image path: {}", img);
    
    let path = Path::new(img);
    println!("File extension: {:?}", path.extension());
    
    if !is_png(img) {
        println!("is_png check failed");
        return Err(anyhow!("Only *.png images are supported."));
    }
    println!("Image is PNG");

    let img_path = if is_uri(img) {
        println!("Image is URI");
        by_uri(img).await?
    } else {
        println!("Image is local path");
        by_path(img).await?
    };
    println!("Image path: {}", img_path);

    encode_image(&img_path, msg, step, out)
}

fn encode_image(img_path: &str, msg: &str, step: &str, out: &str) -> Result<()> {
    println!("Encoding image");
    let (width_step, height_step) = is_right_step(step)?;

    let mut img = image::open(img_path)?;
    let (width, height) = img.dimensions();

    let text_length = msg.len();
    let meta = format!("{}|{}|{}|", text_length, width_step, height_step);

    let meta_bits = string_to_bits(&meta);
    let msg_bits = string_to_bits(msg);

    let mut meta_index = 0;
    let mut msg_index = 0;

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            // Encode meta information
            if meta_index < meta_bits.len() {
                pixel[0] = if meta_bits[meta_index] {
                    pixel[0] | 1
                } else {
                    pixel[0] & 254
                };
                meta_index += 1;
            }

            // Encode message
            if y % height_step == 0 && x % width_step == 0 && msg_index < msg_bits.len() {
                pixel[3] = if msg_bits[msg_index] {
                    pixel[3] | 1
                } else {
                    pixel[3] & 254
                };
                msg_index += 1;
            }

            img.put_pixel(x, y, pixel);
        }
    }

    if msg_index < msg_bits.len() {
        return Err(anyhow!("Error: A very long message or a wide step! This amount of text does not fit in this picture. Please reduce step, length of the text or use image with higher resolution."));
    }

    let file = File::create(out)?;
    let mut w = BufWriter::new(file);

    img.write_to(&mut w, image::ImageFormat::Png)?;

    println!("{} has been encoded", out);
    println!("message: {}", msg);
    println!("pattern: {}", step);

    println!("Encoding completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};
    use tempfile::NamedTempFile;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_encode() {
        let temp_in = NamedTempFile::new().unwrap();
        let temp_out = NamedTempFile::new().unwrap();

        // Create a path with .png extension
        let mut input_path = PathBuf::from(temp_in.path());
        input_path.set_extension("png");

        // Create a test image
        let img = RgbaImage::from_fn(100, 100, |_, _| Rgba([255, 255, 255, 255]));
        img.save_with_format(&input_path, image::ImageFormat::Png).unwrap();

        println!("Test image saved at: {:?}", input_path);
        println!("Test image extension: {:?}", input_path.extension());

        let result = encode(
            input_path.to_str().unwrap(),
            "Test message",
            "1x1",
            temp_out.path().to_str().unwrap(),
        )
        .await;

        if let Err(e) = &result {
            eprintln!("Encode error: {:?}", e);
        }
        assert!(result.is_ok());

        // Check if the output file exists and is not empty
        let metadata = fs::metadata(temp_out.path()).unwrap();
        assert!(metadata.len() > 0, "Output file is empty");
    }
}
