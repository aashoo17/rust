# hyper

## Generic http
Request, Response, Header, Body (having data moving from one server to other)
















## making a get request and writing the response to a file

```rust
use hyper::Client;
//bringing stream into scope to use body as stream and calling next() on it
use futures::stream::StreamExt;
//hyper body works with bytes so bytes crate is brought in scope to manipulate the bytes to [u8]
use bytes::Buf;
use std::fs::File;
use std::io::Write;

//this macro puts async main (which is a future) on tokio executor to be run
//make sure that in cargo.toml you use macro feature of tokio like below
//tokio = { version = "0.2.9", features = ["macros"] }
#[tokio::main]
//main returns a result as we dont want to handle errors if any inside main func and just return it with ?
//since error can be of any type we just reurn std::error::Error trait object which can work like any error
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    //use parse func to convert &str to Uri type
    let uri = "http://jsonplaceholder.typicode.com/photos".parse()?;
    //call async get func and await it for completion
    let mut res = Client::new().get(uri).await?;
    //get the body stream as mutable
    let body = res.body_mut();
    //create a file where incoming data will be written
    //TODO: use tokio's async read/write instead 
    let mut f = File::create("str.txt")?;
    //loop over stream which returns future and await them
    while let Some(data) = body.next().await{
        let d = data?;
        //use bytes method to get underlying [u8]
        let b = d.bytes();
        //write() call to write it to file
        f.write(b)?;
    }
    //if no error during main execution return Ok() with unit tuple to ensure return of a Result<> from main() is satisfied
    Ok(())
}
```
New way of making requests in hyper

```rust
use hyper::Client;
//Response body implements HttpBodt trait
use hyper::body::HttpBody;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let client = Client::new();
    let uri = "http://jsonplaceholder.typicode.com/users".parse()?;
    let mut res = client.get(uri).await?;

    while let Some(body) = res.body_mut().data().await{
        println!("{:?}",body);
    }

    Ok(())
}
```