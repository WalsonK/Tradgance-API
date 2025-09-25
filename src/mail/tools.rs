use imap::{ClientBuilder, Connection, Session};
use mailparse::parse_mail;
use crate::models::trade::TradeSignal;

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

/// Récupère les messages UNSEEN et convertit chacun en `TradeSignal`
pub fn fetch_and_parse(s: &mut Session<Connection>, risk_amount: f64) -> Vec<TradeSignal> {
    let ids = match s.search("UNSEEN") {
        Ok(ids) => ids,
        Err(e) => {
            eprintln!("[monitor] search UNSEEN échoué : {}", e);
            return vec![];
        }
    };

    ids.iter()
        .filter_map(|id| {
            let fetches = s.fetch(id.to_string(), "RFC822").ok()?;
            let fetch = fetches.iter().next()?;
            let body = fetch.body()?;
            let mail_raw = std::str::from_utf8(body).ok()?;
            let mail = extract_body(mail_raw);
            println!("[monitor] Corps du mail reçu :\n{:?}", mail);
            TradeSignal::from_str(mail, risk_amount)
        })
        .collect()
}