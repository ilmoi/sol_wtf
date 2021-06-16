#![allow(clippy::async_yields_async)]

use crate::twitter::domain::media::{fetch_all_media_for_tweet, Media};
use crate::twitter::domain::tweet::{fetch_next_page_of_tweets, fetch_tweet, Tweet};
use crate::twitter::domain::user::{fetch_user_by_uuid, User};
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

// ----------------------------------------------------------------------------- structs & enums

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
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[tracing::instrument]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("health ok!")
}

// http://localhost:5001/tweets4?last_tweet_id=0&sort_by=popularity&timeframe=week
#[tracing::instrument(skip(pool))]
#[get("/tweets4")]
pub async fn tweets4(
    form: web::Query<TweetParams>,
    pool: web::Data<Arc<PgPool>>,
) -> impl Responder {
    let pool = pool.as_ref().deref();
    let tweets = fetch_next_page_of_tweets(pool, &form).await.unwrap();

    let mut full_tweets: Vec<FullTweet> = vec![];

    for t in tweets.into_iter() {
        let mut full_tweet = prep_full_tweet(pool, t).await;

        // tries to add a reply tweet, if present
        if let Some(ref reply_tweet_id) = full_tweet.tweet.replied_to_tweet_id {
            if let Ok(reply_tweet) = fetch_tweet(pool, reply_tweet_id).await {
                let reply_full_tweet = prep_full_tweet(pool, reply_tweet).await;
                full_tweet.reply_to = Box::new(Some(reply_full_tweet));
            }
        }

        // tried to add a quote tweet, if present
        if let Some(ref quote_tweet_id) = full_tweet.tweet.quoted_tweet_id {
            if let Ok(quote_tweet) = fetch_tweet(pool, quote_tweet_id).await {
                let quote_full_tweet = prep_full_tweet(pool, quote_tweet).await;
                full_tweet.quote_of = Box::new(Some(quote_full_tweet));
            }
        }

        full_tweets.push(full_tweet);
    }

    let body = serde_json::to_string(&full_tweets).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[tracing::instrument(skip(pool))]
pub async fn prep_full_tweet(pool: &PgPool, tweet: Tweet) -> FullTweet {
    let author = fetch_user_by_uuid(&pool, tweet.user_id).await.unwrap();
    let media = fetch_all_media_for_tweet(&pool, tweet.id).await.unwrap();

    FullTweet {
        tweet,
        author,
        media: Some(media),
        reply_to: Box::new(None::<FullTweet>),
        quote_of: Box::new(None::<FullTweet>),
    }
}

// ----------------------------------------------------------------------------- errors

#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
}

impl ResponseError for MyError {}
