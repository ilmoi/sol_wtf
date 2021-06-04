use std::ops::Deref;
use std::sync::Arc;

use crate::config::Settings;
use crate::twitter::domain::tweet::store_tweet;
use crate::twitter::domain::user::store_user;
use crate::twitter::scrapers::v2_api::get_user_timeline;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use sqlx::PgPool;

#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();
    let body = get_user_timeline(config).await.unwrap();

    // store all users first (always at least 1 user (author) present, so .unwrap ok)
    let users = &body["includes"]["users"];
    for user in users.as_array().unwrap() {
        store_user(pool.as_ref(), &user).await.unwrap();
    }

    // then store tweets (references users, so must go second) (always present, so .unwrap ok)
    let tweets = &body["data"];
    for tweet in tweets.as_array().unwrap() {
        store_tweet(pool.as_ref(), &tweet, &body, "normal")
            .await
            .unwrap();
    }

    // finally store helper tweets (may/not be present)
    let helper_tweets = &body["includes"]["tweets"].as_array();
    if let Some(helper_ts) = helper_tweets {
        for ht in helper_ts.iter() {
            store_tweet(pool.as_ref(), &ht, &body, "helper")
                .await
                .unwrap();
        }
    }

    // for rt_original tweets fetch 1)media, 2)helper tweets (sorted by popularity)

    // for all helper tweets fetch media (sorted by popularity)

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
