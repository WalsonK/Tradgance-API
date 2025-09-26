use axum::http::{header, HeaderValue, Method};
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;
use tracing::info;

mod routes;
mod models;
mod binance;
mod mail;
mod tester;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tools::init_logger();

    // cors
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_credentials(true);

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello))
        .route("/trade", post(routes::trading::receive_trade_signal))
        .layer(cors);

    // run app
    let addr = "127.0.0.1:3001";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    info!("[MAIN] API started & Listening on http://{} 🚀", addr);

    axum::serve(listener, app).await?;

    Ok(())
}