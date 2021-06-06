use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::config::Settings;
use crate::twitter::domain::tweet::{fetch_tweets_to_backfill_by_class, store_tweet, Tweet};
use crate::twitter::domain::user::store_user;
use crate::twitter::scrapers::v2_api::{
    fetch_followed_users, get_single_tweet, get_user_timeline, RateLimits,
};
use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use crate::twitter::domain::media::handle_media_for_tweet;

use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use std::future::Future;
use std::thread;

#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    let (users, _) = fetch_followed_users(&config).await.unwrap(); // todo currently only pulls top 100
    let selected_users = &users["data"].as_array().unwrap()[..]; //todo testing
    loop_until_hit_rate_limit(selected_users, config, pool.as_ref(), process_user_timeline).await;

    HttpResponse::Ok()
}

/// todo fix rate limts - i only have 900 for 150k tweets
#[get("/backfill")]
pub async fn backfill(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    // 1) process rt_original tweets (download media + helpers)
    let rt_originals = fetch_tweets_to_backfill_by_class(pool.as_ref(), "rt_original")
        .await
        .unwrap();
    loop_until_hit_rate_limit(
        &rt_originals,
        config,
        pool.as_ref(),
        process_rt_original_tweet,
    )
    .await;

    // 2) process helper tweets (download media only)
    let helpers = fetch_tweets_to_backfill_by_class(pool.as_ref(), "helper")
        .await
        .unwrap();
    loop_until_hit_rate_limit(&helpers, config, pool.as_ref(), process_helper_tweet).await;

    HttpResponse::Ok()
}

#[get("/exhaust")]
pub async fn exhaust(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();
    let packed_config = Arc::new(config);

    // for _ in 1..16 {
    //     let handle = thread::spawn(move || {
    //         let inner_config = Arc::clone(&packed_config);
    //         get_single_tweet(inner_config.as_ref().deref(), "1401287393228038149");
    //     });
    // }

    HttpResponse::Ok()
}

// ----------------------------------------------------------------------------- helpers

// todo less than ideal - rework
// - need to fetch the limits before the 1st call, dummy of 1 causes problems
// - return vs continue?
pub async fn loop_until_hit_rate_limit<'a, T, Fut>(
    object_arr: &'a [T],
    settings: &'a Settings,
    pool: &'a PgPool,
    f: impl Fn(&'a Settings, &'a PgPool, &'a T) -> Fut,
) where
    Fut: Future<Output = Option<RateLimits>>,
{
    // start off with dummy limits
    let mut current_rate_limits = RateLimits {
        limit_left: 1,
        limit_total: 1,
        reset_time: Utc::now(),
    };

    let total = object_arr.len();

    for (i, object) in object_arr.iter().enumerate() {
        println!(">>> PROCESSING {}/{}", i + 1, total);
        println!(">>> {}", current_rate_limits);

        // test rate limits
        if current_rate_limits.limit_left <= 0 {
            println!(
                ">>> RATE LIMIT EXHAUSTED, EXITING. LIMIT STATUS: {}.",
                current_rate_limits
            );
            continue;
        }

        // issue next call & store rate limits
        if let Some(new_rate_limits) = f(settings, pool, object).await {
            current_rate_limits = new_rate_limits
        }
    }
}

pub async fn process_user_timeline(
    config: &Settings,
    pool: &PgPool,
    user_object: &Value,
) -> Option<RateLimits> {
    // get timeline
    if let Ok((user_timeline, rate_limits)) =
        get_user_timeline(config, user_object["id"].as_str().unwrap()).await
    {
        // store users (must go first)
        if let Some(users) = user_timeline["includes"]["users"].as_array() {
            for user in users.iter() {
                store_user(pool, &user).await.unwrap();
            }
        }

        // store tweets (references users, so must go second)
        if let Some(tweets) = user_timeline["data"].as_array() {
            for tweet in tweets.iter() {
                store_tweet(pool, &tweet, &user_timeline, "normal")
                    .await
                    .unwrap();
            }
        }

        // store helper tweets (least important - goes last)
        if let Some(helper_tweets) = user_timeline["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(pool, &ht, &user_timeline, "helper")
                    .await
                    .unwrap();
            }
        }
        return Some(rate_limits);
    }
    None
}

pub async fn process_rt_original_tweet(
    config: &Settings,
    pool: &PgPool,
    rt_original: &Tweet,
) -> Option<RateLimits> {
    if let Ok((tweet_body, rate_limit)) = get_single_tweet(config, &rt_original.tweet_id).await {
        // 1 media
        handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
            .await
            .unwrap();

        // 2 helper tweets
        // we first need to save the users
        if let Some(users) = &tweet_body["includes"]["users"].as_array() {
            for user in users.iter() {
                store_user(pool, &user).await.unwrap();
            }
        }
        // only then the actual helper tweets
        if let Some(helper_tweets) = tweet_body["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(pool, &ht, &tweet_body, "helper").await.unwrap();
            }
        }
        return Some(rate_limit);
    }
    None
}

pub async fn process_helper_tweet(
    config: &Settings,
    pool: &PgPool,
    helper: &Tweet,
) -> Option<RateLimits> {
    if let Ok((tweet_body, rate_limits)) = get_single_tweet(config, &helper.tweet_id).await {
        handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
            .await
            .unwrap();
        return Some(rate_limits);
    }
    None
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
