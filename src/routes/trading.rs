use axum::Json;
use crate::binance;
use crate::models::trade::{TradeHttp, TradeSignal};
use tracing::{info};

pub async fn receive_trade_signal(Json(payload): Json<TradeHttp>) {
    info!("[TRADING] Received trade signal with payload {:?}", payload);

    // Vérification de la balance et calcul du risk
    let balances = binance::wallet::get_balance("USDC".to_string()).await.unwrap();
    let risk_amount = binance::tools::money_management(balances.1.to_string()).await.unwrap();
    info!("[TRADING] Binance data well fetch : {:?} & risk calculated : {}", balances, risk_amount);

    // récuperation de l'entrée :
    let last_candle_value = binance::charts::last_closed_candle_close("BTCUSDC").await.unwrap();
    info!("[TRADING] Last candle value well fetch : {}", last_candle_value);

    // Calc trade signal
    let signal: TradeSignal = TradeSignal::new(last_candle_value, payload.take_profit, payload.stop_loss, risk_amount);
    info!(
        signal = %signal,
        balance_currency = %balances.0,
        balance_value = %balances.1,
        risk = %risk_amount,
        "[TRADING] Trade ready 🚀"
    );
}