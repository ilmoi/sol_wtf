use sqlx::PgPool;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;

use crate::config::Settings;
use crate::twitter::core::loops::loop_until_hit_rate_limit;
use crate::twitter::core::processors::{
    process_helper_tweet, process_rt_original_tweet, process_user_timeline,
};
use crate::twitter::model::tweet::{
    fetch_core_tweets_to_backfill, fetch_helper_tweets_to_backfill,
};
use crate::twitter::scrapers::specific::fetch_all_followed_users;
use crate::utils::constants::{RETRY_BASE, RETRY_COUNT_IMPORTANT, RETRY_FACTOR};
use anyhow::Context;
use std::cmp::min;

#[tracing::instrument(skip(pool, config))]
pub async fn pull_timelines_for_followed_users(
    pool: &PgPool,
    config: &Settings,
) -> anyhow::Result<()> {
    // factor = to turn milliseconds into seconds
    // base = what gets put to the power on each iteration
    // take 1 means original iteration plus one more. 3 takes of 5 mean: now > 5s > 25s > 125s
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_IMPORTANT);
    let (users, _) = Retry::spawn(retry_strategy, || async {
        fetch_all_followed_users(config).await
    })
    .await
    .context(format!(
        "failed to fetch followed users after {} retries",
        RETRY_COUNT_IMPORTANT
    ))?;

    let users = &users[..min(config.app.max_users, users.len())];
    //the below is fallible, but it won't let me propagate error up, so handled inside of loop (only logging, no retries)
    loop_until_hit_rate_limit(&users, config, pool, process_user_timeline, 1500).await;

    tracing::info!(">>>I: total processed user timelines: {}", users.len());
    Ok(())
}

/// Algo:
/// 1) take rt_originals in the last 24h ordered by popularity = the ones most likely to appear at the top of the feed
///     1.1) backfill media + helper tweets for them
/// 2) take helper tweets in the last 24h ordered by popularity
///     2.1) backfill media for them
///
/// Capacity calc:
/// - Twitter gives me 900 calls / 15min
/// - With 130 people followed and 13k tweets pulled, I have to backfill around 600 tweets for 7d / 85 for 1d.
/// - With 1500 people followed and 150k tweets pulled, this becomes 6900 for 7d and 977.5 for 24h.
/// - BUT: since we never have to backfill a tweet twice, and we'll be calling this func every 15min, the amount will go down over time.
/// - In other words it should be safe to set days_back to 7.
#[tracing::instrument(skip(pool, config))]
pub async fn backfill_missing_media_and_helper_tweets(
    pool: &PgPool,
    config: &Settings,
) -> anyhow::Result<()> {
    // 1) process core (normal + rt_orinals) tweets (download media + helpers)
    // sometimes a tweet will be deleted (eg 1401933150012559361) - and we keep trying to backfill it. In theory should handle - but for now I'll just let it drop out of timeframe.
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_IMPORTANT);
    let core = Retry::spawn(retry_strategy, || async {
        fetch_core_tweets_to_backfill(pool, 7).await
    })
    .await
    .context(format!(
        "failed to fetch core tweets to backfill after {} retries",
        RETRY_COUNT_IMPORTANT
    ))?;
    //the below is fallible, but it won't let me propagate error up, so handled inside of loop (only logging, no retries)
    loop_until_hit_rate_limit(
        &core,
        config,
        pool,
        process_rt_original_tweet,
        900, //in theory can ask twitter for remaining
    )
    .await;

    // 2) process helper tweets (download media only)
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_IMPORTANT);
    let helpers = Retry::spawn(retry_strategy, || async {
        fetch_helper_tweets_to_backfill(pool, 7).await
    })
    .await
    .context(format!(
        "failed to fetch helper tweets to backfill after {} retries",
        RETRY_COUNT_IMPORTANT
    ))?;
    //the below is fallible, but it won't let me propagate error up, so handled inside of loop (only logging, no retries)
    loop_until_hit_rate_limit(&helpers, config, pool, process_helper_tweet, 900).await;

    tracing::info!(
        ">>>I: Total executed: {} core and {} helpers",
        core.len(),
        helpers.len(),
    );
    Ok(())
}

// ----------------------------------------------------------------------------- keeping for personal ref - sync fn w retry crate
// let users = retry_with_index(Fixed::from_millis(10000), |current_try| {
//     if current_try > 3 {
//         return OperationResult::Err("failed after 3 attempts");
//     }
//
//     match fetch_all_followed_users(config).await {
//         Ok((users, _)) => {
//             tracing::info!(">>>I: successfully fetched followed users.");
//             OperationResult::Ok(users)
//         }
//         Err(_) => {
//             tracing::info!(">>>I: failed attempt at fetching followed users, trying again.");
//             OperationResult::Retry("retrying")
//         }
//     }
// })
// .map_err(|e| anyhow::anyhow!("{}", e))?;
