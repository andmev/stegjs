use std::error::Error;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Client;
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