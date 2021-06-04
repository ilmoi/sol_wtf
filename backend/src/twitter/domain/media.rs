use crate::twitter::domain::tweet::{fetch_tweet, Tweet};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct Media {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub media_key: String,
    pub media_type: String,
    pub preview_image_url: Option<String>,
    pub tweet_id: Uuid,
}

// ----------------------------------------------------------------------------- fn

pub async fn store_all_media(
    pool: &PgPool,
    tweet_id: &str,
    media_objects: Vec<&Value>,
) -> Result<(), sqlx::error::Error> {
    let parent_tweet = fetch_tweet(&pool, tweet_id).await.unwrap();

    for &mo in media_objects.iter() {
        let media_key = mo["media_key"].as_str().unwrap();
        let found_media = fetch_media(&pool, media_key).await;
        match found_media {
            Ok(_m) => println!("media obj {} already exists in db.", media_key),
            Err(_e) => store_media(&pool, &parent_tweet, mo).await.unwrap(),
        }
    }

    println!("stored all media for user {}", tweet_id);
    Ok(())
}

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

    println!("fetched media with id {}", media_key);
    Ok(res)
}

pub async fn store_media(
    pool: &PgPool,
    parent_tweet: &Tweet,
    media_object: &Value,
) -> Result<(), sqlx::error::Error> {
    sqlx::query!(
        r#"
            INSERT INTO media
            (id, created_at, media_key, media_type, preview_image_url, tweet_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        Uuid::new_v4(),
        Utc::now(),
        media_object["media_key"].as_str(),
        media_object["type"].as_str(),
        media_object["preview_image_url"].as_str(),
        parent_tweet.id,
    )
    .execute(pool)
    .await
    .unwrap();

    println!(
        "stored media with id {}",
        media_object["media_key"].as_str().unwrap()
    );
    Ok(())
}
