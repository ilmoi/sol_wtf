use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::twitter::domain::tweet::{fetch_tweet, Tweet};
use crate::utils::general::pick_first_option_available;

#[derive(sqlx::FromRow, Debug)]
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

    println!("fetched media with id {}", media_key);
    Ok(res)
}

pub async fn handle_media_for_tweet(
    pool: &PgPool,
    tweet: &Value,
    body: &Value,
) -> Result<(), sqlx::error::Error> {
    // not all tweets have media_keys - only those with photos / videos / etc attached
    let media_keys = tweet["attachments"]["media_keys"].as_array();
    if let Some(media_ks) = media_keys {
        let tweet_id = tweet["id"].as_str().unwrap();

        // there can be 2 cases here:
        // 1) we're storing a normal tweet - media objects will be available if media_keys are
        // 2) we're storing a retweet - only keys will be present. So we store just the keys and will fetch objects later.
        let media_objects = body["includes"]["media"].as_array();
        match media_objects {
            Some(media_obj) => {
                let filtered_media_obj = media_obj
                    .iter()
                    .filter(|&mo| {
                        media_ks
                            .iter()
                            .any(|mk| mo["media_key"].as_str().unwrap() == mk.as_str().unwrap())
                    })
                    .collect::<Vec<&Value>>();
                store_all_media(&pool, tweet_id, filtered_media_obj).await?;
            }
            None => {
                let fake_media_obj = media_ks
                    .iter()
                    .map(|mk| {
                        json!({
                            "media_key": mk.as_str().unwrap()
                        })
                    })
                    .collect::<Vec<Value>>();

                // todo ugly
                let fake_media_obj_2 = fake_media_obj.iter().map(|mo| mo).collect::<Vec<&Value>>();

                store_all_media(&pool, tweet_id, fake_media_obj_2).await?;
            }
        }
    }

    Ok(())
}

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

pub async fn store_media(
    pool: &PgPool,
    parent_tweet: &Tweet,
    media_object: &Value,
) -> Result<(), sqlx::error::Error> {
    let photo_url = media_object["url"].as_str();
    let preview_url = media_object["preview_image_url"].as_str();
    let display_url = pick_first_option_available(photo_url, preview_url, None);

    sqlx::query!(
        r#"
            INSERT INTO media
            (id, created_at, media_key, media_type, display_url, tweet_id)
            VALUES ($1, $2, $3, $4, $5, $6) 
            "#,
        Uuid::new_v4(),
        Utc::now(),
        media_object["media_key"].as_str(),
        media_object["type"].as_str(),
        display_url,
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
