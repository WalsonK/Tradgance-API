#[cfg(test)]
mod tests {
    use crate::models::trade::TradeSignal;

    #[test]
    fn test_trade_signal() {
        let qty = TradeSignal::calculate_qty(100606.63, 100606.7, 0.2);

        let correct_trade_signal = TradeSignal {
            devices: "USDC/BTC".to_string(),
            direction: "Achat".to_string(),
            entry_price: 100606.63,
            take_profit: 103487.79865253,
            stop_loss: 100606.7,
            quantity: qty.unwrap(),
        };

        let trade_signal = TradeSignal::new(
            "Direction:Achat;entry:100606.63;take_profit:103487.79865253;stop_loss:100606.7".to_string(),
            0.2
        ).unwrap();

        println!("{:?}", trade_signal);

        assert_eq!(correct_trade_signal.quantity, trade_signal.quantity);
        assert_eq!(correct_trade_signal.stop_loss, trade_signal.stop_loss);
        assert_eq!(correct_trade_signal.take_profit, trade_signal.take_profit);
        assert_eq!(correct_trade_signal.entry_price, trade_signal.entry_price);
        assert_eq!(correct_trade_signal.direction, trade_signal.direction);
    }
}