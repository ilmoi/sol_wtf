use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use tracing::log::LevelFilter::Debug;
use tracing_log::LogTracer;

use backend::config::get_config;
use backend::startup::run_server;
use backend::twitter::schedulers::tokio_async::schedule_tweet_refresh;
use backend::utils::tracing::configure_tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ----------------------------------------------------------------------------- tracing & logging
    // configure tracing subscriber
    configure_tracing();

    // log http events from actix
    LogTracer::init().expect("failed to enable http request logging");

    // ----------------------------------------------------------------------------- config & pg
    let config = get_config().expect("failed to read settings");
    let addr = format!("{}:{}", config.app.host, config.app.port);

    // configure sqlx connection
    let mut conn_options = config.database.conn_opts();
    // the line below makes sqlx logs appear as debug, not as info
    let conn_options_w_logging = conn_options.log_statements(Debug); //must be a separate var

    // get a connection pool
    let pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(60)) //on purpose setting longer to avoid sqlx PoolTimedOut
        .connect_with(conn_options_w_logging.to_owned())
        .await
        .expect("failed to connect to Postgres");

    // ----------------------------------------------------------------------------- run
    let arc_pool = Arc::new(pg_pool);
    let arc_config = Arc::new(config);

    schedule_tweet_refresh(arc_pool.clone(), arc_config.clone()).await;
    run_server(&addr, arc_pool.clone(), arc_config.clone())?.await
}
