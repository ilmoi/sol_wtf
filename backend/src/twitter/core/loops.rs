use std::cmp::min;
use std::future::Future;

use sqlx::PgPool;

use crate::config::Settings;

#[tracing::instrument(skip(object_arr, settings, pool, f, rate_limit))]
pub async fn loop_until_hit_rate_limit<'a, T, Fut>(
    object_arr: &'a [T],
    settings: &'a Settings,
    pool: &'a PgPool,
    f: impl Fn(&'a Settings, &'a PgPool, &'a T) -> Fut + Copy,
    rate_limit: usize,
) where
    // https://stackoverflow.com/questions/60717746/how-to-accept-an-async-function-as-an-argument
    Fut: Future<Output = anyhow::Result<()>>,
{
    // this is the easiest way to impl. rate limits.
    // A much harder approach would be to wrap one in Arc(Mutex()) and update from each async task.
    let total = object_arr.len();
    let capped_total = min(total, rate_limit); // in theory should handle the ones that didn't fit in, but for now cba

    let mut futs = vec![];
    for (i, object) in object_arr[..capped_total].iter().enumerate() {
        futs.push(async move {
            tracing::info!(">>>I: Processing {}/{}", i + 1, total);

            // todo: https://stackoverflow.com/questions/68030477/rust-print-the-name-of-the-function-passed-in-as-argument
            // if try to add ? -> get: cannot use the `?` operator in an async block that returns `()`. So instead handing errors here.
            f(settings, pool, object).await.unwrap_or_else(|e| {
                tracing::error!(
                    ">>>E: Failed to process iteration {} of the loop. Function used: Full error: {}",
                    i + 1,
                    e,
                );
            });
        });
    }
    futures::future::join_all(futs).await;
}

// pub async fn loop_until_hit_rate_limit_sync<'a, T, Fut>(
//     object_arr: &'a [T],
//     settings: &'a Settings,
//     pool: &'a PgPool,
//     f: impl Fn(&'a Settings, &'a PgPool, &'a T) -> Fut + Copy,
//     rate_limit: usize,
// ) where
//     Fut: Future,
// {
//     let total = object_arr.len();
//     let capped_total = min(total, rate_limit);
//     for (i, object) in object_arr[..capped_total].iter().enumerate() {
//         tracing::info!(">>>I: Processing {}/{}", i + 1, total);
//         f(settings, pool, object).await;
//     }
// }
