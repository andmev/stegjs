use crate::checker::{is_png, is_right_step, is_uri};
use crate::converters::string_to_bits;
use crate::get_file::{by_path, by_uri};
use anyhow::{anyhow, Result};
use image::{GenericImage, GenericImageView};
use std::fs::File;
use std::io::BufWriter;

/// Function to encode a message into an image
pub async fn encode(img: &str, msg: &str, step: &str, out: &str) -> Result<()> {
    if !is_png(img) {
        return Err(anyhow!("Only *.png images are supported."));
    }

    let img_path = if is_uri(img) {
        by_uri(img).await?
    } else {
        by_path(img).await?
    };

    encode_image(&img_path, msg, step, out)
}

fn encode_image(img_path: &str, msg: &str, step: &str, out: &str) -> Result<()> {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_encode() {
        let temp_in = NamedTempFile::new().unwrap();
        let temp_out = NamedTempFile::new().unwrap();
        
        // Create a test image
        let img = RgbaImage::from_fn(100, 100, |_, _| Rgba([255, 255, 255, 255]));
        img.save(temp_in.path()).unwrap();

        let result = encode(
            temp_in.path().to_str().unwrap(),
            "Test message",
            "1x1",
            temp_out.path().to_str().unwrap()
        ).await;

        assert!(result.is_ok());
    }
}