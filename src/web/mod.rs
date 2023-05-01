pub mod routes;

use axum::{routing::post, Router};
use serde::Serialize;

use crate::neural::NeuralInferrer;
use self::routes::{detect, distort, invert};

pub fn routes(inferrer: NeuralInferrer) -> Router {
	Router::new()
        .route("/detect", post(detect))
        .route("/distort", post(distort))
        .route("/invert", post(invert))
        .with_state(inferrer)
}

#[derive(Serialize)]
pub struct ImageResponse {
    name: String,
    data: Vec<u8>
}

impl ImageResponse {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, data }
    }
}