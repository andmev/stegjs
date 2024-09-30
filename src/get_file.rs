use reqwest::Client;
use std::error::Error;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::checker::has_access;

/// Checks the readability of the file and returns the file path.
pub async fn by_path(img_path: &str) -> Result<String, Box<dyn Error>> {
  has_access(img_path).await
}

/// Takes a URL, downloads the file, and returns the full path to it.
pub async fn by_uri(uri: &str) -> Result<String, Box<dyn Error>> {
  let client = Client::new();
  let response = client.get(uri).send().await?;

  if !response.status().is_success() {
    return Err("Something wrong with URL or server".into());
  }

  let url = Url::parse(uri)?;
  let filename = url
    .path_segments()
    .and_then(|segments| segments.last())
    .ok_or("Cannot extract filename from URL")?
    .to_string();

  let path = PathBuf::from(&filename);
  let mut file = File::create(&path).await?;
  let content = response.bytes().await?;
  file.write_all(&content).await?;

  Ok(path.canonicalize()?.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::Path;
  use tokio::fs;

  #[tokio::test]
  async fn test_by_path() {
    let temp_file = "test_file.txt";
    fs::write(temp_file, "test content").await.unwrap();

    let result = by_path(temp_file).await;
    assert!(result.is_ok());

    fs::remove_file(temp_file).await.unwrap();
  }

  #[tokio::test]
  async fn test_by_uri() {
    // Use a reliable URL that we know contains an image
    let uri =
      "https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-128x128.png";
    let result = by_uri(uri).await;

    if let Err(e) = &result {
      eprintln!("Error in test_by_uri: {:?}", e);
    }
    assert!(result.is_ok());

    let binding = result.unwrap();
    let path = Path::new(&binding);
    assert!(path.exists());

    fs::remove_file(path).await.unwrap();
  }
}
