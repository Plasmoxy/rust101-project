use axum::extract::multipart::Field;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CropImage {
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl CropImage {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_some() && self.y.is_some() && self.width.is_some() && self.height.is_some()
    }
}

#[derive(Serialize)]
pub struct ImageResponse {
    name: String,
    data: Vec<u8>,
}

impl ImageResponse {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, data }
    }
}

pub async fn parse_u32(field: Field<'_>) -> anyhow::Result<u32> {
    let str = field.text().await?;
    let num: u32 = str.parse()?;
    Ok(num)
}
