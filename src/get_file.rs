use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use anyhow::{Result, anyhow};
use crate::checker::has_access;

/// Function checks the readability of the file and returns the file path.
pub async fn by_path(img_path: &str) -> Result<String> {
    has_access(img_path).await
}

/// Function takes a url, downloads the file and returns the full path to it.
pub async fn by_uri(uri: &str) -> Result<String> {
    let filename = uri.split('/').last().ok_or_else(|| anyhow!("Invalid URI"))?;
    let path = std::env::current_dir()?.join(filename);
    let path_str = path.to_str().ok_or_else(|| anyhow!("Invalid path"))?;

    let client = Client::new();
    let response = client.get(uri).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Something wrong with URL or server"));
    }

    let mut file = AsyncFile::create(&path).await?;
    let mut content = response.bytes().await?;
    file.write_all_buf(&mut content).await?;

    Ok(path_str.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
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
        let uri = "https://example.com/image.jpg";
        let result = by_uri(uri).await;
        assert!(result.is_ok());

        let binding = result.unwrap();
        let path = Path::new(&binding);
        assert!(path.exists());

        fs::remove_file(path).await.unwrap();
    }
}