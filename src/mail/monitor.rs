use std::time::Duration;
use imap::{Connection};
use imap::extensions::idle::Handle;
use imap::types::UnsolicitedResponse;

/// Bloque jusqu’à ce qu’un nouveau message soit détecté ou le timeout atteint
pub fn wait_for_new_mail(idle: &mut Handle<Connection>) {
    idle.timeout(Duration::from_secs(25 * 60));
    println!("[monitor] Surveillance en cours...");
    let _ = idle.wait_while(|resp| match resp {
        UnsolicitedResponse::Exists(_) => false, // On s’arrête
        _ => true,
    });
}
