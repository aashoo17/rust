use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, Mutex};

#[test]
fn mutexes() -> Result<(), Box<dyn Error>> {
    let a = Arc::new(Mutex::new(10));
    let b = Arc::clone(&a);
    let r = Runtime::new()?;
    r.block_on(async move {
        println!("Hello World");
        *b.lock().await = 30;
        tokio::spawn(async move {
            println!("2nd thread");
        });
    });

    println!("{:?}", a);
    Ok(())
}
