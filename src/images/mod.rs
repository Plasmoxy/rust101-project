pub mod processing;

use image::{io::Reader as ImageReader, RgbaImage};

pub fn load_image_buffer(path: &str) -> anyhow::Result<RgbaImage> {
    let img = ImageReader::open(path)?.decode()?;

    let buf: RgbaImage = img.into_rgba8(); // convert to rgba immediately

    let bw = buf.width();
    let bh = buf.height();
    println!("Loaded image {path} as {bw}x{bh} rgba");

    Ok(buf)
}

pub fn save_image_buffer(path: &str, buf: &RgbaImage) -> anyhow::Result<()> {
    buf.save_with_format(path, image::ImageFormat::Png)?;

    println!("Saved to {path}");
    Ok(())
}
