use axum::Router;
use axum::routing::get;
use dotenv::dotenv;

mod routes;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}