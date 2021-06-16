use crate::config::Environment;
use crate::config::Settings;
use crate::twitter::routes::pull::{
    backfill_missing_media_and_helper_tweets, pull_timelines_for_followed_users,
};
use sqlx::PgPool;
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

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
        let mut interval = time::interval(Duration::from_secs(60 * 16)); //todo change for testing

        // all 3 calls are happening inside the same spawn, so are between themselves synchronous (like they should be)
        let mut first_run = true;
        loop {
            // needed as otherwise it executes immediately on deploy, which can lead to api rate limit problems
            if first_run {
                first_run = false;
                interval.tick().await;
                continue;
            }
            tracing::info!(">>> START SCHEDULED TWEET REFRESH.");

            tracing::info!(">>> [1/3] Begin 16 min delay"); //intentionally upfront, otherwise on refreshes gets triggered and exhausts api
            interval.tick().await;

            tracing::info!(">>> [2/3] Pull timelines");
            pull_timelines_for_followed_users(pool.clone().as_ref(), config.clone().as_ref()).await;

            tracing::info!(">>> [3/3] Backfill media/helper tweets");
            backfill_missing_media_and_helper_tweets(
                pool.clone().as_ref(),
                config.clone().as_ref(),
            )
            .await;
        }
    });
}
