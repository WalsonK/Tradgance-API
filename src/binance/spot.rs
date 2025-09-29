use crate::models::trade::TradeSignal;
use binance_sdk::spot::rest_api::{RestApi, NewOrderSideEnum, NewOrderParams, NewOrderTypeEnum, OrderOcoParams, OrderOcoSideEnum};
use rust_decimal::{Decimal};
use rust_decimal::prelude::FromPrimitive;
use crate::binance::tools;
use crate::models::direction::Direction;
use tracing::{info};

pub async fn execute_trade(signal: TradeSignal) {
    let config = tools::get_binance_config().await;
    let api = RestApi::new(config);

    let order_side = match signal.direction {
        Direction::Buy => NewOrderSideEnum::Buy,
        Direction::Sell => NewOrderSideEnum::Sell
    };
    let qty = Decimal::from_f64(signal.quantity.clone()).unwrap();

    let order_params = NewOrderParams::builder(
        signal.symbol.clone(),
        order_side,
        NewOrderTypeEnum::Market
    ).quantity(qty.clone()).build().unwrap();

    let _ = api.new_order(order_params).await;
    info!("[SPOT] Market Order executed");


    // Étape 4 : Placer l'ordre OCO pour le TP et le SL
    let oco_params = OrderOcoParams::builder(
        signal.symbol,
        OrderOcoSideEnum::Sell,
        qty, Decimal::from_f64(signal.take_profit).unwrap(),
        Decimal::from_f64(signal.stop_loss).unwrap()
    ).build().unwrap();

    let _ = api.order_oco(oco_params).await;
    info!("[SPOT] OCO Order placed");
}