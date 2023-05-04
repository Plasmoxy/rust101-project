use axum::{
    extract::{Multipart, Path, Query, State},
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;
use image::imageops::resize;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde::{Deserialize};

use crate::images::{draw_bboxes_on_image, get_image_as_bytes, load_image_from_bytes};
use crate::{images::processing::Processing, neural::NeuralInferrer};

#[debug_handler]
pub async fn detect(State(inferrer): State<NeuralInferrer>, mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let buf = load_image_from_bytes(field).await.unwrap();

        let bboxes = inferrer.infer_face(&buf);
        let detected = draw_bboxes_on_image(
            resize(&buf, buf.width() / 4, buf.height() / 4, image::imageops::FilterType::Triangle),
            bboxes,
        );

        let bytes = get_image_as_bytes(detected).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn detect_bbox(State(inferrer): State<NeuralInferrer>, mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let buf = load_image_from_bytes(field).await.unwrap();
        let bboxes = inferrer.infer_face(&buf);

        return (StatusCode::OK, axum::Json::from(bboxes)).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn distort(mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let mut buf = load_image_from_bytes(field).await.unwrap();

        Processing::wobble(&mut buf);
        let bytes = get_image_as_bytes(buf).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn invert(mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let mut buf = load_image_from_bytes(field).await.unwrap();

        Processing::negative_basic(&mut buf);
        let bytes = get_image_as_bytes(buf).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn trim(mut data: Multipart) -> Response {
    if let Some(field) = data.next_field().await.unwrap() {
        let buf = load_image_from_bytes(field).await.unwrap();

        let trimmed = Processing::remove_borders(&buf).unwrap();
        let bytes = get_image_as_bytes(trimmed).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn rotate(Path(angle): Path<f32>, mut data: Multipart) -> Response {
    while let Some(field) = data.next_field().await.unwrap() {
        let buf = load_image_from_bytes(field).await.unwrap();

        let rotated = Processing::rotate(&buf, angle);
        let bytes = get_image_as_bytes(rotated).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[debug_handler]
pub async fn crop(Query(params): Query<CropParams>, mut data: Multipart) -> Response {
    while let Some(field) = data.next_field().await.unwrap() {
        let buf = load_image_from_bytes(field).await.unwrap();

        let cropped = Processing::crop_image(&buf, params.x, params.y, params.w, params.h);
        let bytes = get_image_as_bytes(cropped).unwrap();

        return (StatusCode::OK, [(CONTENT_TYPE, "image/png")], bytes).into_response();
    }

    (StatusCode::BAD_REQUEST).into_response()
}

#[derive(Deserialize)]
pub struct CropParams {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}
