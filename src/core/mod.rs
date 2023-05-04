use std::{fs::File, io::Cursor};

use reqwest::Client;

/// Boxed trait obj error type for nn module
pub type DynError = Box<dyn std::error::Error>;

/// Bounding box defined as `[x_top_left, y_top_left, x_bottom_right, y_bottom_right]`.
pub type Bbox = [f32; 4];

pub async fn download_file(client: &Client, url: &str, filepath: impl AsRef<std::path::Path>) -> Result<(), DynError> {
    let resp = client.get(url).send().await?;

    let mut file = File::create(filepath)?;
    let mut content = Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}
