/*
Builder:
[Builder docs](https://docs.rs/tokio/1.2.0/tokio/runtime/struct.Builder.html)
build runtime using single thread
build runtime using multi thread
change no of worker threads
change thread stack size
run some functions automatically when thread starts and stops
 */

use tokio::runtime;

pub fn builder_custom() -> Result<(), Box<dyn std::error::Error>>{
    //create signle threaded runtime
    let single_rt = runtime::Builder::new_current_thread().build()?;

    //create multi threaded runtime
    let multi_rt = runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .thread_stack_size(3* 1024* 1024)
        .on_thread_start(|| {
            println!("thread started");
        })
        .on_thread_stop(||{
            println!("thread stoped");
        })
        .build()?;

    Ok(())
}

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
use tokio::runtime::Runtime;
use std::error::Error;

pub fn multithread_scheduler() -> Result<(),Box<dyn Error>>{
    //this spawns multi thread scheduler
    let rt = Runtime::new()?;

    //TODO: how to spawn another green thread and scedue it on runtime
    rt.block_on(async{
        println!("Hello World!");
        //spawning another green thread to be scheduled on the runtime
        tokio::spawn(async{
            println!("print from another green thread");
        });
    });

    //TODO: try to spawn another Runtime when 1 is still active see if error is thrown
    //ok so new runtime is created without any problem and a future is also spawned without any issue
    let new_rt = Runtime::new()?;

    new_rt.block_on(async{
        println!("Hello new runtime");
    });
    Ok(())
}

/*
this multithreaded schedular code written above in multithread_scheduler() is easily written as using
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

pub fn currentthread_scheduler() -> Result<(),Box<dyn Error>>{
    let rt = runtime::Builder::new_current_thread().build()?;
    rt.block_on(async{
        println!("current thread scheduler");
    });

    Ok(())
}