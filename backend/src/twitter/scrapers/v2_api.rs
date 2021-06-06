use crate::config::Settings;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Response;
use serde_json::Value;
use std::fmt;

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize)]
pub struct Params {
    pub expansions: String,
    pub tweet___fields: String,
    pub user___fields: String,
    pub media___fields: String,
    pub max_results: Option<u32>,
}

#[derive(Debug)]
pub struct RateLimits {
    pub limit_left: u32,
    pub limit_total: u32,
    pub reset_time: DateTime<Utc>,
}

impl fmt::Display for RateLimits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RATE LIMIT LEFT: {} / {}, RESET TIME: {}",
            self.limit_left,
            self.limit_total,
            self.reset_time.format("%H:%M:%S GMT")
        )
    }
}

pub async fn v2_api_get(
    config: &Settings,
    mut url: String,
    params: Option<&Params>,
) -> Result<(Value, RateLimits), reqwest::Error> {
    let client = reqwest::Client::new();
    let bearer_token = &config.twitter.bearer_token;

    if let Some(params) = params {
        let endpoint_params_raw = serde_url_params::to_string(params).unwrap();
        //needed coz twitter has weird field namings with dots, and you can't have dots in struct fields
        let endpoint_params = endpoint_params_raw.replace("___", ".");
        url = format!("{}?{}", url, endpoint_params);
    }

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .await?;

    println!(">>> GET CALL STATUS: {}", res.status());
    let rate_limits = handle_rate_limits(&res);
    let body: Value = res.json().await?;
    // println!("Body:\n\n{:#?}", &body);
    // println!("Rate limits:\n\n{:#?}", &rate_limits);
    Ok((body, rate_limits))
}

pub fn handle_rate_limits(res: &Response) -> RateLimits {
    let limit_left = res
        .headers()
        .get("x-rate-limit-remaining")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let limit_total = res
        .headers()
        .get("x-rate-limit-limit")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let reset_time = res
        .headers()
        .get("x-rate-limit-reset")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let reset_time =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(reset_time.into(), 0), Utc);

    let rate_limits = RateLimits {
        limit_left,
        limit_total,
        reset_time,
    };

    println!(">>> {}", rate_limits);

    rate_limits
}

// ----------------------------------------------------------------------------- specific calls

pub async fn get_user_timeline(
    config: &Settings,
    user_id: &str,
) -> Result<(Value, RateLimits), reqwest::Error> {
    let url = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
    let params = Params {
        expansions: String::from("author_id,referenced_tweets.id,referenced_tweets.id.author_id,in_reply_to_user_id,attachments.media_keys"),
        tweet___fields: String::from(
            "created_at,in_reply_to_user_id,public_metrics,referenced_tweets",
        ),
        user___fields: String::from("name,username,profile_image_url,url,public_metrics"),
        media___fields: String::from("preview_image_url,url"),
        max_results: Some(100),
    };
    v2_api_get(&config, url, Some(&params)).await
}

pub async fn get_single_tweet(
    config: &Settings,
    tweet_id: &str,
) -> Result<(Value, RateLimits), reqwest::Error> {
    let url = format!("https://api.twitter.com/2/tweets/{}", tweet_id);
    let params = Params {
        expansions: String::from("author_id,referenced_tweets.id,referenced_tweets.id.author_id,in_reply_to_user_id,attachments.media_keys"),
        tweet___fields: String::from(
            "created_at,in_reply_to_user_id,public_metrics,referenced_tweets",
        ),
        user___fields: String::from("name,username,profile_image_url,url,public_metrics"),
        media___fields: String::from("preview_image_url,url"),
        max_results: None,
    };
    v2_api_get(&config, url, Some(&params)).await
}

pub async fn fetch_followed_users(
    config: &Settings,
) -> Result<(Value, RateLimits), reqwest::Error> {
    let soldotwtf = "1397861458441089025";
    let url = format!("https://api.twitter.com/2/users/{}/following", soldotwtf);
    v2_api_get(&config, url, None).await
}
