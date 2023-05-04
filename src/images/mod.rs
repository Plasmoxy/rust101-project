pub mod processing;

use std::io::{BufWriter, Cursor};
use std::time::Instant;

use axum::extract::multipart::Field;
use image::{io::Reader as ImageReader, load_from_memory, ImageBuffer, ImageFormat, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_hollow_rect, draw_text},
    rect::Rect,
};
use tract_onnx::tract_hir::tract_num_traits::ToPrimitive;

pub fn load_image_buffer(path: &str) -> anyhow::Result<RgbImage> {
    let start = Instant::now();
    let img = ImageReader::open(path)?.decode()?;

    let buf: RgbImage = img.into_rgb8(); // convert to rgb immediately

    println!("Loaded image {path} as {}x{} in {:?}", buf.width(), buf.height(), start.elapsed());

    Ok(buf)
}

pub async fn load_image_from_bytes(field: Field<'_>) -> anyhow::Result<RgbImage> {
    let start = Instant::now();

    let name = field.file_name().unwrap().to_string();
    let data = field.bytes().await?;
    let img = load_from_memory(&data)?;
    let buf: RgbImage = img.into_rgb8(); // convert to rgb immediately

    println!("Loaded image {name} as {}x{} in {:?}", buf.width(), buf.height(), start.elapsed());

    Ok(buf)
}

pub fn get_image_as_bytes(data: ImageBuffer<Rgb<u8>, Vec<u8>>) -> anyhow::Result<Vec<u8>> {
    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));

    data.write_to(&mut buffer, ImageFormat::Jpeg)?;
    let bytes: Vec<u8> = buffer.into_inner()?.into_inner();

    Ok(bytes)
}

pub fn save_image_buffer(path: &str, buf: &RgbImage) -> anyhow::Result<()> {
    let start = Instant::now();
    buf.save_with_format(path, image::ImageFormat::Jpeg)?;

    println!("Saved to {path} in {:?}", start.elapsed());
    Ok(())
}

/// Draw bounding boxes with confidence scores on the image.
/// https://github.com/sgasse/infercam_onnx/blob/main/infer_server/src/inferer.rs
pub fn draw_bboxes_on_image(mut frame: RgbImage, bboxes_with_confidences: Vec<([f32; 4], f32)>) -> RgbImage {
    let (width, height) = (frame.width().to_f32().unwrap(), frame.height().to_f32().unwrap());

    let color = Rgb::from([0, 255, 0]);

    for (bbox, confidence) in bboxes_with_confidences.iter() {
        // Coordinates of top-left and bottom-right points
        // Coordinate frame basis is on the top left corner
        let (x_tl, y_tl) = (bbox[0] * width, bbox[1] * height);
        let (x_br, y_br) = (bbox[2] * width, bbox[3] * height);
        let rect_width = x_br - x_tl;
        let rect_height = y_br - y_tl;

        let face_rect = Rect::at(x_tl as i32, y_tl as i32).of_size(rect_width as u32, rect_height as u32);

        frame = draw_hollow_rect(&frame, face_rect, Rgb::from([255, 0, 255]));
        // frame = draw_text(
        //     &frame,
        //     color,
        //     x_tl as i32,
        //     y_tl as i32,
        //     rusttype::Scale { x: 16.0, y: 16.0 },
        //     &DEJAVU_MONO,
        //     &format!("{:.2}%", confidence * 100.0),
        // );
    }

    frame
}
