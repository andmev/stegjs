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
  let parts: Vec<&str> = step.split(|c| c == 'x' || c == 'х').collect();
  if parts.len() == 2 {
    let width = parts[0].parse::<i32>().map_err(|_| "Invalid width")?;
    let height = parts[1].parse::<i32>().map_err(|_| "Invalid height")?;
    Ok((width.to_string(), height.to_string()))
  } else {
    Err("Wrong step input. Check help!")
  }
}
