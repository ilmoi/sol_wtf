use crate::config::Settings;
use crate::twitter::scrapers::general::{v2_api_get, Params, RateLimits};
use serde_json::Value;

#[tracing::instrument(skip(config))]
pub async fn get_user_timeline(
    config: &Settings,
    user_id: &str,
) -> anyhow::Result<(Value, RateLimits)> {
    let url = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
    let params = Params {
        expansions: Some(String::from("author_id,referenced_tweets.id,referenced_tweets.id.author_id,in_reply_to_user_id,attachments.media_keys")),
        tweet___fields: Some(String::from(
            "created_at,in_reply_to_user_id,public_metrics,referenced_tweets",
        )),
        user___fields: Some(String::from("name,username,profile_image_url,url,public_metrics")),
        media___fields: Some(String::from("preview_image_url,url")),
        max_results: Some(config.app.refresh_tweets_per_user),
        pagination_token: None,
    };
    v2_api_get(&config, url, Some(&params)).await
}

#[tracing::instrument(skip(config))]
pub async fn get_single_tweet(
    config: &Settings,
    tweet_id: &str,
) -> anyhow::Result<(Value, RateLimits)> {
    let url = format!("https://api.twitter.com/2/tweets/{}", tweet_id);
    let params = Params {
        expansions: Some(String::from("author_id,referenced_tweets.id,referenced_tweets.id.author_id,in_reply_to_user_id,attachments.media_keys")),
        tweet___fields: Some(String::from(
            "created_at,in_reply_to_user_id,public_metrics,referenced_tweets",
        )),
        user___fields: Some(String::from("name,username,profile_image_url,url,public_metrics")),
        media___fields: Some(String::from("preview_image_url,url")),
        max_results: None,
        pagination_token: None,
    };
    v2_api_get(&config, url, Some(&params)).await
}

#[tracing::instrument(skip(config))]
pub async fn fetch_followed_users(
    config: &Settings,
    pagination_token: Option<String>,
) -> anyhow::Result<(Value, RateLimits)> {
    let soldotwtf = &config.app.followers_for_account;
    let url = format!("https://api.twitter.com/2/users/{}/following", soldotwtf);
    let params = Params {
        expansions: None,
        tweet___fields: None,
        user___fields: None,
        media___fields: None,
        max_results: Some(1000),
        pagination_token,
    };
    v2_api_get(&config, url, Some(&params)).await
}

#[tracing::instrument(skip(config), level = "debug")]
pub async fn fetch_all_followed_users(
    config: &Settings,
) -> anyhow::Result<(Vec<Value>, RateLimits)> {
    let mut users: Vec<Value> = vec![];
    let mut rate_limits;
    let mut page_token: Option<String> = None;

    loop {
        let (mut new_users, new_rate_limits) =
            fetch_followed_users(&config, page_token.clone()).await?;
        let mut new_users_vec = new_users["data"]
            .as_array_mut()
            .ok_or(anyhow::anyhow!("failed to convert vec to array"))?;

        // taken from https://stackoverflow.com/questions/40792801/best-way-to-concatenate-vectors-in-rust#40795247
        users.append(&mut new_users_vec);
        rate_limits = new_rate_limits;

        match new_users["meta"]["next_token"].as_str() {
            Some(next_token) => page_token = Some(next_token.into()),
            None => break,
        }
    }
    Ok((users, rate_limits))
}
