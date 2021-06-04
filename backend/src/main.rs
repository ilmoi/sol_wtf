use backend::config::get_config;
use backend::startup::run;
use env_logger::Env;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = get_config().expect("failed to read settings");
    let addr = format!("127.0.0.1:{}", config.app_port);

    let pg_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to pg");

    run(&addr, pg_pool, config)?.await
}
