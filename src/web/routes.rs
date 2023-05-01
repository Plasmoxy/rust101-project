use axum::{extract::{Multipart, State}, Json};
use axum_macros::debug_handler;

use image::imageops::resize;
use std::time::Instant;

use crate::{neural::NeuralInferrer, images::processing::Processing};
use crate::images::{draw_bboxes_on_image, load_image_from_bytes, get_image_as_bytes};
use crate::web::ImageResponse;

#[debug_handler]
pub async fn detect(State(inferrer): State<NeuralInferrer>, mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();
    
    while let Some(field) = data.next_field().await.unwrap() {
        let (name, buf) = load_image_from_bytes(field).await.unwrap();
        
        let bboxes = inferrer.infer_face(&buf);
        let detected = draw_bboxes_on_image(
            resize(&buf, buf.width() / 4, buf.height() / 4, image::imageops::FilterType::Triangle),
            bboxes,
        );

        let bytes = get_image_as_bytes(detected).unwrap();
    
        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}

#[debug_handler]
pub async fn distort(mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();
    
    while let Some(field) = data.next_field().await.unwrap() {
        let (name, mut buf) = load_image_from_bytes(field).await.unwrap();    

        let time = Instant::now();
        Processing::wobble(&mut buf);
        println!("Wobble of {} finished in {:?}", name, time.elapsed());

        let bytes = get_image_as_bytes(buf).unwrap();
    
        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}

#[debug_handler]
pub async fn invert(mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();
    
    while let Some(field) = data.next_field().await.unwrap() {
        let (name, mut buf) = load_image_from_bytes(field).await.unwrap();    

        let time = Instant::now();
        Processing::negative_basic(&mut buf);
        println!("Inversion of {} finished in {:?}", name, time.elapsed());

        let bytes = get_image_as_bytes(buf).unwrap();
    
        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}
