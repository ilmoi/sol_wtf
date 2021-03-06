use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::twitter::model::tweet::{fetch_tweet, Tweet};

// ----------------------------------------------------------------------------- structs/enums

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

#[tracing::instrument(skip(pool), level = "debug")]
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
#[tracing::instrument(skip(pool), level = "debug")]
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
#[tracing::instrument(skip(pool, tweet, body), level = "debug")]
pub async fn handle_media_for_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value,
) -> anyhow::Result<()> {
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
                        .filter(|&mo| {
                            // rust doesn't allow ? inside closures: https://mbuffett.com/posts/rust-less-error-handling/
                            // I know that twitter's api always provides media_key so I'm just going to use expect here
                            mo["media_key"].as_str().expect("no media_key [impossible]")
                                == mk.as_str().expect("no media_key [impossible]")
                        })
                        .collect::<Vec<&Value>>();
                    let relevant_object = relevant_objects.get(0);
                    prep_final_media_obj(relevant_object, mk)
                })
                .collect::<Vec<Value>>();

            let tweet_id = tweet["id"].as_str().ok_or(anyhow::anyhow!("no tweet_id"))?;
            store_all_media(&pool, tweet_id, final_media_objects).await?;
        }
    }
    Ok(())
}

#[tracing::instrument(skip(relevant_object, mk), level = "debug")]
pub fn prep_final_media_obj(relevant_object: Option<&&Value>, mk: &Value) -> Value {
    // rust doesn't allow ? inside closures: https://mbuffett.com/posts/rust-less-error-handling/
    // I know that twitter's api always provides media_key so I'm just going to use expect here
    match relevant_object {
        Some(relevant_obj) => json!({
            "media_key": mk.as_str().expect("media_key not present [impossible]"),
            "type": relevant_obj["type"].as_str(),
            "url": relevant_obj["url"].as_str(),
            "preview_image_url": relevant_obj["preview_image_url"].as_str(),
        }),
        None => json!({
            "media_key": mk.as_str().expect("media_key not present [impossible]"),
        }),
    }
}

#[tracing::instrument(skip(pool, media_objects), level = "debug")]
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

#[tracing::instrument(skip(pool, parent_tweet, media_object), level = "debug")]
pub async fn store_media(
    pool: &PgPool,
    parent_tweet: &Tweet,
    media_object: &Value,
) -> Result<(), sqlx::error::Error> {
    let display_url = build_display_url(&media_object);
    sqlx::query!(
        r#"
        INSERT INTO media
            (id, created_at, media_key, media_type, display_url, tweet_id)
        VALUES 
            ($1, $2, $3, $4, $5, $6)
            
        ON CONFLICT (media_key)
        DO UPDATE SET
            media_type = $4,
            display_url = $5;
        "#,
        Uuid::new_v4(),
        Utc::now(),
        media_object["media_key"].as_str(),
        media_object["type"].as_str(),
        display_url,
        parent_tweet.id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(skip(media_object), level = "debug")]
pub fn build_display_url(media_object: &Value) -> Option<&str> {
    let photo_url = media_object["url"].as_str();
    let preview_url = media_object["preview_image_url"].as_str();
    photo_url.or(preview_url)
}
