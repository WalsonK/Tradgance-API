use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logger() {
    // Appender fichier (rotation quotidienne)
    let file_appender = rolling::daily("logs", "api.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Subscriber pour console
    let console_layer = fmt::layer()
        .with_target(false)
        .with_level(true);

    // Subscriber pour fichier (JSON structuré)
    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(false)
        .with_level(true);

    // Combine console + fichier
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::from_default_env()) // support RUST_LOG=debug cargo run
        .init();

    // IMPORTANT : garder guard vivant pour flush auto
    std::mem::forget(guard);
}