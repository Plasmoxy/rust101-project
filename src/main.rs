pub mod camera;
pub mod core;
pub mod images;
pub mod neural;

use std::time::Instant;

use image::imageops::resize;
use images::{draw_bboxes_on_image, load_image_buffer, processing::Processing, save_image_buffer};
use neural::NeuralInferrer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let inferrer = NeuralInferrer::new().await;

    let mut buf = load_image_buffer("girl.jpeg")?;

    // let bboxes = inferrer.infer_face(&buf);
    // println!("{:?}", bboxes);

    // let detected = draw_bboxes_on_image(
    //     resize(&buf, buf.width() / 4, buf.height() / 4, image::imageops::FilterType::Triangle),
    //     bboxes,
    // );
    // save_image_buffer("data/detected.jpg", &detected)?;

    let wobble_t = Instant::now();
    // Processing::negative_basic(&mut buf);
    // buf = Processing::crop_image(&buf, 800, 350, 1700, 2000);
    buf = Processing::rotate(&buf, 90.0);
    println!("Wobble {:?}", wobble_t.elapsed());

    save_image_buffer("data/out.jpg", &buf)?;

    Ok(())
}
