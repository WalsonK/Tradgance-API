/*let client = wallet::rest_api::CapitalApiClient::new(configuration);
    let params = client.all_coins_information(Default::default()).await.unwrap();
    let data = params.data().await.unwrap();
    println!("{:#?}", data);*/

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::wallet;
use crate::binance::tools;

fn check_balance() {
    let config = tools::get_binance_config();
    //let client = wallet::WalletRestApi::production(config);
}
