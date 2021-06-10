use sqlx::ConnectOptions;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use backend::config::get_config;
use backend::startup::run;
use sqlx::postgres::PgPoolOptions;
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
    let addr = format!("{}:{}", config.app.host, config.app.port);

    // configure sqlx connection
    let mut conn_options = config.database.conn_opts();
    let conn_options_w_logging = conn_options.log_statements(Debug); //must be a separate var

    // get a connection pool
    let pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2)) //setting shorter timeout so we find out if db is dead quicker
        .connect_with(conn_options_w_logging.to_owned())
        .await
        .expect("failed to connect to Postgres");

    run(&addr, pg_pool, config)?.await
}
