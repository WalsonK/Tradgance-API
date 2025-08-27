#[derive(Debug)]
pub struct TradeSignal {
    pub(crate) devices: String,
    pub(crate) direction: String,
    pub(crate) entry_price: f64,
    pub(crate) take_profit: f64,
    pub(crate) stop_loss: f64,
    pub(crate) quantity: f64
}
impl TradeSignal {
    pub fn new(body: String, risk_amount: f64) -> Option<Self> {
        let mut direction = None;
        let mut entry = None;
        let mut take_profit = None;
        let mut stop_loss = None;

        // Normaliser les séparateurs
        let normalized = body.replace(";", " ");

        for token in normalized.split_whitespace() {
            if let Some((key, value)) = token.split_once(':') {
                match key.trim().to_lowercase().as_str() {
                    "Direction" => direction = Some(value.trim().to_string()),
                    "entry" => entry = value.trim().parse::<f64>().ok(),
                    "take_profit" => take_profit = value.trim().parse::<f64>().ok(),
                    "stop_loss" => stop_loss = value.trim().parse::<f64>().ok(),
                    _ => {}
                }
            }
        }

        let (direction, entry, take_profit, stop_loss) = (
            direction?, entry?, take_profit?, stop_loss?
        );

        let quantity = TradeSignal::calculate_qty(entry, stop_loss, risk_amount).unwrap();

        Some(Self {
            devices: "USDC/BTC".to_string(),
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
}

