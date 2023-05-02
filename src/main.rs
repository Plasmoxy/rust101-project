pub mod camera;
pub mod core;
pub mod images;
pub mod neural;
pub mod web;

use neural::NeuralInferrer;
use std::net::SocketAddr;
use web::routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let inferrer = NeuralInferrer::new().await?;

    let routes = routes(inferrer);
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING on {address}\n");
    axum::Server::bind(&address).serve(routes.into_make_service()).await.unwrap();

    Ok(())
}
