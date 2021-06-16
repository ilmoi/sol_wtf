// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;
//
// use clokwerk::{AsyncScheduler, Scheduler, TimeUnits};
// use sqlx::PgPool;
//
// use crate::config::Settings;
// use crate::twitter::routes::pull::{loop_until_hit_rate_limit, process_user_timeline};
// use crate::twitter::scrapers::v2_api::fetch_all_followed_users;
//
// /// NOTE this function is NOT async, so we don't .await on it
// pub fn clokwerk_async_multithreaded_scheduler(pg_pool: Arc<PgPool>, config: Arc<Settings>) {
//     thread::spawn(move || {
//         let rt = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//
//         rt.block_on(async {
//             let mut scheduler = AsyncScheduler::new();
//             println!("==== SCHEDULER thread id is {:?}", thread::current().id());
//
//             scheduler.every(10.seconds()).run(move || {
//                 let arc_pool = pg_pool.clone();
//                 let arc_config = config.clone();
//                 async {
//                     println!("working!");
//                     println!("==== TASK thread id is {:?}", thread::current().id());
//                     // pull_from_main(arc_pool, arc_config).await;
//                 }
//             });
//
//             loop {
//                 scheduler.run_pending().await;
//                 tokio::time::sleep(Duration::from_millis(100)).await;
//             }
//         });
//     });
// }
//
// pub async fn pull_from_main(pool: Arc<PgPool>, config: Arc<Settings>) {
//     let config = config.as_ref();
//     let pool = pool.as_ref();
//
//     let (users, _) = fetch_all_followed_users(&config).await.unwrap();
//     let users = &users[..2]; //todo change for testing
//
//     loop_until_hit_rate_limit(&users, &config, &pool, process_user_timeline, 1500).await;
//     // loop_until_hit_rate_limit_sync(&users, config, pool.as_ref(), process_user_timeline, 1500).await;
// }
