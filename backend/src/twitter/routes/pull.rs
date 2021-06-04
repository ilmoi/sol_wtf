use std::ops::Deref;
use std::sync::Arc;

use crate::config::Settings;
use crate::twitter::domain::tweet::{fetch_tweet, store_tweet, update_tweet};
use crate::twitter::domain::user::{fetch_user, store_user, update_user};
use crate::twitter::scrapers::v2_api::get_user_timeline;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use sqlx::PgPool;

#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>, config: web::Data<Arc<Settings>>) -> impl Responder {
    let config = config.as_ref().deref();
    let body = get_user_timeline(config).await;

    // store all users first
    let users = &body["includes"]["users"];
    for user in users.as_array().unwrap() {
        let found_user = fetch_user(pool.as_ref(), &user["id"].as_str().unwrap()).await;
        match found_user {
            Ok(_u) => update_user(pool.as_ref(), &user).await.unwrap(),
            Err(_e) => store_user(pool.as_ref(), &user).await.unwrap(),
        }
    }

    // then store tweets (references users, so must go second)
    let tweets = &body["data"];
    for tweet in tweets.as_array().unwrap() {
        let found_tweet = fetch_tweet(pool.as_ref(), &tweet["id"].as_str().unwrap()).await;
        match found_tweet {
            Ok(_t) => update_tweet(pool.as_ref(), &tweet).await.unwrap(),
            Err(_e) => store_tweet(pool.as_ref(), &tweet, &body).await.unwrap(),
        }
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
