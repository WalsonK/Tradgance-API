use std::thread;
use std::time::Duration;
use imap::{Connection, Session};
use crate::mail::tools::get_session;

/*pub async fn get_all_email() {
    let mut session = get_session();
    session.select("INBOX").unwrap();

    // get all email
    let messages = session.fetch("1:*", "RFC822").unwrap();
    if let Some(last) = messages.iter().last() {
        if let Some(body) = last.body() {
            println!("Dernier message: {}", String::from_utf8_lossy(body));
        }
    } else { println!("No messages"); }

    session.logout().unwrap();
}*/
/* IMAP AUTO & Trade
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
});*/

/// Retourne toujours une session valide (reconnecte en cas d’échec)
pub fn wait_online_session() -> Session<Connection> {
    loop {
        let mut sess = get_session();
        match sess.select("INBOX") {
            Ok(_) => return sess,
            Err(e) => {
                eprintln!("[monitor] select INBOX échoué : {}", e);
                thread::sleep(Duration::from_secs(30));
            }
        }
    }
}