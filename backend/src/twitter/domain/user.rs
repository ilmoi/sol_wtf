use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub twitter_user_id: String,
    pub twitter_name: String,
    pub twitter_handle: String,
    pub profile_url: String,
    pub profile_image: Option<String>,
    pub followers_count: Option<i64>,
    pub following_count: Option<i64>,
    pub listed_count: Option<i64>,
    pub tweet_count: Option<i64>,
    pub entire_user: Option<Value>,
}

pub async fn fetch_user(pool: &PgPool, user_id: &str) -> Result<User, sqlx::error::Error> {
    let res = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE twitter_user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    println!("fetched user with id {}", user_id);
    Ok(res)
}

pub async fn fetch_user_by_uuid(pool: &PgPool, id: Uuid) -> Result<User, sqlx::error::Error> {
    let res = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
        id,
    )
    .fetch_one(pool)
    .await?;

    println!("fetched user with id {}", id);
    Ok(res)
}

pub async fn store_user(pool: &PgPool, user: &Value) -> Result<(), sqlx::error::Error> {
    // check if user already exists - if so, update it
    let user_id = user["id"].as_str().unwrap();
    let found_user = fetch_user(&pool, user_id).await;
    if let Ok(_) = found_user {
        return update_user(&pool, &user).await;
    }

    sqlx::query!(
        r#"
        INSERT INTO users
        (id, created_at, 
        twitter_user_id, twitter_name, twitter_handle, profile_url, profile_image, 
        followers_count, following_count, listed_count, tweet_count)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
        "#,
        Uuid::new_v4(),
        Utc::now(),
        user_id,
        user["name"].as_str(),
        user["username"].as_str(),
        user["url"].as_str(),
        user["profile_image_url"].as_str(),
        user["public_metrics"]["followers_count"].as_i64(),
        user["public_metrics"]["following_count"].as_i64(),
        user["public_metrics"]["listed_count"].as_i64(),
        user["public_metrics"]["tweet_count"].as_i64(),
    )
    .execute(pool)
    .await?;

    println!("stored user with id {}", user["id"].as_str().unwrap());
    Ok(())
}

pub async fn update_user(pool: &PgPool, user: &Value) -> Result<(), sqlx::error::Error> {
    sqlx::query!(
        r#"
        UPDATE users SET 
        twitter_name = $1, 
        twitter_handle = $2, 
        profile_url = $3, 
        profile_image = $4, 
        followers_count = $5, 
        following_count = $6, 
        listed_count = $7, 
        tweet_count = $8
        WHERE twitter_user_id = $9
        "#,
        user["name"].as_str(),
        user["username"].as_str(),
        user["url"].as_str(),
        user["profile_image_url"].as_str(),
        user["public_metrics"]["followers_count"].as_i64(),
        user["public_metrics"]["following_count"].as_i64(),
        user["public_metrics"]["listed_count"].as_i64(),
        user["public_metrics"]["tweet_count"].as_i64(),
        user["id"].as_str(),
    )
    .execute(pool)
    .await?;

    println!("updated user with id {}", user["id"].as_str().unwrap());
    Ok(())
}
