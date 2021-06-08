/*
Builder:
[Builder docs](https://docs.rs/tokio/1.5.0/tokio/runtime/struct.Builder.html)
build runtime using single thread
build runtime using multi thread
change no of worker threads
change thread stack size
run some functions automatically when thread starts and stops
 */

use std::error::Error;
use tokio::runtime::{self, Runtime};

/*
Runtime:
1. Multi thread scheduler
2. Current thread scheduler

lifetime of spawned threads:
While the Runtime is active, threads may shutdown after periods of being idle. Once Runtime is dropped,
all runtime threads are forcibly shutdown. Any tasks that have not yet completed will be dropped.

[Runtime struct docs](https://docs.rs/tokio/1.2.0/tokio/runtime/struct.Runtime.html)
TODO: can we have multiple instances of Runtime
TODO: block_on() vs spawn() method
 */

#[test]
fn multi_thread_scheduler() -> Result<(), Box<dyn Error>> {
    //this spawns multi thread scheduler
    //TODO: is it spawning threads equal to machine native cores
    let rt = Runtime::new()?;

    rt.block_on(async {
        println!("Hello World!");
        //spawning another green thread to be scheduled on the runtime
        tokio::spawn(async {
            println!("print from another green thread");
        });
    });

    //TODO: try to spawn another Runtime when 1 is still active see if error is thrown
    //ok so new runtime is created without any problem and a future is also spawned without any issue
    let new_rt = Runtime::new()?;

    new_rt.block_on(async {
        println!("Hello new runtime");
    });
    Ok(())
}

/*
this multithreaded scheduler code written above in multi_thread_scheduler() is easily written as using
#[tokio::main] macro - this will spawn multi threaded runtime

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    print!("print me");
    Ok(())
}
this gets desugared to
fn main(){
    let rt = Runtime::new()?;
    rt.block_on(
        async {
            print!("print me");
        }
    )
}
 */

#[tokio::main]
#[test]
async fn multi_thread_using_macro() -> Result<(), Box<dyn Error>> {
    println!("Hello World");
    //this future does not get spawned in a green thread - we have to explicitly call tokio::spawn()
    //isn't that would have been nice to spawn future automatically rather than calling tokio::spawn()
    async {
        print!("some future");
    };
    //this green thread runs independently of previous thread - call it parallel in simple terms
    //.await when called inside future they hold the execution of the current future till inner future is not completed
    tokio::spawn(async {
        print!("message from another green thread");
    });
    Ok(())
}

#[test]
fn current_thread_scheduler() -> Result<(), Box<dyn Error>> {
    //TODO: is current thread means single thread scheduler
    let rt = runtime::Builder::new_current_thread().build()?;
    //TODO: does block_on means block main thread - so what are other methods on runtime to spawn futures on
    rt.block_on(async {
        println!("current thread scheduler");
    });

    Ok(())
}

#[test]
fn builder_custom() -> Result<(), Box<dyn Error>> {
    //create single threaded runtime
    let single_rt = runtime::Builder::new_current_thread().build()?;

    //create multi threaded runtime
    let multi_rt = runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .thread_stack_size(3 * 1024 * 1024)
        .on_thread_start(|| {
            println!("thread started");
        })
        .on_thread_stop(|| {
            println!("thread stoped");
        })
        .build()?;
    Ok(())
}
