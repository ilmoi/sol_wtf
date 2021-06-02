use actix_web::{get, web, HttpResponse, Responder, Result};

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

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[get("/tweets4")]
pub async fn tweets4(form: web::Query<TweetParams>) -> Result<String> {
    println!("{} {:?} {:?}", form.page, form.sort_by, form.timeframe);
    Ok("worked".into())
}
