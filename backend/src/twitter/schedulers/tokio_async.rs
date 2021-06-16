use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;

use sqlx::PgPool;
use tokio::time;

use crate::config::Environment;
use crate::config::Settings;
use crate::twitter::routes::pull::{
    backfill_missing_media_and_helper_tweets, pull_timelines_for_followed_users,
};

#[tracing::instrument(skip(pool, config))]
pub async fn schedule_tweet_refresh(pool: Arc<PgPool>, config: Arc<Settings>) {
    let app_env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("failed to determine App Environment.");

    // don't want to run the below when dev'ing
    if app_env == Environment::Dev {
        tracing::info!("no scheduler in dev.");
        return;
    }

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60 * 16));
        interval.tick().await;

        // all 3 calls are happening inside the same spawn, so are between themselves synchronous (like they should be)
        loop {
            tracing::info!(">>> START SCHEDULED TWEET REFRESH.");

            tracing::info!(">>> [0/2] Begin 16 min delay"); //intentionally upfront, otherwise on refreshes gets triggered and exhausts api
            interval.tick().await;

            tracing::info!(">>> [1/2] Pull timelines");
            pull_timelines_for_followed_users(pool.clone().as_ref(), config.clone().as_ref()).await;

            tracing::info!(">>> [2/2] Backfill media/helper tweets");
            backfill_missing_media_and_helper_tweets(
                pool.clone().as_ref(),
                config.clone().as_ref(),
            )
            .await;
        }
    });
}
