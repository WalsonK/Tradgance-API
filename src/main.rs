use std::thread;
use axum::Router;
use axum::routing::get;
use dotenv::dotenv;

mod routes;
mod models;
mod mail;
mod tester;

#[tokio::main]
async fn main() {
    dotenv().ok();

    thread::spawn(|| {
        // 1. Component : récupération ou reconnexion au serveur
        let mut sess = mail::mailer::wait_online_session();

        loop {
            let mut idle = sess.idle();

            // 2. Component : attend détection via IDLE
            mail::monitor::wait_for_new_mail(&mut idle);
            drop(idle);

            // 3. Component : extraction d’un signal depuis le serveur
            let signals = mail::tools::fetch_and_parse(&mut sess);
            for sig in signals {
                println!("[monitor] Signal détecté : {:?}", sig);

                // launch_trade(sig);
            }
        }
    });

    // routes
    let app = Router::new()
        .route("/", get(routes::hello::hello));

    // run app
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}