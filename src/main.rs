use std::net::TcpListener;
use sqlx::{PgPool};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use first::configuration::get_configuration;
use first::startup::run;

//#[actix_web::main] // or
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // tracing configuration
    LogTracer::init().expect("Failed to start LogTRacer!");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer =  BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout
    );

    let registry = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(registry).expect("Can't set subscriber");

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;
    let connection = PgPool::connect(&configuration.database.to_string())
        .await
        .expect("Can't connect to Postgres");
    run(listener, connection)?.await
}
