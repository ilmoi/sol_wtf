use sqlx::{ConnectOptions, PgPool};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use backend::config::get_config;
use backend::startup::run;
use sqlx::postgres::PgConnectOptions;
use tracing::log::LevelFilter::Debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // configure tracing subscriber
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")); //default to info
    let formatting_layer = BunyanFormattingLayer::new("backend".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("failed to set subscriber");

    // log http events from actix
    LogTracer::init().expect("failed to enable http request logging");

    let config = get_config().expect("failed to read settings");
    let addr = format!("127.0.0.1:{}", config.app_port);

    // configure sqlx connection
    let mut conn_options = PgConnectOptions::new()
        .host(&config.database.host)
        .port(config.database.port)
        .username(&config.database.username)
        .password(&config.database.password)
        .database(&config.database.db_name);

    // configure sqlx logging to run at debug level, not info (default)
    let conn_options_w_logging = conn_options.log_statements(Debug);

    // get a connection pool
    let pg_pool = PgPool::connect_with(conn_options_w_logging.to_owned())
        .await
        .expect("failed to connect to pg");

    run(&addr, pg_pool, config)?.await
}
