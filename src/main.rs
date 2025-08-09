use axum::Router;
use axum::routing::get;
use binance_sdk::wallet;

mod routes;
mod models;
mod binance;

#[tokio::main]
async fn main() {
    binance::tools::get_account_params().await;

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}