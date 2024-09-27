use anyhow::{Result, anyhow};

/// The function converts string into a vector of booleans (bits).
pub fn string_to_bits(str: &str) -> Vec<bool> {
    str.as_bytes()
        .iter()
        .flat_map(|&byte| {
            (0..8).rev().map(move |i| ((byte >> i) & 1) == 1)
        })
        .collect()
}

/// The function converts a vector of booleans (bits) into a readable string.
pub fn bits_to_string(bits: &[bool]) -> Result<String> {
    if bits.len() % 8 != 0 {
        return Err(anyhow!("Number of bits must be divisible by 8"));
    }

    let bytes: Vec<u8> = bits.chunks(8)
        .map(|chunk| {
            chunk.iter().fold(0u8, |acc, &bit| {
                (acc << 1) | (bit as u8)
            })
        })
        .collect();

    String::from_utf8(bytes).map_err(|e| anyhow!("Invalid UTF-8 sequence: {}", e))
}

/// Split meta-information.
pub fn meta_to_obj(str: &str) -> Result<(String, String, String)> {
    let parts: Vec<&str> = str.split('|').collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid meta-information format"));
    }
    Ok((parts[0].to_string(), parts[1].to_string(), parts[2].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_bits() {
        let input = "Hello";
        let expected = vec![
            false, true, false, false, true, false, false, false,
            false, true, true, false, false, true, false, true,
            false, true, true, false, true, true, false, false,
            false, true, true, false, true, true, false, false,
            false, true, true, false, true, true, true, true
        ];
        assert_eq!(string_to_bits(input), expected);
    }

    #[test]
    fn test_bits_to_string() {
        let input = vec![
            false, true, false, false, true, false, false, false,
            false, true, true, false, false, true, false, true,
            false, true, true, false, true, true, false, false,
            false, true, true, false, true, true, false, false,
            false, true, true, false, true, true, true, true
        ];
        assert_eq!(bits_to_string(&input).unwrap(), "Hello");
    }

    #[test]
    fn test_meta_to_obj() {
        let input = "width|height|filename";
        let (width, height, filename) = meta_to_obj(input).unwrap();
        assert_eq!(width, "width");
        assert_eq!(height, "height");
        assert_eq!(filename, "filename");
    }

    #[test]
    fn test_meta_to_obj_error() {
        let input = "invalid|format";
        assert!(meta_to_obj(input).is_err());
    }
}