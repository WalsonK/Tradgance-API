use serde::Deserialize;
use crate::models::direction::Direction;

#[derive(Debug, Deserialize)]
pub struct TradeHttp {
    pub(crate) take_profit: f64,
    pub(crate) stop_loss: f64,
}
#[derive(Debug)]
pub struct TradeSignal {
    symbol: String,
    direction: Direction,
    entry_price: f64,
    take_profit: f64,
    stop_loss: f64,
    quantity: f64
}
impl TradeSignal {
    pub fn new(e: f64, tp: f64, sl:f64, risk_amount: f64) -> Self {
        let direction = TradeSignal::deduct_direction(e, tp, sl);
        let quantity = TradeSignal::calculate_qty(e, sl, risk_amount).unwrap();

        Self {
            symbol: "BTCUSDC".to_string(),
            direction,
            entry_price: e,
            take_profit: tp,
            stop_loss: sl,
            quantity
        }
    }

    pub fn from_str(body: String, risk_amount: f64) -> Option<Self> {
        let mut entry = None;
        let mut take_profit = None;
        let mut stop_loss = None;

        // Normaliser les séparateurs
        let normalized = body.replace(";", " ");

        for token in normalized.split_whitespace() {
            if let Some((key, value)) = token.split_once(':') {
                match key.trim().to_lowercase().as_str() {
                    "entry" => entry = value.trim().parse::<f64>().ok(),
                    "take_profit" => take_profit = value.trim().parse::<f64>().ok(),
                    "stop_loss" => stop_loss = value.trim().parse::<f64>().ok(),
                    _ => {}
                }
            }
        }

        let (entry, take_profit, stop_loss) = (
            entry?, take_profit?, stop_loss?
        );

        let direction = TradeSignal::deduct_direction(entry, take_profit, stop_loss);
        let quantity = TradeSignal::calculate_qty(entry, stop_loss, risk_amount).unwrap();

        Some(Self {
            symbol: "BTCUSDC".to_string(),
            direction,
            entry_price: entry,
            take_profit,
            stop_loss,
            quantity,
        })
    }

    pub fn calculate_qty(entry: f64, stop_loss: f64, risk_amount: f64) -> Option<f64> {
        let delta_price = (entry - stop_loss).abs();
        if delta_price <= 0.0 {
            return None; // éviter division par zéro
        }
        Some(risk_amount / delta_price)
    }

    pub fn deduct_direction(entry: f64, tp: f64, sl:f64) -> Direction {
        if tp > entry && entry > sl {
            Direction::Buy
        }else if tp < entry && entry < sl {
            Direction::Sell
        }else {
            panic!(
                "[Trade Signal] Cant deduct direction : \n\
                Signal incohérent: entry={entry}, tp={tp}, sl={sl}"
            );
        }
    }
}

