use binance_sdk::config::ConfigurationRestApi;
use dotenv::dotenv;

pub async fn get_binance_config() -> ConfigurationRestApi {
    dotenv().ok();

    let configuration = ConfigurationRestApi::builder()
        .base_path("https://api.binance.com")
        .api_key(dotenv::var("BINANCE_API_KEY").expect("API_KEY must be set"))
        .api_secret(dotenv::var("BINANCE_PASS").expect("PASS must be set"))
        .build()
        .unwrap();

    configuration
}

/*pub async fn get_account_params() {
    let configuration = get_binance_config().await;
    let client = wallet::WalletRestApi::production(configuration);
    let params = wallet::rest_api::AccountInfoParams::default();
    let response = client.account_info(params).await.unwrap();

    let data = response.data().await.unwrap();
    println!("{:#?}", data);
}*/