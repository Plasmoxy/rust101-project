pub mod routes;
pub mod util;

use self::routes::*;
use axum::{routing::post, Router};
use crate::neural::NeuralInferrer;

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