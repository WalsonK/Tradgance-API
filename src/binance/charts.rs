use binance_sdk::spot::rest_api::{RestApi, KlinesParams, KlinesIntervalEnum, KlinesItemInner};
use crate::binance::tools;
use anyhow::{Context, Result};

pub async fn last_closed_candle_close(symbol: &str) -> Result<f64> {
    let config = tools::get_binance_config().await;
    let restapi = RestApi::new(config);


    // On demande 2 bougies, pour éviter de tomber sur celle en cours de formation
    let params = KlinesParams::builder(symbol.to_string(), KlinesIntervalEnum::Interval1m)
        .limit(2)
        .build()?;
    let resp = restapi.klines(params).await?;

    let data = resp.data().await.context("no kline data")?;
    let idx = if data.len() >= 2 { data.len() - 2 } else { 0 };

    // Close = champ 4 de la kline [open_time, open, high, low, close, ...]
    match &data[idx][4] {
        KlinesItemInner::String(s) => Ok(s.parse::<f64>()?),
        KlinesItemInner::Integer(i) => Ok(*i as f64),
        KlinesItemInner::Other(v) => {
            let s = v.as_str().context("unexpected close type")?;
            Ok(s.parse::<f64>()?)
        }
    }
}