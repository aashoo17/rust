use std::time::{Duration, Instant, SystemTime};
use std::thread::sleep;

//duration - how seconds, millisecond etc will be counted
#[test]
fn durations(){
    let five_seconds = Duration::new(5,0);
    let second = Duration::SECOND;

    let a = Duration::from_secs(3);
    let b = Duration::from_millis(10);
    let c = Duration::from_micros(10);
    let d = Duration::from_nanos(10);

    println!("{:?}",five_seconds);
}

//measuring calendar time - SystemTime in rust
#[test]
fn system_time(){
    let a = SystemTime::now();
    println!("{:?}",a);
}

//using monotonic clock for measuring elapsed time - Instant
#[test]
fn instant(){
    //get the current monotonic instant
    let a = Instant::now();
    //make the thread sleep for 3 secs
    println!("Hello CPU time");
    //check elapsed time in secs 
    let b = a.elapsed().as_nanos();
    println!("{}",b);
} 

#[test]
fn sleep_some_time(){
    sleep(Duration::SECOND * 3);    //this could be any code and we can measure cpu time in its execution
}