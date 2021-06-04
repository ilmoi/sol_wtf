use crate::twitter::domain::media::store_all_media;
use crate::twitter::domain::user;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug)]
pub struct Tweet {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub tweet_id: String,
    pub tweet_created_at: DateTime<Utc>,
    pub tweet_text: String,
    pub tweet_url: String,
    // reference tweets
    pub replied_to_tweet_id: Option<String>,
    pub is_reply: Option<bool>,
    pub quoted_tweet_id: Option<String>,
    pub is_quote: Option<bool>,
    pub retweeted_tweet_id: Option<String>,
    pub is_retweet: Option<bool>,
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

pub struct TweetMetrics {
    pub like_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
    pub total_retweet_count: i64,
    pub popularity_count: i64,
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
pub async fn store_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value,
) -> Result<(), sqlx::error::Error> {
    let author_id = tweet["author_id"].as_str().unwrap();
    let author = user::fetch_user(&pool, author_id).await.unwrap();
    let tweet_id = tweet["id"].as_str().unwrap();
    let tweet_url = format!("https://twitter.com/{}/status/{}", &author_id, &tweet_id);
    let tweet_created_at =
        DateTime::parse_from_rfc3339(tweet["created_at"].as_str().unwrap()).unwrap();

    // handle reference tweets
    let mut replied_to_tweet_id: Option<String> = None;
    let mut quoted_tweet_id: Option<String> = None;
    let mut retweeted_tweet_id: Option<String> = None;

    let ref_tweets = tweet.get("referenced_tweets");
    if let Some(ref_tweets) = ref_tweets {
        for rt in ref_tweets.as_array().unwrap() {
            match rt["type"].as_str().unwrap() {
                "replied_to" => replied_to_tweet_id = Some(rt["id"].as_str().unwrap().into()),
                "quoted" => quoted_tweet_id = Some(rt["id"].as_str().unwrap().into()),
                "retweeted" => retweeted_tweet_id = Some(rt["id"].as_str().unwrap().into()),
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
        replied_to_tweet_id, is_reply, quoted_tweet_id, is_quote, retweeted_tweet_id, is_retweet,
        like_count, quote_count, reply_count, retweet_count, total_retweet_count, popularity_count,
        user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
        $19)
        "#,
        Uuid::new_v4(),
        Utc::now(),
        tweet_id,
        tweet_created_at,
        tweet["text"].as_str(),
        tweet_url,
        replied_to_tweet_id,
        false,
        quoted_tweet_id,
        false,
        retweeted_tweet_id,
        false,
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
    let media_keys = tweet["attachments"]["media_keys"].as_array();
    if let Some(media_ks) = media_keys {
        let media_objects = body["includes"]["media"].as_array();
        if let Some(media_obj) = media_objects {
            let filtered_media_obj = media_obj
                .iter()
                .filter(|&mo| {
                    media_ks
                        .iter()
                        .any(|k| mo["media_key"].as_str().unwrap() == k.as_str().unwrap())
                })
                .collect::<Vec<&Value>>();

            store_all_media(&pool, tweet_id, filtered_media_obj).await?;
        }
    }

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
