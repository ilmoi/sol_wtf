use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;

use sqlx::PgPool;
use tokio::time;

use crate::config::Environment;
use crate::config::Settings;
use crate::twitter::core::jobs::{
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
        tracing::info!(">>>I: no scheduler in dev.");
        return;
    }

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60 * config.app.refresh_freq));
        interval.tick().await;

        // all 3 calls are happening inside the same spawn, so are between themselves synchronous (like they should be)
        loop {
            tracing::info!(">>>I: Begin scheduled tweet refresh.");

            //intentionally upfront, otherwise on refreshes gets triggered and exhausts api
            tracing::info!(">>>I: [0/2] Begin {} min delay", config.app.refresh_freq);
            interval.tick().await;

            // retry logic already inside
            tracing::info!(">>>I: [1/2] Pull timelines");
            pull_timelines_for_followed_users(pool.clone().as_ref(), config.clone().as_ref())
                .await
                .unwrap_or_else(|e| {
                    tracing::error!(">>>E: Failed to pull timelines for users: {}", e);
                });

            // retry logic already inside
            tracing::info!(">>>I: [2/2] Backfill media/helper tweets");
            backfill_missing_media_and_helper_tweets(
                pool.clone().as_ref(),
                config.clone().as_ref(),
            )
            .await
            .unwrap_or_else(|e| {
                tracing::error!(">>>E: Failed to backfill media/helper tweets: {}", e);
            });
        }
    });
}
