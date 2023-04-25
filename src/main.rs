pub mod camera;
pub mod core;
pub mod images;

use images::{
    load_image_buffer,
    processing::{negative_basic, wobble},
    save_image_buffer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut buf = load_image_buffer("girl.jpeg")?;

    wobble(&mut buf);

    save_image_buffer("data/out.png", &buf)?;

    Ok(())
}
