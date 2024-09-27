use tokio::fs::File;
use url::Url;
use anyhow::{Result, anyhow};

/// Checks that the file extension is PNG.
pub fn is_png(img_path: &str) -> bool {
    img_path.to_lowercase().ends_with(".png")
}

/// Check if the given path is a valid URI.
pub fn is_uri(img_path: &str) -> bool {
    if let Ok(url) = Url::parse(img_path) {
        url.scheme() == "http" || url.scheme() == "https"
    } else {
        false
    }
}

/// Checks read access to the file.
pub async fn has_access(img_path: &str) -> Result<String> {
    let full_path = std::env::current_dir()?.join(img_path);
    let canonical_path = full_path.canonicalize()?;

    // Check if file exists and is readable
    File::open(&canonical_path).await?;

    Ok(canonical_path.to_string_lossy().into_owned())
}

/// Checks the syntax of the pattern. Returns a tuple of step length and height.
pub fn is_right_step(step: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = step.split(['x', 'х']).collect();

    if parts.len() != 2 {
        return Err(anyhow!("Wrong step input. Check help!"));
    }

    let width = parts[0].parse::<u32>()
        .map_err(|_| anyhow!("Invalid width in step"))?;
    let height = parts[1].parse::<u32>()
        .map_err(|_| anyhow!("Invalid height in step"))?;

    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_is_png() {
        assert!(is_png("image.png"));
        assert!(is_png("image.PNG"));
        assert!(!is_png("image.jpg"));
    }

    #[test]
    fn test_is_uri() {
        assert!(is_uri("http://example.com/image.png"));
        assert!(is_uri("https://example.com/image.png"));
        assert!(!is_uri("ftp://example.com/image.png"));
        assert!(!is_uri("/path/to/image.png"));
    }

    #[tokio::test]
    async fn test_has_access() {
        let temp_file = "test_file.txt";
        tokio::fs::write(temp_file, "test content").await.unwrap();

        let result = has_access(temp_file).await;
        assert!(result.is_ok());

        tokio::fs::remove_file(temp_file).await.unwrap();
    }

    #[test]
    fn test_is_right_step() {
        assert_eq!(is_right_step("10x20"), Ok((10, 32)));
        assert_eq!(is_right_step("10х20"), Ok((10, 32)));
        assert!(is_right_step("10").is_err());
        assert!(is_right_step("10x20x30").is_err());
        assert!(is_right_step("axb").is_err());
    }
}