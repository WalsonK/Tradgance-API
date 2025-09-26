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