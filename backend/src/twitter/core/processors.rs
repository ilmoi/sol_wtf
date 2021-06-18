use serde_json::Value;
use sqlx::PgPool;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;

use crate::config::Settings;
use crate::twitter::model::media::handle_media_for_tweet;
use crate::twitter::model::tweet::{store_tweet, Tweet};
use crate::twitter::model::user::store_user;
use crate::twitter::scrapers::specific::{get_single_tweet, get_user_timeline};
use crate::utils::constants::{RETRY_BASE, RETRY_COUNT_NORMAL, RETRY_FACTOR};
use anyhow::Context;

#[tracing::instrument(skip(config, pool, user_object))]
pub async fn process_user_timeline(
    config: &Settings,
    pool: &PgPool,
    user_object: &Value,
) -> anyhow::Result<()> {
    // get timeline, retrying 2 times (5s and 25s)
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_NORMAL);
    let (user_timeline, _) = Retry::spawn(retry_strategy, || async {
        get_user_timeline(
            config,
            user_object["id"].as_str().ok_or(anyhow::anyhow!("no id"))?,
        )
        .await
    })
    .await
    .context(format!(
        "failed to fetch user timeline after {} retries",
        RETRY_COUNT_NORMAL
    ))?;

    // 1 store users (must go first)
    if let Some(users) = user_timeline["includes"]["users"].as_array() {
        for user in users.iter() {
            store_user(pool, &user)
                .await
                .context("failed to store user when processing timeline")?;
        }
    }

    // 2 store tweets (references users, so must go second)
    if let Some(tweets) = user_timeline["data"].as_array() {
        for tweet in tweets.iter() {
            store_tweet(pool, &tweet, &user_timeline, "normal")
                .await
                .context("failed to store tweet when processing timeline")?;
        }
    }

    // 3 store helper tweets (least important - goes last)
    if let Some(helper_tweets) = user_timeline["includes"]["tweets"].as_array() {
        for ht in helper_tweets.iter() {
            store_tweet(pool, &ht, &user_timeline, "helper")
                .await
                .context("failed to store helper tweet when processing timeline")?;
        }
    }
    Ok(())
}

#[tracing::instrument(skip(config, pool, rt_original))]
pub async fn process_rt_original_tweet(
    config: &Settings,
    pool: &PgPool,
    rt_original: &Tweet,
) -> anyhow::Result<()> {
    // get the original retweet, retrying 2 times (5s and 25s)
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_NORMAL);
    let (tweet_body, _) = Retry::spawn(retry_strategy, || async {
        get_single_tweet(config, &rt_original.tweet_id).await
    })
    .await
    .context(format!(
        "failed to fetch rt_original tweet after {} retries",
        RETRY_COUNT_NORMAL
    ))?;

    // 1 save its media
    handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
        .await
        .context("failed to handle media for rt_original tweet")?;
    // 2 save its helper tweets
    // 2.1 first save users
    if let Some(users) = &tweet_body["includes"]["users"].as_array() {
        for user in users.iter() {
            store_user(pool, &user)
                .await
                .context("failed to store user when processing rt_original tweet")?;
        }
    }
    // 2.2 then actual helper tweets
    if let Some(helper_tweets) = tweet_body["includes"]["tweets"].as_array() {
        for ht in helper_tweets.iter() {
            store_tweet(pool, &ht, &tweet_body, "helper")
                .await
                .context("failed to store helper tweet when processing rt_original tweet")?;
        }
    }
    Ok(())
}

#[tracing::instrument(skip(config, pool, helper))]
pub async fn process_helper_tweet(
    config: &Settings,
    pool: &PgPool,
    helper: &Tweet,
) -> anyhow::Result<()> {
    // get the helper retweet, retrying 2 times (5s and 25s)
    let retry_strategy = ExponentialBackoff::from_millis(RETRY_BASE)
        .factor(RETRY_FACTOR)
        .take(RETRY_COUNT_NORMAL);
    let (tweet_body, _) = Retry::spawn(retry_strategy, || async {
        get_single_tweet(config, &helper.tweet_id).await
    })
    .await
    .context(format!(
        "failed to fetch helper tweet after {} retries",
        RETRY_COUNT_NORMAL
    ))?;

    // save its media
    handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
        .await
        .context("failed to handle media for helper tweet")?;
    Ok(())
}
