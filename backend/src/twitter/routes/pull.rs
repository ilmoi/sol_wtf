use std::ops::Deref;
use std::sync::Arc;

use crate::config::Settings;
use crate::twitter::domain::tweet::{fetch_tweets_to_backfill_by_class, store_tweet};
use crate::twitter::domain::user::store_user;
use crate::twitter::scrapers::v2_api::{fetch_followed_users, get_single_tweet, get_user_timeline};
use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use crate::twitter::domain::media::handle_media_for_tweet;

use sqlx::PgPool;

/// todo need some form of handling for when the twitter api returns non 200 (error handling)
/// todo build in controls for rate limits
#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    let users = fetch_followed_users(&config).await.unwrap();
    //todo 5 for testing
    for user_object in &users["data"].as_array().unwrap()[..5] {
        // get timeline
        let user_timeline = get_user_timeline(config, user_object["id"].as_str().unwrap())
            .await
            .unwrap();

        // store all users first (always at least 1 user (author) present, so .unwrap ok)
        for user in user_timeline["includes"]["users"]
            .as_array()
            .unwrap()
            .iter()
        {
            store_user(pool.as_ref(), &user).await.unwrap();
        }

        // store tweets (references users, so must go second) (always present, so .unwrap ok)
        for tweet in user_timeline["data"].as_array().unwrap().iter() {
            store_tweet(pool.as_ref(), &tweet, &user_timeline, "normal")
                .await
                .unwrap();
        }

        // store helper tweets (may/not be present, so do if let Some)
        if let Some(helper_tweets) = user_timeline["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(pool.as_ref(), &ht, &user_timeline, "helper")
                    .await
                    .unwrap();
            }
        }
    }

    HttpResponse::Ok()
}

/// same as above endpoint
#[get("/backfill")]
pub async fn backfill(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();

    // for rt_original tweets fetch 1)media, 2)helper tweets (sorted by popularity)
    let rt_originals = fetch_tweets_to_backfill_by_class(pool.as_ref(), "rt_original")
        .await
        .unwrap();
    for rt_original in rt_originals.iter() {
        let tweet_body = get_single_tweet(config, &rt_original.tweet_id)
            .await
            .unwrap();

        // 1 media
        handle_media_for_tweet(&pool, &tweet_body["data"], &tweet_body)
            .await
            .unwrap();

        // 2 helper tweets
        // we first need to save the users
        let users = &tweet_body["includes"]["users"];
        for user in users.as_array().unwrap() {
            store_user(pool.as_ref(), &user).await.unwrap();
        }

        // only then the actual helper tweets
        if let Some(helper_tweets) = tweet_body["includes"]["tweets"].as_array() {
            for ht in helper_tweets.iter() {
                store_tweet(&pool, &ht, &tweet_body, "helper")
                    .await
                    .unwrap();
            }
        }
    }

    // for all helper tweets fetch media (sorted by popularity)
    let helpers = fetch_tweets_to_backfill_by_class(pool.as_ref(), "helper")
        .await
        .unwrap();
    for helper in helpers.iter() {
        let tweet_body = get_single_tweet(config, &helper.tweet_id).await.unwrap();
        handle_media_for_tweet(pool.as_ref(), &tweet_body["data"], &tweet_body)
            .await
            .unwrap();
    }

    HttpResponse::Ok()
}

// ----------------------------------------------------------------------------- helpers

// ----------------------------------------------------------------------------- errors

#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
}

impl ResponseError for MyError {}
