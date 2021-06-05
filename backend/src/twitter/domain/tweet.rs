use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::twitter::domain::media::{handle_media_for_tweet, Media};
use crate::twitter::domain::user::{fetch_user, store_user, User};
use crate::twitter::routes::serve::TweetParams;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub tweet_id: String,
    pub tweet_created_at: DateTime<Utc>,
    pub tweet_text: String,
    pub tweet_url: String,
    // reference tweets
    pub replied_to_tweet_id: Option<String>,
    pub quoted_tweet_id: Option<String>,
    pub tweet_class: String,
    // pub is_reply: Option<bool>,
    // pub is_quote: Option<bool>,
    // pub retweeted_tweet_id: Option<String>,
    // pub is_retweet: Option<bool>,
    // metrics
    pub like_count: Option<i64>,
    pub quote_count: Option<i64>,
    pub reply_count: Option<i64>,
    pub retweet_count: Option<i64>,
    pub total_retweet_count: Option<i64>,
    pub popularity_count: Option<i64>,
    pub entire_tweet: Option<Value>,
    pub user_id: Uuid,
}

pub struct TweetMetrics {
    pub like_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
    pub total_retweet_count: i64,
    pub popularity_count: i64,
}

// todo switch to enums if can get it working - https://github.com/launchbadge/sqlx/issues/1004
//  for SELECT - https://github.com/launchbadge/sqlx/issues/1038
// #[allow(non_camel_case_types)]
// #[derive(Debug, sqlx::Type)]
// pub enum TweetClass {
//     normal,
//     rt_original,
//     helper,
// }

// ----------------------------------------------------------------------------- fn

pub async fn fetch_tweet(pool: &PgPool, tweet_id: &str) -> Result<Tweet, sqlx::error::Error> {
    let res = sqlx::query_as!(
        Tweet,
        r#"
        SELECT * FROM tweets WHERE tweet_id = $1
        "#,
        tweet_id,
    )
    .fetch_one(pool)
    .await?;

    println!("fetched tweet with id {}", tweet_id);
    Ok(res)
}

pub fn extract_tweet_metrics(tweet: &Value) -> TweetMetrics {
    let like_count = tweet["public_metrics"]["like_count"].as_i64().unwrap();
    let quote_count = tweet["public_metrics"]["quote_count"].as_i64().unwrap();
    let reply_count = tweet["public_metrics"]["reply_count"].as_i64().unwrap();
    let retweet_count = tweet["public_metrics"]["retweet_count"].as_i64().unwrap();
    let total_retweet_count = quote_count + retweet_count;
    let popularity_count = total_retweet_count + like_count + reply_count;

    TweetMetrics {
        like_count,
        quote_count,
        reply_count,
        retweet_count,
        total_retweet_count,
        popularity_count,
    }
}

// not ideal that we have to pass the body here, but I need the media array from "includes"
#[async_recursion]
pub async fn store_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value,
    tweet_class: &str,
) -> Result<(), sqlx::error::Error> {
    // check if tweet already exists - if so, update it
    let tweet_id = tweet["id"].as_str().unwrap();
    let found_tweet = fetch_tweet(&pool, tweet_id).await;
    if let Ok(_) = found_tweet {
        return update_tweet(&pool, &tweet).await;
    }

    let author_id = tweet["author_id"].as_str().unwrap();
    let author = fetch_user(&pool, author_id).await.unwrap();
    let tweet_url = format!("https://twitter.com/{}/status/{}", &author_id, &tweet_id);
    let tweet_created_at =
        DateTime::parse_from_rfc3339(tweet["created_at"].as_str().unwrap()).unwrap();

    // handle reference tweets
    let mut replied_to_tweet_id: Option<String> = None;
    let mut quoted_tweet_id: Option<String> = None;
    // let mut retweeted_tweet_id: Option<String> = None; // todo search for all mentions of this and clean

    let ref_tweets = tweet.get("referenced_tweets");
    if let Some(ref_tweets) = ref_tweets {
        for rt in ref_tweets.as_array().unwrap() {
            match rt["type"].as_str().unwrap() {
                "retweeted" => {
                    // retweeted_tweet_id = Some(rt["id"].as_str().unwrap().into())

                    // NOTE: returns from function, as we only care to store the original post
                    let retweet_id = rt["id"].as_str().unwrap();
                    println!(
                        "Retweet detected, storing original tweet instead (id: {})",
                        retweet_id
                    );

                    let retweet = body["includes"]["tweets"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter(|&t| t["id"].as_str().unwrap() == retweet_id)
                        .collect::<Vec<&Value>>(); //todo is there a better way (single value) than .collect here?
                    return store_tweet(&pool, &retweet[0], &body, "rt_original").await;
                }
                "replied_to" => replied_to_tweet_id = Some(rt["id"].as_str().unwrap().into()),
                "quoted" => quoted_tweet_id = Some(rt["id"].as_str().unwrap().into()),
                _ => println!("unrecognized referenced_tweet type"),
            }
        }
    }

    // handle metrics
    let tweet_metrics = extract_tweet_metrics(&tweet);

    // store the actual tweet
    sqlx::query!(
        r#"
        INSERT INTO tweets
        (id, created_at,
        tweet_id, tweet_created_at, tweet_text, tweet_url,
        replied_to_tweet_id, quoted_tweet_id, tweet_class, 
        like_count, quote_count, reply_count, retweet_count, total_retweet_count, popularity_count,
        user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        "#,
        Uuid::new_v4(),
        Utc::now(),
        tweet_id,
        tweet_created_at,
        tweet["text"].as_str(),
        tweet_url,
        replied_to_tweet_id,
        quoted_tweet_id,
        tweet_class,
        // retweeted_tweet_id,
        // false,
        // false,
        // false,
        tweet_metrics.like_count,
        tweet_metrics.quote_count,
        tweet_metrics.reply_count,
        tweet_metrics.retweet_count,
        tweet_metrics.total_retweet_count,
        tweet_metrics.popularity_count,
        author.id,
    )
    .execute(pool)
    .await?;

    // handle media (IMPORTANT: must go after tweet itself, as references stored tweet id)
    handle_media_for_tweet(&pool, &tweet, &body).await?;

    println!("stored tweet with id {}", tweet["id"].as_str().unwrap());
    Ok(())
}

pub async fn update_tweet(pool: &PgPool, tweet: &Value) -> Result<(), sqlx::error::Error> {
    let tweet_metrics = extract_tweet_metrics(&tweet);
    sqlx::query!(
        r#"
        UPDATE tweets SET
        like_count = $1,
        quote_count = $2,
        reply_count = $3,
        retweet_count = $4,
        total_retweet_count = $5,
        popularity_count = $6
        WHERE tweet_id = $7
        "#,
        tweet_metrics.like_count,
        tweet_metrics.quote_count,
        tweet_metrics.reply_count,
        tweet_metrics.retweet_count,
        tweet_metrics.total_retweet_count,
        tweet_metrics.popularity_count,
        tweet["id"].as_str(),
    )
    .execute(pool)
    .await?;

    println!("updated tweet with id {}", tweet["id"].as_str().unwrap());
    Ok(())
}

// ----------------------------------------------------------------------------- backfill

/// Criteria:
/// 1. tweet_class match
/// 2. ordered by popularity
/// 3. within 24h or 1wk timeframe - todo
/// 4. currently missing media? I think that's probably the more important one of the two - todo
pub async fn find_tweets_to_backfill_by_class(
    pool: &PgPool,
    tweet_class: &str,
) -> Result<Vec<Tweet>, sqlx::error::Error> {
    let tweets = sqlx::query_as!(
        Tweet,
        r#"
        SELECT * FROM tweets WHERE tweet_class = $1 ORDER BY popularity_count
        "#,
        tweet_class,
    )
    .fetch_all(pool)
    .await?;

    Ok(tweets)
}

pub async fn backfill_rt_original_tweet(
    pool: &PgPool,
    body: &Value,
) -> Result<(), sqlx::error::Error> {
    // 1 media
    handle_media_for_tweet(&pool, &body["data"], &body).await?;

    // 2 helper tweets
    // we first need to save the users
    let users = &body["includes"]["users"];
    for user in users.as_array().unwrap() {
        store_user(&pool, &user).await.unwrap();
    }

    // only then the actual helper tweets
    let helper_tweets = body["includes"]["tweets"].as_array();
    if let Some(helper_ts) = helper_tweets {
        for ht in helper_ts.iter() {
            store_tweet(&pool, &ht, &body, "helper").await.unwrap();
        }
    }

    Ok(())
}

pub async fn backfill_helper_tweet(pool: &PgPool, body: &Value) -> Result<(), sqlx::error::Error> {
    handle_media_for_tweet(&pool, &body["data"], &body).await?;
    Ok(())
}

// ----------------------------------------------------------------------------- serve

/// What it does:
/// 1. Filters: by chosen timeframe (eg last 24h) + filters out 'helper' tweets
/// 2. Pages: sorts by tweet_id as secondary metric, then uses tweet_id as cursor to always get the next 20 tweets
/// 3. Sorts: by passed in metric first, tweet_id second (see 2)
/// 4. Limits to 20 tweets = page size
pub async fn fetch_next_page_of_tweets(
    pool: &PgPool,
    form: &web::Query<TweetParams>,
) -> Result<Vec<Tweet>, sqlx::error::Error> {
    let sql = format!(
        r#"
        SELECT * 
        FROM tweets
        WHERE tweet_created_at >= '{1}'
        AND tweet_class != 'helper'
        AND tweets.tweet_id > '{2}'
        ORDER BY {0} DESC, tweet_id DESC
        LIMIT 20;
        "#,
        form.sort_by.to_string(),
        form.timeframe.to_string(),
        form.last_tweet_id,
    );

    println!("{}", sql);

    let tweets = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(tweets)
}
