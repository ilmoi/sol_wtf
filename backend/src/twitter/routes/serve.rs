#![allow(clippy::async_yields_async)]

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;

use crate::twitter::model::media::{fetch_all_media_for_tweet, Media};
use crate::twitter::model::tweet::{fetch_next_page_of_tweets, fetch_tweet, Tweet};
use crate::twitter::model::user::{fetch_user_by_uuid, User};
use crate::utils::errors::ApiError;
use anyhow::Context;

// ----------------------------------------------------------------------------- structs/enums

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TweetParams {
    pub sort_by: SortBy,
    pub timeframe: Timeframe,
    pub last_tweet_id: String,
    pub last_metric: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Popularity,
    Retweets,
    Likes,
    Replies,
    Time,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Timeframe {
    Hour,
    Four,
    Day,
    Twodays,
    Week,
    Month,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullTweet {
    pub tweet: Tweet,
    pub author: User,
    pub media: Option<Vec<Media>>,
    pub reply_to: Box<Option<FullTweet>>,
    pub quote_of: Box<Option<FullTweet>>,
}

// ----------------------------------------------------------------------------- traits

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortBy::Popularity => write!(f, "popularity_count"),
            SortBy::Retweets => write!(f, "total_retweet_count"),
            SortBy::Likes => write!(f, "like_count"),
            SortBy::Replies => write!(f, "reply_count"),
            SortBy::Time => write!(f, "tweet_created_at"),
        }
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let now = Utc::now();
        match self {
            Timeframe::Hour => {
                let then = now - Duration::hours(1);
                write!(f, "{}", then.to_rfc3339())
            }
            Timeframe::Four => {
                let then = now - Duration::hours(4);
                write!(f, "{}", then.to_rfc3339())
            }
            Timeframe::Day => {
                let then = now - Duration::hours(24);
                write!(f, "{}", then.to_rfc3339())
            }
            Timeframe::Twodays => {
                let then = now - Duration::hours(48);
                write!(f, "{}", then.to_rfc3339())
            }
            Timeframe::Week => {
                let then = now - Duration::hours(24 * 7);
                write!(f, "{}", then.to_rfc3339())
            }
            Timeframe::Month => {
                let then = now - Duration::hours(24 * 30);
                write!(f, "{}", then.to_rfc3339())
            }
        }
    }
}

// ----------------------------------------------------------------------------- fns

#[tracing::instrument]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("health ok!")
}

#[tracing::instrument(skip(pool))]
#[get("/tweets")]
pub async fn serve_tweets(
    form: web::Query<TweetParams>,
    pool: web::Data<Arc<PgPool>>,
) -> Result<HttpResponse, ApiError> {
    let pool = pool.as_ref().deref();
    let tweets = fetch_next_page_of_tweets(pool, &form)
        .await
        .context("failed to fetch next page of tweets")?;

    let mut full_tweets: Vec<FullTweet> = vec![];

    for t in tweets.into_iter() {
        let mut full_tweet = prep_full_tweet(pool, t)
            .await
            .context("failed to prep full tweet")?;

        // tries to add a reply tweet, if present
        if let Some(ref reply_tweet_id) = full_tweet.tweet.replied_to_tweet_id {
            if let Ok(reply_tweet) = fetch_tweet(pool, reply_tweet_id).await {
                let reply_full_tweet = prep_full_tweet(pool, reply_tweet)
                    .await
                    .context("failed to prep full tweet")?;
                full_tweet.reply_to = Box::new(Some(reply_full_tweet));
            }
        }

        // tried to add a quote tweet, if present
        if let Some(ref quote_tweet_id) = full_tweet.tweet.quoted_tweet_id {
            if let Ok(quote_tweet) = fetch_tweet(pool, quote_tweet_id).await {
                let quote_full_tweet = prep_full_tweet(pool, quote_tweet)
                    .await
                    .context("failed to prep full tweet")?;
                full_tweet.quote_of = Box::new(Some(quote_full_tweet));
            }
        }

        full_tweets.push(full_tweet);
    }

    let body = serde_json::to_string(&full_tweets).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

#[tracing::instrument(skip(pool, tweet), level = "debug")]
pub async fn prep_full_tweet(pool: &PgPool, tweet: Tweet) -> Result<FullTweet, sqlx::error::Error> {
    let author = fetch_user_by_uuid(&pool, tweet.user_id).await?;
    let media = fetch_all_media_for_tweet(&pool, tweet.id).await?;

    Ok(FullTweet {
        tweet,
        author,
        media: Some(media),
        reply_to: Box::new(None::<FullTweet>),
        quote_of: Box::new(None::<FullTweet>),
    })
}
