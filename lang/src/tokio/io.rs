use std::error::Error;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
#[test]
async fn read() -> io::Result<()> {
    let mut f = File::open("file.txt").await?;
    let mut buf = [0u8; 100];
    f.read(&mut buf).await?;
    print!("{:?}", buf);
    Ok(())
}

#[tokio::main]
#[test]
async fn write() -> io::Result<()> {
    let mut f = File::create("file.txt").await?;
    f.write(b"Hello from tokio").await?;
    Ok(())
}

#[tokio::main]
#[test]
async fn custom() -> io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open("file.txt")
        .await?;
    f.write(b"Hello from tokio again").await?;
    Ok(())
}
