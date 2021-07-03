# Tokio
[tokio web tutorial](https://tokio.rs/tokio/tutorial)

## async fn main()

this async fn main is with #[tokio::main] is shortened form for normal synchronous main function like  

```rust
#[tokio::main]
async fn main() {
    println!("hello");
}
```
above code converts to
```rust
fn main(){
    fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    //so code inside async fn main is just wrapped by async and spawnned on runtime so it makes a green thread which is spawnning a future on tokio executor
    rt.block_on(async {
        println!("hello");
    })
}
}
```

what's up with the await keyword  
```rust
#[tokio::main]
async fn main(){
    //this entire code is wrapped into async that is it is made a future
    //since call() itself is a future now we have two futures and we know futures can run in concurrent/parallel
    //but when we hit linw with await it pretty much says to other future that you can proceed until i finish fully
    //if call is not able to run the whole future will be kept aside and run when it is possible
    //so they are not blocking thread either/still
    call().await;
    println!("main")
}

async fn call(){
    println!("i am called");
}
```
so when futures are totally depend on other and we want to run them one after another not in parallel use await  

what if we want to run many futures/green threads in parallel we go with spawn

```rust
#[tokio::main]
async fn main() {
    //spawn takes futures and run them concurrently until or unless any synchronisation method is used
    //channel or mutex
    //if you run this all the prints can come in any order as we have 5 futures here
    tokio::spawn(async {
        println!("Hello thread")
    });
    tokio::spawn(async {
        println!("Hello thread_1")
    });
    tokio::spawn(async {
        println!("Hello thread_2")
    });
    tokio::spawn(async {
        println!("Hello thread_3")
    });
    //future 1
    println!("Hello main");
}
```

## channels
kinda same like std channels uses

## mutex
tokio::sync::Mutex;  
used just like std Mutex by taking locks for modifying some data has to be used inside Arc

## bytes crate
byte = Arc<Vec<u8>> roughly but ref counted underlying buffer
so how byte can be used to safely pass modify between threads  
TODO: how bytes help in async context ?? 

Bytes = A reference counted contiguous slice of memory. //so this will allow read access from any thread
BytesMut = A unique reference to a contiguous slice of memory.  //this will give write access 

Buf = Read bytes from a buffer.
BufMut = A trait for values that provide sequential write access to bytes.

## Stream
it is like iterator which when we call next() gives next value  
calling next() on streams will give futures  

for in loop can be called on iterator but stream as of now does not support for loop  
so they are called with while let generally


## async IO


## async tcp

### client
```rust
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let mut client = TcpStream::connect("127.0.0.1:8000").await?;

    let mut buf = [0;30];
    //read from server into buf
    client.read(&mut buf).await?;
    println!("{:?}",buf);

    Ok(())
}
```

### server
```rust
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>>{
    //connection has to be made first hence we are awaiting and other futures should not run before as connection is required first
    let mut server = TcpListener::bind("127.0.0.1:8000").await?;

    //after connection is made talking to clients is totally independent hence all the futures can be processed concurrently
    //Stream is best way to achieve this so after incoming() call we get Incoming struct implementing Stream
    //with which all clients comes as future and can be processed concurrently 
    while let Some(client) = server.incoming().next().await{
        let mut cl = client?;
        //all basic buffers creation can be used with bytes in bytes crate to modify from different green threads without putting sync logics by yourself
        //that's cool and simple for user
        let buf = b"Hello World!";
        cl.write(buf).await?;
    }

    Ok(())
}
```

