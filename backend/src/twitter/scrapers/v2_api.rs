use crate::config::Settings;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize)]
pub struct Params {
    expansions: String,
    tweet___fields: String,
    user___fields: String,
    media___fields: String,
    max_results: u32,
}

pub async fn get_user_timeline(config: &Settings) -> Result<Value, reqwest::Error> {
    let params = Params {
        expansions: String::from("author_id,referenced_tweets.id,referenced_tweets.id.author_id,in_reply_to_user_id,attachments.media_keys"),
        tweet___fields: String::from(
            "created_at,in_reply_to_user_id,public_metrics,referenced_tweets",
        ),
        user___fields: String::from("name,username,profile_image_url,url,public_metrics"),
        media___fields: String::from("preview_image_url,url"),
        max_results: 100,
    };

    let endpoint_url = "https://api.twitter.com/2/users/1394594789492879363/tweets";
    let endpoint_params_raw = serde_url_params::to_string(&params).unwrap();
    //needed coz twitter has weird field namings with dots, and you can't have dots in struct fields
    let endpoint_params = endpoint_params_raw.replace("___", ".");
    let url = format!("{}?{}", endpoint_url, endpoint_params,);

    let body = v2_api_get(&config, &url).await?;
    Ok(body)
}

pub async fn v2_api_get(config: &Settings, url: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let bearer_token = &config.twitter.bearer_token;

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .await?;

    println!("Status: {}", res.status());
    let body: Value = res.json().await?;
    println!("Body:\n\n{:#?}", &body);

    Ok(body)
}
