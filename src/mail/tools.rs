use imap::{ClientBuilder, Connection, Session};

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