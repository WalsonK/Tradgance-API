use axum::Router;
use axum::routing::get;

mod routes;
mod models;
mod binance;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //binance::tools::get_account_params().await;
    //let balances = binance::wallet::get_balance("USDC".to_string()).await?;
    //println!("{:?}", balances);

    let risk = binance::tools::money_management().await?;
    println!("Risk: {}", risk);

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}