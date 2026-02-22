// use std::time::{Duration, Instant, SystemTime};
// use std::thread::sleep;
//
// //Duration - this struct keeps track of seconds and nanosecond
// #[test]
// fn durations(){
//     let five_seconds = Duration::new(5,0);  //5 sec and 0 nano
//     //const variables
//     let second = Duration::SECOND;      // 1 second
//
//     //associated functions
//     let a = Duration::from_secs(3);
//     let b = Duration::from_millis(10);
//     let c = Duration::from_micros(10);
//     let d = Duration::from_nanos(10);
//
//     println!("{:?} {:?} {:?} {:?}",a,b,c,d);
// }
//
// //measuring calendar time - SystemTime in rust
// #[test]
// fn system_time(){
//     let a = SystemTime::now();
//     println!("{:?}",a);
//     //todo: convert to calendar time - in GMT and local time zone
// }
//
// //using monotonic clock for measuring elapsed time - Instant
// #[test]
// fn instant(){
//     //get the current monotonic instant
//     let a = Instant::now();
//     println!("{:?}",a);
//     //make the thread sleep for 3 secs
//     println!("Hello elapsed time");
//     //check elapsed time in secs
//     let b = a.elapsed();
//     println!("{:?}",b);
// }
//
// //todo: CPU time
// //todo: alarm signal handling
//
// #[test]
// fn sleep_some_time(){
//     sleep(Duration::SECOND * 3);
// }