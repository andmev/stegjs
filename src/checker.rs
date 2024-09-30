use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use url::Url;

/// Checks that the file extension is PNG.
pub fn is_png(img_path: &str) -> bool {
  let re = Regex::new(r"(?i)\.png$").unwrap();
  re.is_match(img_path)
}

/// Checks if the given path is a valid HTTP or HTTPS URI.
pub fn is_uri(img_path: &str) -> bool {
  match Url::parse(img_path) {
    Ok(url) => url.scheme() == "http" || url.scheme() == "https",
    Err(_) => false,
  }
}

/// Checks read access to the file.
pub async fn has_access(img_path: &str) -> Result<String, Box<dyn Error>> {
  let path = Path::new(&img_path).canonicalize()?;
  fs::metadata(&path)?;
  Ok(path.to_string_lossy().into_owned())
}

/// Checks the syntax of the pattern. Returns a tuple of step width and height.
pub fn is_right_step(step: &str) -> Result<(String, String), &'static str> {
  let parts: Vec<&str> = step.split(['x', 'х'].as_ref()).collect();
  if parts.len() == 2 {
    let width = parts[0].parse::<i32>().map_err(|_| "Invalid width")?;
    let height = parts[1].parse::<i32>().map_err(|_| "Invalid height")?;
    Ok((width.to_string(), height.to_string()))
  } else {
    Err("Wrong step input. Check help!")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_png() {
    assert!(is_png("image.png"));
    assert!(!is_png("image.jpg"));
  }

  #[test]
  fn test_is_uri() {
    assert!(is_uri("http://example.com"));
    assert!(is_uri("https://example.com"));
    assert!(!is_uri("ftp://example.com"));
    assert!(!is_uri("not a uri"));
  }

  #[tokio::test]
  async fn test_has_access() {
    let result = has_access("Cargo.toml").await;
    assert!(result.is_ok());

    let result = has_access("nonexistent_file.png").await;
    assert!(result.is_err());
  }

  #[test]
  fn test_is_right_step() {
    assert_eq!(
      is_right_step("10x20").unwrap(),
      ("10".to_string(), "20".to_string())
    );
    assert_eq!(
      is_right_step("10х20").unwrap(),
      ("10".to_string(), "20".to_string())
    );
    assert!(is_right_step("10x").is_err());
    assert!(is_right_step("x20").is_err());
  }
}
