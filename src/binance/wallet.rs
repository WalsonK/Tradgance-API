use binance_sdk::wallet;
use binance_sdk::wallet::rest_api::{AllCoinsInformationParams, CapitalApi};
use crate::binance::tools;
use anyhow::Result;
use tracing::error;

pub async fn get_balance(wanted_symbol: String) -> Result<(String, String)> {
    let config = tools::get_binance_config().await;
    let capital_client = wallet::rest_api::CapitalApiClient::new(config);
    let params = AllCoinsInformationParams { recv_window: Some(5_000) };

    let res_result = capital_client.all_coins_information(params).await?;

    let coins = res_result.data().await?;

    for c in &coins {
        if let Some(symbol) = &c.coin {
            if symbol == &wanted_symbol {
                let free_amount = c.free.as_deref().unwrap_or("0").to_string();
                return Ok((symbol.clone(), free_amount));
            }
        }
    }

    error!("[WALLET] USDC not found");
    Err(anyhow::anyhow!("USDC non trouvé"))
}