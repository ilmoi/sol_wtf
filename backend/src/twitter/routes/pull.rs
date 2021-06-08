use std::cmp::min;
use std::future::Future;
use std::ops::Deref;
use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use serde_json::Value;
use sqlx::PgPool;

use crate::config::Settings;
use crate::twitter::domain::media::handle_media_for_tweet;
use crate::twitter::domain::tweet::{fetch_tweets_to_backfill_by_class, store_tweet, Tweet};
use crate::twitter::domain::user::store_user;
use crate::twitter::scrapers::v2_api::{
    fetch_all_followed_users, get_single_tweet, get_user_timeline,
};

#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    let (users, _) = fetch_all_followed_users(&config).await.unwrap();
    let users = &users[..]; //todo change for testing

    loop_until_hit_rate_limit(&users, config, pool.as_ref(), process_user_timeline, 1500).await;
    // loop_until_hit_rate_limit_sync(&users, config, pool.as_ref(), process_user_timeline, 1500).await;

    HttpResponse::Ok()
}

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
        900, //todo in theory can ask twitter for remaining but for now good enough
    )
    .await;

    // 2) process helper tweets (download media only)
    let helpers = fetch_tweets_to_backfill_by_class(pool.as_ref(), "helper")
        .await
        .unwrap();
    loop_until_hit_rate_limit(&helpers, config, pool.as_ref(), process_helper_tweet, 900).await;

    println!(
        "Total: {} rt_originals and {} helpers",
        rt_originals.len(),
        helpers.len()
    );

    HttpResponse::Ok()
}

// ----------------------------------------------------------------------------- helpers

// todo https://stackoverflow.com/questions/67876855/handling-duplicate-inserts-into-database-in-async-rust
pub async fn loop_until_hit_rate_limit<'a, T, Fut>(
    object_arr: &'a [T],
    settings: &'a Settings,
    pool: &'a PgPool,
    f: impl Fn(&'a Settings, &'a PgPool, &'a T) -> Fut + Copy,
    rate_limit: usize,
) where
    Fut: Future,
{
    // this is the easiest way to impl. rate limits.
    // A much harder approach would be to wrap one in Arc(Mutex()) and update from each async task.
    // todo currently not retrying the ones that run over the limit
    let total = object_arr.len();
    let capped_total = min(total, rate_limit);

    let mut futs = vec![];
    for (i, object) in object_arr[..capped_total].iter().enumerate() {
        futs.push(async move {
            println!(">>> PROCESSING {}/{}", i + 1, total);
            f(settings, pool, object).await;
        });
    }
    futures::future::join_all(futs).await;
}

pub async fn loop_until_hit_rate_limit_sync<'a, T, Fut>(
    object_arr: &'a [T],
    settings: &'a Settings,
    pool: &'a PgPool,
    f: impl Fn(&'a Settings, &'a PgPool, &'a T) -> Fut + Copy,
    rate_limit: usize,
) where
    Fut: Future,
{
    let total = object_arr.len();
    let capped_total = min(total, rate_limit);
    for (i, object) in object_arr[..capped_total].iter().enumerate() {
        println!(">>> PROCESSING {}/{}", i + 1, total);
        f(settings, pool, object).await;
    }
}

pub async fn process_user_timeline(config: &Settings, pool: &PgPool, user_object: &Value) {
    // get timeline
    if let Ok((user_timeline, _)) =
        get_user_timeline(config, user_object["id"].as_str().unwrap()).await
    {
        // store users (must go first)
        if let Some(users) = user_timeline["includes"]["users"].as_array() {
            for user in users.iter() {
                store_user(pool, &user).await.unwrap_or_else(|e| {
                    println!(
                        ">>>X>>> failed to store user {}: {:?}",
                        user["id"].as_str().unwrap(),
                        e
                    )
                });
            }
        }

        // store tweets (references users, so must go second)
        if let Some(tweets) = user_timeline["data"].as_array() {
            for tweet in tweets.iter() {
                store_tweet(pool, &tweet, &user_timeline, "normal")
                    .await
                    .unwrap_or_else(|e| {
                        println!(
                            ">>>X>>> failed to store tweet {}: {:?}",
                            tweet["id"].as_str().unwrap(),
                            e
                        )
                    });
            }
        }

        // store helper tweets (least important - goes last)
        if let Some(helper_tweets) = user_timeline["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(pool, &ht, &user_timeline, "helper")
                    .await
                    .unwrap_or_else(|e| {
                        println!(
                            ">>>X>>> failed to store helper tweet {}: {:?}",
                            ht["id"].as_str().unwrap(),
                            e
                        )
                    });
            }
        }
    }
}

pub async fn process_rt_original_tweet(config: &Settings, pool: &PgPool, rt_original: &Tweet) {
    if let Ok((tweet_body, _)) = get_single_tweet(config, &rt_original.tweet_id).await {
        // 1 media
        handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
            .await
            .unwrap_or_else(|e| {
                println!(
                    ">>>X>>> failed to store media for tweet {}: {:?}",
                    tweet_body["data"]["id"].as_str().unwrap(),
                    e
                )
            });

        // 2 helper tweets
        // we first need to save the users
        if let Some(users) = &tweet_body["includes"]["users"].as_array() {
            for user in users.iter() {
                store_user(pool, &user).await.unwrap_or_else(|e| {
                    println!(
                        ">>>X>>> failed to store user {}: {:?}",
                        user["id"].as_str().unwrap(),
                        e
                    )
                });
            }
        }
        // only then the actual helper tweets
        if let Some(helper_tweets) = tweet_body["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(pool, &ht, &tweet_body, "helper")
                    .await
                    .unwrap_or_else(|e| {
                        println!(
                            ">>>X>>> failed to store helper tweet {}: {:?}",
                            ht["id"].as_str().unwrap(),
                            e
                        )
                    });
            }
        }
    }
}

pub async fn process_helper_tweet(config: &Settings, pool: &PgPool, helper: &Tweet) {
    if let Ok((tweet_body, _)) = get_single_tweet(config, &helper.tweet_id).await {
        handle_media_for_tweet(pool, &tweet_body["data"], &tweet_body)
            .await
            .unwrap_or_else(|e| {
                println!(
                    ">>>X>>> failed to store media for helper tweet {}: {:?}",
                    tweet_body["data"]["id"].as_str().unwrap(),
                    e
                )
            });
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
