use std::error::Error;
use tokio::io;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

#[test]
fn channels() -> io::Result<()> {
    let (ts, mut tr) = mpsc::channel(1);
    let r = Runtime::new()?;

    r.block_on(async move {
        tokio::spawn(async move {
            println!("Hello second thread");
            ts.send(10).await.unwrap();
        });
        tr.recv().await;
        println!("Hello World");
    });
    Ok(())
}

//TODO: learn other types of channel provided by tokio
