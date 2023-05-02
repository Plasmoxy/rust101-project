pub mod routes;

use self::routes::*;
use crate::neural::NeuralInferrer;
use axum::{routing::post, Router};

pub fn routes(inferrer: NeuralInferrer) -> Router {
    Router::new()
        .route("/detect", post(detect))
        .route("/distort", post(distort))
        .route("/invert", post(invert))
        .route("/trim", post(trim))
        .route("/rotate/:angle", post(rotate))
        .route("/crop", post(crop))
        .with_state(inferrer)
}
