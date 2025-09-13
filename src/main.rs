use axum::Router;
use axum::routing::get;

mod routes;
mod models;
mod binance;
mod mail;
mod tester;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tokio::spawn(async move {
        // 1. Component : récupération ou reconnexion au serveur
        let mut sess = mail::mailer::wait_online_session();

        loop {
            let mut idle = sess.idle();

            // 2. Component : attend détection via IDLE
            mail::monitor::wait_for_new_mail(&mut idle);
            drop(idle);

            // 3. Component : Vérification de la balance et calcul du risk
            println!("[monitor] Mail reçu :\n");
            let balances = binance::wallet::get_balance("USDC".to_string()).await.unwrap();
            println!("- Balances : {:?}", balances);
            let risk_amount = binance::tools::money_management().await.unwrap();
            println!("- Risk amount : {}", risk_amount);

            // 4. Component : extraction d’un signal depuis le serveur
            let signals = mail::tools::fetch_and_parse(&mut sess, risk_amount);
            for sig in signals {
                println!("[monitor] Signal détecté : {:?}", sig);

                println!("[monitor] Lancement trade sur binance..");
                // launch_trade(sig);
            }
        }
    });

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}