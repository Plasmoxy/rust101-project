use axum::{
    extract::{Multipart, Path, State},
    response::{IntoResponse, Response},
    Json,
};
use axum_macros::debug_handler;
use image::{imageops::resize, ImageBuffer, ImageFormat, Rgb};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use std::{
    io::{BufWriter, Cursor},
    time::Instant,
};

use crate::images::{draw_bboxes_on_image, get_image_as_bytes, load_image_from_bytes};
use crate::{
    images::processing::Processing,
    neural::NeuralInferrer,
    web::util::{parse_u32, CropImage, ImageResponse},
};

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
pub async fn detect_single(State(inferrer): State<NeuralInferrer>, mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let (name, buf) = load_image_from_bytes(field).await.unwrap();

        let bboxes = inferrer.infer_face(&buf);
        let detected = draw_bboxes_on_image(
            resize(&buf, buf.width() / 4, buf.height() / 4, image::imageops::FilterType::Triangle),
            bboxes,
        );

        let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
        detected.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
        let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();
        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
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

#[debug_handler]
pub async fn trim(mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();

    while let Some(field) = data.next_field().await.unwrap() {
        let (name, mut buf) = load_image_from_bytes(field).await.unwrap();

        let time = Instant::now();
        buf = Processing::remove_borders(&buf).unwrap();
        println!("Removed black borders from {} in {:?}", name, time.elapsed());

        let bytes = get_image_as_bytes(buf).unwrap();

        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}

#[debug_handler]
pub async fn rotate(Path(angle): Path<f32>, mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();

    while let Some(field) = data.next_field().await.unwrap() {
        let (name, mut buf) = load_image_from_bytes(field).await.unwrap();

        let time = Instant::now();
        buf = Processing::rotate(&buf, angle);
        println!("Rotation of {} finished in {:?}", name, time.elapsed());

        let bytes = get_image_as_bytes(buf).unwrap();

        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}

#[debug_handler]
pub async fn crop(mut data: Multipart) -> Json<Vec<ImageResponse>> {
    let mut response: Vec<ImageResponse> = Vec::new();

    let mut args = CropImage::new();
    let mut file_name: Option<String> = None;
    let mut buffer: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> = None;

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "x" => {
                args.x = Some(parse_u32(field).await.unwrap());
                continue;
            }
            "y" => {
                args.y = Some(parse_u32(field).await.unwrap());
                continue;
            }
            "width" => {
                args.width = Some(parse_u32(field).await.unwrap());
                continue;
            }
            "height" => {
                args.height = Some(parse_u32(field).await.unwrap());
                continue;
            }
            "image" => {
                let (name, buf) = load_image_from_bytes(field).await.unwrap();
                file_name = Some(name);
                buffer = Some(buf);
            }
            _ => (),
        };
    }

    if args.is_valid() && file_name.is_some() && buffer.is_some() {
        let time = Instant::now();
        let name = file_name.unwrap();
        let mut buf = buffer.unwrap();

        buf = Processing::crop_image(&buf, args.x.unwrap(), args.y.unwrap(), args.width.unwrap(), args.height.unwrap());

        println!("Cropping of {} finished in {:?}", name, time.elapsed());
        let bytes = get_image_as_bytes(buf).unwrap();

        response.push(ImageResponse::new(name, bytes));
    }

    Json(response)
}
