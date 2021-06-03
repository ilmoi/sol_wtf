use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

// ----------------------------------------------------------------------------- structs & enums

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TweetParams {
    pub page: u32,
    pub sort_by: SortBy,
    pub timeframe: Timeframe,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Popularity,
    Retweets,
    Likes,
    Replies,
    Time,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Timeframe {
    Hour,
    Four,
    Day,
    Week,
    Month,
}

// ----------------------------------------------------------------------------- traits

trait Convert {
    fn convert(&self) -> String;
}

impl Convert for SortBy {
    fn convert(&self) -> String {
        match self {
            SortBy::Popularity => String::from("popularity"),
            SortBy::Retweets => String::from("combined_retweet_count"),
            SortBy::Likes => String::from("like_count"),
            SortBy::Replies => String::from("reply_count"),
            SortBy::Time => String::from("tweet_created_at"),
        }
    }
}

impl Convert for Timeframe {
    fn convert(&self) -> String {
        match self {
            Timeframe::Hour => String::from("1"),
            Timeframe::Four => String::from("4"),
            Timeframe::Day => String::from("24"),
            Timeframe::Week => String::from("168"),
            Timeframe::Month => String::from("720"),
        }
    }
}

// ----------------------------------------------------------------------------- fns

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

// http://localhost:5001/tweets4?page=1&sort_by=popularity&timeframe=day
#[get("/tweets4")]
pub async fn tweets4(form: web::Query<TweetParams>, pool: web::Data<PgPool>) -> impl Responder {
    println!("{} {:?} {:?}", form.page, form.sort_by, form.timeframe);
    println!("{:?}", pool);
    HttpResponse::Ok().body("hey from tweets4 endpoint!")
}

#[get("/pull")]
pub async fn pull(pool: web::Data<PgPool>) -> impl Responder {
    let fake_json_data = r#"
    { "name": "hi" }
    "#;

    let v: Value = serde_json::from_str(fake_json_data).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO users
        (id, created_at, twitter_user_id, twitter_name, twitter_handle, profile_image, profile_url, entire_user)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        Uuid::new_v4(),
        Utc::now(),
        "3",
        "4",
        "5",
        "6",
        "7",
        v,
    )
        .execute(pool.as_ref())
        .await
        .unwrap();

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
