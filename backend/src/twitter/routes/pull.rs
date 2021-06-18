#![allow(clippy::async_yields_async)]

use std::ops::Deref;
use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::config::Settings;
use crate::twitter::core::jobs::{
    backfill_missing_media_and_helper_tweets, pull_timelines_for_followed_users,
};
use crate::utils::errors::ApiError;
use anyhow::Context;

#[tracing::instrument(skip(pool, config))]
#[get("/pull")]
pub async fn pull(
    pool: web::Data<Arc<PgPool>>,
    config: web::Data<Arc<Settings>>,
) -> Result<HttpResponse, ApiError> {
    let config = config.as_ref().deref();
    let pool = pool.as_ref().deref();
    pull_timelines_for_followed_users(pool, config)
        .await
        .context("failed to pull timelines for followed users")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(skip(pool, config))]
#[get("/backfill")]
pub async fn backfill(
    pool: web::Data<Arc<PgPool>>,
    config: web::Data<Arc<Settings>>,
) -> Result<HttpResponse, ApiError> {
    let config = config.as_ref().deref();
    let pool = pool.as_ref().deref();
    backfill_missing_media_and_helper_tweets(pool, config)
        .await
        .context("failed to backfill media / helper tweets")?;
    Ok(HttpResponse::Ok().finish())
}
