use crate::checker::has_access;
use anyhow::{anyhow, Result};
use reqwest::Client;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;

/// Function checks the readability of the file and returns the file path.
pub async fn by_path(img_path: &str) -> Result<String> {
  has_access(img_path).await
}

/// Function takes an url, downloads the file and returns the full path to it.
pub async fn by_uri(uri: &str) -> Result<String> {
  println!("Downloading from URI: {}", uri);
  let filename = uri
    .split('/')
    .last()
    .ok_or_else(|| anyhow!("Invalid URI"))?;
  let path = std::env::current_dir()?.join(filename);
  let path_str = path.to_str().ok_or_else(|| anyhow!("Invalid path"))?;

  println!("Saving to path: {}", path_str);

  let client = Client::new();
  let response = client.get(uri).send().await?;

  println!("Response status: {}", response.status());

  if !response.status().is_success() {
    return Err(anyhow!("Something wrong with URL or server"));
  }

  let mut file = AsyncFile::create(&path).await?;
  let mut content = response.bytes().await?;
  file.write_all_buf(&mut content).await?;

  println!("File downloaded and saved");

  Ok(path_str.to_string())
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
    let uri = "https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-128x128.png";
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
