use crate::mail::tools::get_session;
pub async fn get_all_email() {
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
}