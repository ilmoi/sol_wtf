use std::ops::Deref;
use std::sync::Arc;

use crate::config::Settings;
use crate::twitter::domain::tweet::{
    backfill_helper_tweet, backfill_rt_original_tweet, find_tweets_to_backfill_by_class,
    store_tweet,
};
use crate::twitter::domain::user::store_user;
use crate::twitter::scrapers::v2_api::{fetch_followed_users, get_single_tweet, get_user_timeline};
use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use sqlx::PgPool;

// todo need some form of handling for when the twitter api returns non 200 (error handling)
#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();
    // let users = ["1398489893236576258", "2440399742"];

    let users = fetch_followed_users(&config).await.unwrap();

    for user_object in users["data"].as_array().unwrap() {
        // get timeline
        let user_timeline = get_user_timeline(config, user_object["id"].as_str().unwrap())
            .await
            .unwrap();

        // store all users first (always at least 1 user (author) present, so .unwrap ok)
        let users = &user_timeline["includes"]["users"];
        for user in users.as_array().unwrap() {
            store_user(pool.as_ref(), &user).await.unwrap();
        }

        // then store tweets (references users, so must go second) (always present, so .unwrap ok)
        let tweets = &user_timeline["data"];
        for tweet in tweets.as_array().unwrap() {
            store_tweet(pool.as_ref(), &tweet, &user_timeline, "normal")
                .await
                .unwrap();
        }

        // finally store helper tweets (may/not be present)
        let helper_tweets = &user_timeline["includes"]["tweets"].as_array();
        if let Some(helper_ts) = helper_tweets {
            for ht in helper_ts.iter() {
                store_tweet(pool.as_ref(), &ht, &user_timeline, "helper")
                    .await
                    .unwrap();
            }
        }
    }

    HttpResponse::Ok()
}

// todo build in controls for rate limits + prioritize queries better
#[get("/backfill")]
pub async fn backfill(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    // for rt_original tweets fetch 1)media, 2)helper tweets (sorted by popularity)
    // NOTE: we're limited to 900 calls/15min due to api rate limits
    let rt_originals = find_tweets_to_backfill_by_class(pool.as_ref(), "rt_original")
        .await
        .unwrap();
    for rt_original in rt_originals.iter() {
        let tweet_body = get_single_tweet(config, &rt_original.tweet_id)
            .await
            .unwrap();
        backfill_rt_original_tweet(pool.as_ref(), &tweet_body)
            .await
            .unwrap();
    }

    // for all helper tweets fetch media (sorted by popularity)
    let helpers = find_tweets_to_backfill_by_class(pool.as_ref(), "helper")
        .await
        .unwrap();
    for helper in helpers.iter() {
        let tweet_body = get_single_tweet(config, &helper.tweet_id).await.unwrap();
        backfill_helper_tweet(pool.as_ref(), &tweet_body)
            .await
            .unwrap();
    }

    HttpResponse::Ok()
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
