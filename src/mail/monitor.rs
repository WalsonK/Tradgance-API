use std::time::Duration;
use std::thread;
use imap::types::UnsolicitedResponse;
use crate::mail::tools::get_session;

pub fn surveiller_mail() {
    thread::spawn(|| {
        loop {
            let mut sess = get_session();

            if let Err(e) = sess.select("INBOX") {
                eprintln!("[monitor] select INBOX échoué: {}", e);
                thread::sleep(Duration::from_secs(30));
                continue;
            }

            // Boucle de surveillance avec IDLE
            loop {
                let mut idle = match sess.idle() {
                    Ok(idle) => idle,
                    Err(e) => {
                        eprintln!("[monitor] idle échoué: {}", e);
                        break;
                    }
                };

                // Timeout 25 min (Gmail limite à 30 min)
                idle.timeout(Duration::from_secs(25 * 60));

                let mut new_mail = false;
                println!("[monitor] Surveillance de mail en cours...");

                let _ = idle.wait_while(|response: UnsolicitedResponse| {
                    if let UnsolicitedResponse::Exists(n) = response {
                        println!("[monitor] Nouveau mail détecté (id: {})", n);
                        new_mail = true;
                        return false;
                    }
                    true
                });

                drop(idle);

                // Ici, on peut chercher les nouveaux mails TradingView
                if new_mail {
                    if let Ok(ids) = sess.search("UNSEEN") {
                        for id in ids.iter() {
                            if let Ok(fetches) = sess.fetch(id.to_string(), "RFC822") {
                                for f in fetches.iter() {
                                    if let Some(body) = f.body() {
                                        let mail = std::str::from_utf8(body).unwrap_or("");
                                        println!("[monitor] Mail reçu:\n{}", mail);

                                        // 👉 Ici tu peux parser ton JSON TradingView
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let _ = sess.logout();
            thread::sleep(Duration::from_secs(10));
        }
    });
}
