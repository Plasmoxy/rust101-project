pub mod routes;
pub mod util;

use self::routes::*;
use crate::neural::NeuralInferrer;
use axum::{routing::post, Router};

pub fn routes(inferrer: NeuralInferrer) -> Router {
    Router::new()
        .route("/detect", post(detect))
        .route("/detect-single", post(detect_single))
        .route("/distort", post(distort))
        .route("/invert", post(invert))
        .route("/trim", post(trim))
        .route("/rotate/:angle", post(rotate))
        .route("/crop", post(crop))
        .with_state(inferrer)
}
