use imap::{ClientBuilder, Connection, Session};
use mailparse::parse_mail;

pub fn get_session() -> Session<Connection>{
    let client = ClientBuilder::new("imap.gmail.com", 993).connect().unwrap();

    let session = client
        .login(
            dotenv::var("EMAIL_ID").expect("Email must be set"),
            dotenv::var("EMAIL_PASS").expect("Email Password must be set")
        )
        .map_err(|e| e.0).unwrap();

    session
}

pub fn extract_body(raw_mail: &str) -> String {
    let parsed = parse_mail(raw_mail.as_bytes()).ok().unwrap();
    let body = parsed.subparts[0].get_body().unwrap();
    let clean = body.replace("\r\n", "");
    clean
}