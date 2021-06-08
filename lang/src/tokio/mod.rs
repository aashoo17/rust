/*
Green Threads:
TODO: put a link to good green thread explanation
executors/runtime:
this can be think of a software program written to abstract threads where something will run
threads -> executors -> your code to be run on threads
so what and when code will go on threads will be decided by executors
now executors themselves can be single threaded or multi threaded what that means is
single threaded executor - will put all task on same thread
multi threaded executor - will/can put on many threads
TODO: explain #[tokio::main] - one way which we can create executor and put code on it to run
TODO: so different ways to create executors in tokio
futures and async:
so future is a piece of code which will never run until it is put on executors which will run that code (or in
other words say bring futures to completion)
creating futures usually is pain in the ass
async simplifies that
async {} - async block
async fn name(){} - async func
these two will emit the futures with the code written in the async block or async function
tokio::spawn():
this is the way can spawn our green threads in tokio
main() thread:
when running inside main any green threads spawned are independently run but when main will terminate it
will cleanup all green threads either complete or not
.await:
any future put on thread::spawn() starts executing immediately
if we want that we should wait for a green thread we have to call .await on the thread
this will be usually used in main() to stop it from completing before other green threads are completed
Mutex:
we can use tokio::sync::Mutex to share data between multiple threads for synchronised writing to it
bytes crate:
this can be think of Arc<Vec<T>>
so Arc based means it can be passed around in diffeent threads without any problem
Vec type means it can grow and shrink as required and
this always does shallow copying means when cloned underlying data is shared not explicitly copied
channel:
channel is used as message passing concept
channel has two part
1. sender
2. receiver
it works like that say from green thread A sender sends something and in green thread B receiver of same
channel tries to receive then thread B will not execute further any code until channel receiver gets data
from the channel sender.
TODO: show the channel working in code
IO:
AsyncWriteExt and AsyncReadExt are two traits which will provide write and read access to anybody who will
implement it
they are implemented generally for somebody which work in green thread environment
file writes - tokio::fs::File
network writes etc - tokio::net::TcpStream
streams:
so streams are like rust iterators
iterators - they keep on giving/emitting values when next() is called on it
streams - they keep on giving futures when next() is called, then if we want we can execute these futures
TODO: improve the below analogy
e.g. say i am network server many clients keep on connecting to me, each clients request can be made as
futures to be run
now stream lets you do this when call next() gives one client as future then next client and so on....
so streams once implemented user will have easy time to work on these futures instead of creating from ground up
 */

/*
runtime::currentthread_scheduler();
runtime::multithread_scheduler();
runtime::builder_custom();

use this to generate multithreaded runtime using macro

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    fut().await;

    Ok(())
}

async fn fut(){
    println!("a future generated by async");
}
*/
mod byte;
mod channels;
mod io;
mod mutex;
mod net;
mod runtime;