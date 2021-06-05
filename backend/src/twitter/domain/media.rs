use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::twitter::domain::tweet::{fetch_tweet, Tweet};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub media_key: String,
    pub media_type: Option<String>,
    pub display_url: Option<String>,
    pub tweet_id: Uuid,
}

// ----------------------------------------------------------------------------- fn

pub async fn fetch_media(pool: &PgPool, media_key: &str) -> Result<Media, sqlx::error::Error> {
    let res = sqlx::query_as!(
        Media,
        r#"
        SELECT * FROM media WHERE media_key = $1
        "#,
        media_key,
    )
    .fetch_one(pool)
    .await?;
    Ok(res)
}

/// returns an empty vector if a tweet has no media
pub async fn fetch_all_media_for_tweet(
    pool: &PgPool,
    tweet_id: Uuid,
) -> Result<Vec<Media>, sqlx::error::Error> {
    let res = sqlx::query_as!(
        Media,
        r#"
        SELECT * FROM media WHERE tweet_id = $1
        "#,
        tweet_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(res)
}

/// Twitter v2 api clarification:
///     - media_keys = array of keys only attached to individual tweet
///     - media_objects = array of objects {key, type, url} for the ENTIRE returned batch of tweets
///     Thus we have to iterate over media keys and find matching objects.
///
/// Model clarification: 2 cases are possible:
///     1) for "normal" tweets - media objects will be available if media_keys are
///     2) for "rt_original" tweets - only keys will be present. So we store just the keys and will fetch objects later.
pub async fn handle_media_for_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value,
) -> Result<(), sqlx::error::Error> {
    // media_keys may not be present for a tweet if has no media content
    if let Some(media_keys) = tweet["attachments"]["media_keys"].as_array() {
        // media_objects may not be present for the batch if no tweets have any media
        if let Some(media_objects) = body["includes"]["media"].as_array() {
            let final_media_objects = media_keys
                .iter()
                .map(|mk| {
                    //will be 0 if not found, 1 if found, hence the option
                    let relevant_objects = media_objects
                        .iter()
                        .filter(|&mo| mo["media_key"].as_str().unwrap() == mk.as_str().unwrap())
                        .collect::<Vec<&Value>>();
                    let relevant_object = relevant_objects.get(0);
                    prep_final_media_obj(relevant_object, mk)
                })
                .collect::<Vec<Value>>();

            let tweet_id = tweet["id"].as_str().unwrap();
            store_all_media(&pool, tweet_id, final_media_objects).await?;
        }
    }
    Ok(())
}

pub fn prep_final_media_obj(relevant_object: Option<&&Value>, mk: &Value) -> Value {
    match relevant_object {
        Some(relevant_obj) => {
            json!({
                "media_key": mk.as_str().unwrap(),
                "type": relevant_obj["type"].as_str(),
                "url": relevant_obj["url"].as_str(),
                "preview_image_url": relevant_obj["preview_image_url"].as_str(),
            })
        }
        None => {
            json!({
                "media_key": mk.as_str().unwrap(),
            })
        }
    }
}

pub async fn store_all_media(
    pool: &PgPool,
    tweet_id: &str,
    media_objects: Vec<Value>,
) -> Result<(), sqlx::error::Error> {
    let parent_tweet = fetch_tweet(&pool, tweet_id).await?;

    for mo in media_objects.iter() {
        store_media(&pool, &parent_tweet, mo).await?;
    }
    Ok(())
}

pub async fn store_media(
    pool: &PgPool,
    parent_tweet: &Tweet,
    media_object: &Value,
) -> Result<(), sqlx::error::Error> {
    // check if media already exists - if so, update it
    let media_key = media_object["media_key"].as_str().unwrap();
    let found_media = fetch_media(&pool, media_key).await;
    let display_url = build_display_url(&media_object);
    if found_media.is_ok() {
        return update_media(&pool, &media_object, display_url).await;
    }

    sqlx::query!(
        r#"
            INSERT INTO media
            (id, created_at, media_key, media_type, display_url, tweet_id)
            VALUES ($1, $2, $3, $4, $5, $6) 
            "#,
        Uuid::new_v4(),
        Utc::now(),
        media_key,
        media_object["type"].as_str(),
        display_url,
        parent_tweet.id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_media(
    pool: &PgPool,
    media_object: &Value,
    display_url: Option<&str>,
) -> Result<(), sqlx::error::Error> {
    sqlx::query!(
        r#"
        UPDATE media SET
        media_type = $1,
        display_url = $2
        WHERE media_key = $3
        "#,
        media_object["media_type"].as_str(),
        display_url,
        media_object["media_key"].as_str(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub fn build_display_url(media_object: &Value) -> Option<&str> {
    let photo_url = media_object["url"].as_str();
    let preview_url = media_object["preview_image_url"].as_str();
    photo_url.or(preview_url)
}
