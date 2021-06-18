use crate::config::Settings;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Response;
use serde_json::Value;
use std::fmt;

// ----------------------------------------------------------------------------- structs/enums

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize)]
pub struct Params {
    pub expansions: Option<String>,
    pub tweet___fields: Option<String>,
    pub user___fields: Option<String>,
    pub media___fields: Option<String>,
    pub max_results: Option<u32>,
    pub pagination_token: Option<String>,
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

// ------------------------------------------------------------------------------ fn

#[tracing::instrument(skip(config), level = "debug")]
pub async fn v2_api_get(
    config: &Settings,
    mut url: String,
    params: Option<&Params>,
) -> anyhow::Result<(Value, RateLimits)> {
    let client = reqwest::Client::new();
    let bearer_token = &config.twitter.bearer_token;

    if let Some(params) = params {
        let endpoint_params_raw = serde_url_params::to_string(params)?;
        //needed coz twitter has weird field namings with dots, and you can't have dots in struct fields
        let endpoint_params = endpoint_params_raw.replace("___", ".");
        url = format!("{}?{}", url, endpoint_params);
    }

    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .await?;

    tracing::info!(">>>I: GET call status: {}", &res.status());
    let rate_limits = handle_rate_limits(&res)?; //  ^ the trait `From<Box<dyn StdError>>` is not implemented for `reqwest::Error`
    let body: Value = res.json().await?;
    // tracing::info!(">>>I: GET call returned body:\n\n{:#?}", &body);
    Ok((body, rate_limits))
}

#[tracing::instrument(level = "debug")]
pub fn handle_rate_limits(res: &Response) -> anyhow::Result<RateLimits> {
    let limit_left = res
        .headers()
        .get("x-rate-limit-remaining")
        // https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no
        // if your function returns a Result<T, E> you cannot use the ? in a value of type Option<T>. Or vice versa.
        // so have to convert Option<T> to Result<T, E>
        // have to use the anyhow macro - https://users.rust-lang.org/t/how-to-return-with-an-error-from-an-iterator-chain/44396/2
        .ok_or(anyhow::anyhow!("no header"))?
        .to_str()?
        .parse::<u32>()?;
    let limit_total = res
        .headers()
        .get("x-rate-limit-limit")
        .ok_or(anyhow::anyhow!("no header"))?
        .to_str()?
        .parse::<u32>()?;
    let reset_time = res
        .headers()
        .get("x-rate-limit-reset")
        .ok_or(anyhow::anyhow!("no header"))?
        .to_str()?
        .parse::<u32>()?;
    let reset_time =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(reset_time.into(), 0), Utc);

    let rate_limits = RateLimits {
        limit_left,
        limit_total,
        reset_time,
    };

    tracing::info!(">>>I: Rate limits: {:?}", rate_limits);
    Ok(rate_limits)
}

// ----------------------------------------------------------------------------- saved for personal ref: box dyn err approach

// #[tracing::instrument(level = "debug")]
// pub fn handle_rate_limits(res: &Response) -> Result<RateLimits, Box<dyn std::error::Error>> {
//     let limit_left = res
//         .headers()
//         .get("x-rate-limit-remaining")
//         // https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no
//         // if your function returns a Result<T, E> you cannot use the ? in a value of type Option<T>. Or vice versa.
//         // so have to convert Option<T> to Result<T, E>
//         .ok_or("no header")?
//         .to_str()?
//         .parse::<u32>()?;
//     let limit_total = res
//         .headers()
//         .get("x-rate-limit-limit")
//         .ok_or("no header")?
//         .to_str()?
//         .parse::<u32>()?;
//     let reset_time = res
//         .headers()
//         .get("x-rate-limit-reset")
//         .ok_or("no header")?
//         .to_str()?
//         .parse::<u32>()?;
//     let reset_time =
//         DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(reset_time.into(), 0), Utc);
//
//     let rate_limits = RateLimits {
//         limit_left,
//         limit_total,
//         reset_time,
//     };
//
//     tracing::info!(">>>I: Rate limits: {:?}", rate_limits);
//     Ok(rate_limits)
// }

// ----------------------------------------------------------------------------- saved for personal ref: thiserror approach

// #[tracing::instrument(level = "debug")]
// pub fn handle_rate_limits(res: &Response) -> Result<RateLimits, RateLimitParseError> {
//     let limit_left = res
//         .headers()
//         .get("x-rate-limit-remaining")
//         // https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no
//         // if your function returns a Result<T, E> you cannot use the ? in a value of type Option<T>. Or vice versa.
//         // so have to convert Option<T> to Result<T, E>
//         .ok_or("no header".to_string())?
//         .to_str()?
//         .parse::<u32>()?;
//     let limit_total = res
//         .headers()
//         .get("x-rate-limit-limit")
//         .ok_or("no header".to_string())?
//         .to_str()?
//         .parse::<u32>()?;
//     let reset_time = res
//         .headers()
//         .get("x-rate-limit-reset")
//         .ok_or("no header".to_string())?
//         .to_str()?
//         .parse::<u32>()?;
//     let reset_time =
//         DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(reset_time.into(), 0), Utc);
//
//     let rate_limits = RateLimits {
//         limit_left,
//         limit_total,
//         reset_time,
//     };
//
//     tracing::info!(">>>I: Rate limits: {:?}", rate_limits);
//     Ok(rate_limits)
// }
//
//
// #[derive(thiserror::Error, Debug)]
// pub enum RateLimitParseError {
//     // NoneError is unstable, so this won't fly - https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no
//     // #[error("header didn't exist")]
//     // NoneError(#[from] NoneError),
//     // instead we use .ok_or() above to convert to a String, and then we impl From<String> for RateLimitParseError
//     #[error("no header present")]
//     NoHeader(String),
//     #[error("header can't be converted to str")]
//     ToStrError(#[from] ToStrError),
//     #[error("failed to parse int out of the header")]
//     ParseIntError(#[from] ParseIntError),
// }
//
// impl From<String> for RateLimitParseError {
//     fn from(e: String) -> Self {
//         Self::NoHeader(e)
//     }
// }
