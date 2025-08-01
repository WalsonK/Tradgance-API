use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::wallet;

mod routes;
mod models;
mod binance;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let configuration = ConfigurationRestApi::builder()
        .api_key(dotenv::var("BINANCE_API_KEY").expect("API_KEY must be set"))
        .api_secret(dotenv::var("BINANCE_PASS").expect("PASS must be set"))
        .build()
        .unwrap();

    let client = wallet::WalletRestApi::production(configuration);
    let params = wallet::rest_api::AccountInfoParams::default();
    let response = client.account_info(params).await.unwrap();

    let data = response.data().await.unwrap();
    println!("{:#?}", data);

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}