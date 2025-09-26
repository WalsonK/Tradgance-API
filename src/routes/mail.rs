use crate::binance;
use crate::mail;

pub async fn active_mail_surveillance() {
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
            let risk_amount = binance::tools::money_management(balances.1.to_string()).await.unwrap();
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
}