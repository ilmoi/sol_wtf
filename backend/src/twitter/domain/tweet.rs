use async_recursion::async_recursion;
use chrono::{DateTime, Duration, Utc};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::twitter::domain::media::handle_media_for_tweet;
use crate::twitter::domain::user::fetch_user;
use crate::twitter::routes::serve::{SortBy, TweetParams};
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

#[async_recursion]
pub async fn store_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value, // not ideal that we have to pass the body here, but I need the media array from "includes"
    tweet_class: &str,
) -> Result<(), sqlx::error::Error> {
    let tweet_id = tweet["id"].as_str().unwrap();
    let author_id = tweet["author_id"].as_str().unwrap();
    let author = fetch_user(&pool, author_id).await?; //needed to pass into sqlx query
    let tweet_url = format!("https://twitter.com/{}/status/{}", &author_id, &tweet_id);
    let tweet_created_at =
        DateTime::parse_from_rfc3339(tweet["created_at"].as_str().unwrap()).unwrap();

    // handle reference tweets
    let mut replied_to_tweet_id: Option<String> = None;
    let mut quoted_tweet_id: Option<String> = None;
    if let Some(ref_tweets) = tweet.get("referenced_tweets") {
        for rt in ref_tweets.as_array().unwrap() {
            match rt["type"].as_str().unwrap() {
                "retweeted" => {
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
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            
        ON CONFLICT (tweet_id)
        DO UPDATE SET
            like_count = $10,
            quote_count = $11,
            reply_count = $12,
            retweet_count = $13,
            total_retweet_count = $14,
            popularity_count = $15
        "#,
        Uuid::new_v4(),
        Utc::now(),
        tweet_id,
        tweet_created_at,
        tweet["text"].as_str(),
        tweet_url,
        // reference tweets
        replied_to_tweet_id,
        quoted_tweet_id,
        tweet_class,
        // metrics
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
    Ok(())
}

// ----------------------------------------------------------------------------- backfill

/// Criteria:
/// 1. tweet_class match
/// 2. ordered by popularity
/// 3. within 24h. My calc:
///     - getting 500 tweets to backfill per 13k in the db
///     - this suggests that when I have 150k, I would have to backfill 6000 - which is already way above 900 limit
/// 4. currently missing media? Thought about it, but I think a tweet without a quote/reply is worse than a tweet without a picture. So for now no.
pub async fn fetch_tweets_to_backfill_by_class(
    pool: &PgPool,
    tweet_class: &str,
) -> Result<Vec<Tweet>, sqlx::error::Error> {
    let last_24h = Utc::now() - Duration::days(1);

    let tweets = sqlx::query_as!(
        Tweet,
        r#"
        SELECT * FROM tweets 
        WHERE tweet_class = $1 
        AND tweet_created_at > $2
        ORDER BY popularity_count
        "#,
        tweet_class,
        last_24h,
    )
    .fetch_all(pool)
    .await?;
    Ok(tweets)
}

// ----------------------------------------------------------------------------- serve

/// Time case is trivial - simply order by creation date.
/// Dates are long / diverse enough that we're unlikely to have two be exactly the same.
///
/// Other, not time based cases:
///
/// New metric
/// - invents a new metric which:
///     - takes user chosen metric (eg popularity count)
///     - takes the first 10 numbers from tweet id
///     - concats the two together into a single string, then casts it to int
/// - why do it?
///     - need a unique metric we can use as cursor when fetching tweets in pages (keyset pagination)
/// - (!) THE COLUMN IS INDEXED FOR FASTER QUERIES
///
/// Filter:
/// - ignore helper tweets
/// - limit to timeframe specified by user (eg last 24h)
/// - bottom of query cut off: use the newly invented metric above
/// - top of query cut off: page size (eg 20)
///
/// Order:
/// - use the newly invented metric above
pub async fn fetch_next_page_of_tweets(
    pool: &PgPool,
    form: &web::Query<TweetParams>,
) -> Result<Vec<Tweet>, sqlx::error::Error> {
    let sql;
    if let SortBy::Time = form.sort_by {
        let mut last_metric = form.last_metric.clone(); //todo .clone() the best way here?

        // on the first call the frontend sends the largest possible integer.
        // This doesn't work for time, so we swap it out for a unix timestamp 1 hour in the future.
        if form.last_metric == "2036854775807" {
            last_metric = (Utc::now() + Duration::hours(1)).to_rfc3339().to_owned()
        }

        sql = format!(
            r#"
            SELECT * 
            FROM tweets
            WHERE 
                tweet_class != 'helper'
                AND tweet_created_at >= '{0}'
                AND tweet_created_at < '{1}'
            ORDER BY tweet_created_at DESC
            LIMIT 20;
            "#,
            form.timeframe.to_string(),
            last_metric,
        );
    } else {
        sql = format!(
            r#"
            SELECT * 
            FROM tweets
            WHERE 
                tweet_class != 'helper'
                AND tweet_created_at >= '{1}'
                AND CAST({0} || LEFT(tweet_id, 10) AS BIGINT) < 
                    CAST('{3}' || LEFT('{2}', 10) AS BIGINT)
            ORDER BY 
                CAST({0} || LEFT(tweet_id, 10) AS BIGINT) DESC 
            LIMIT 20;
            "#,
            form.sort_by.to_string(),
            form.timeframe.to_string(),
            form.last_tweet_id,
            form.last_metric,
        );
    }

    let tweets = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(tweets)
}
