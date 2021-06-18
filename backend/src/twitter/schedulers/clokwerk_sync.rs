// ----------------------------------------------------------------------------- saved for personal ref

// use clokwerk::{AsyncScheduler, Scheduler, TimeUnits};
// use std::thread;
// use std::time::Duration;
//
// pub fn clokwerk_sync_scheduler() {
//     let handle = thread::spawn(|| {
//         let mut scheduler = Scheduler::new();
//
//         scheduler.every(5.seconds()).run(|| {
//             println!("==== thread id is {:?}", thread::current().id());
//             println!("working!")
//         });
//
//         loop {
//             scheduler.run_pending();
//             thread::sleep(Duration::from_millis(10));
//         }
//     });
//
//     // handle.join().unwrap(); DO NOT JOIN!!!! OR IT WILL BLOCK THE SERVER
// }
