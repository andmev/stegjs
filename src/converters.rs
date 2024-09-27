use std::error::Error;

/// Converts a string into a vector of bits.
/// Each character is converted to its UTF-8 byte representation,
/// and each byte is split into its constituent bits.
pub fn string_to_bits(s: &str) -> Vec<bool> {
    s.bytes()
        .flat_map(|byte| {
            (0..8).rev().map(move |i| (byte & (1 << i)) != 0)
        })
        .collect()
}

/// Converts a slice of bits back into a readable string.
/// The bits are grouped into bytes, which are then converted to characters.
/// Returns an error if the number of bits is not a multiple of 8 or if invalid UTF-8 is encountered.
pub fn bits_to_string(bits: &[bool]) -> Result<String, Box<dyn Error>> {
    if bits.len() % 8 != 0 {
        return Err("Number of bits is not a multiple of 8".into());
    }

    let bytes = bits
        .chunks(8)
        .map(|chunk| {
            chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | (bit as u8))
        })
        .collect::<Vec<u8>>();

    Ok(String::from_utf8(bytes)?)
}

/// Splits meta-information string into a tuple of three strings.
/// Returns an error if the input does not contain exactly two pipe ('|') separators.
pub fn meta_to_obj(s: &str) -> Result<(String, String, String), &'static str> {
    let parts: Vec<&str> = s.split('|').collect();
    if parts.len() == 3 {
        Ok((parts[0].to_string(), parts[1].to_string(), parts[2].to_string()))
    } else {
        Err("Input string does not contain exactly two '|' separators")
    }
}