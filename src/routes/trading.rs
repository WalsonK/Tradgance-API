use axum::Json;
use crate::binance;
use crate::models::trade::{TradeHttp, TradeSignal};

pub async fn receive_trade_signal(Json(payload): Json<TradeHttp>) {
    println!("Requête reçue pour trading via front : {:?}", payload);

    // Vérification de la balance et calcul du risk
    let balances = binance::wallet::get_balance("USDC".to_string()).await.unwrap();
    let risk_amount = binance::tools::money_management().await.unwrap();
    println!("- Balances : {:?}", balances);
    println!("- Risk amount : {}", risk_amount);

    // Calc trade signal
}