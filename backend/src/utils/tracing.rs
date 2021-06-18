use std::convert::TryInto;

use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::config::Environment;

pub fn configure_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    // luca's subscriber (as per book)
    // let formatting_layer = BunyanFormattingLayer::new("backend".into(), std::io::stdout);
    // let subscriber = Registry::default()
    //     .with(env_filter)
    //     .with(JsonStorageLayer)
    //     .with(formatting_layer);

    // todo testing: dev = debug + no spans / prod = info + spans

    // official subscriber
    // https://docs.rs/tracing-subscriber/0.2.18/tracing_subscriber/fmt/struct.SubscriberBuilder.html
    let prod_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // inimum level that will be included in output. This OR env_filter should be used to control level of logs
        // .with_timer(ChronoLocal::rfc3339()) //shows by default with better format
        .with_thread_ids(true)
        .with_span_events(FmtSpan::FULL)
        .json()
        .finish();

    let dev_subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter) // env_filter let's me use RUST_LOG from terminal to control level of logs
        .with_thread_ids(true)
        .pretty()
        .finish();

    let app_env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("failed to determine App Environment.");

    match app_env {
        Environment::Dev => set_global_default(dev_subscriber).expect("failed to set subscriber"),
        Environment::Prod => set_global_default(prod_subscriber).expect("failed to set subscriber"),
    };
}
