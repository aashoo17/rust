use tokio::io::{AsyncReadExt, AsyncWriteExt, self};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

/*
tcp client:
TcpStream emulates the tcp socket in rust
connect() call will try to connect to tcp server socket located at that address
then we loop and
keep on writing and reading to and from tcp server in a buffer 
*/
#[tokio::main]
#[test]
async fn client() -> io::Result<()> {
    //connect to this address
    let mut a = TcpStream::connect("127.0.0.1:3000").await?;
    let mut buf = [0;100];
    //loop and write and read continuosly from server
    loop{
        //write this to server
        a.write(b"Hello World\n").await?;
        //read from server and keep in buf
        a.read(&mut buf).await?;
        //print the buffer
        println!("{:?}",buf);
    }
}

/*
tcp server:
TcpListener emulates the tcp server socket
bind() call does three steps
1. create tcp socket
2. bind to given address
3. make socket to listen any incoming connection
then we loop continuously doing these steps
1. accept the connection from any tcp client
2. read data from the client into a buffer
3. write same data on the client socket 
*/

#[tokio::main]
#[test]
async fn server() -> io::Result<()> {
    //create tcp server
    let server = TcpListener::bind("127.0.0.1:3000").await?;
    //iterate over all clients and write to them blocking for each write
    //Todo: see if the stream api can be used
    let mut buf = [0; 100];
    loop {
        let (mut client, _) = server.accept().await?;
        client.read(&mut buf).await?;
        client.write(&buf).await?;
    }
}
/*
motive of this code:
create a server socket 
in a loop do the following
accept the connection from any client socket
read data from the client socket
and send the client address over a channel to another green thread (tokio::spwan())
the purpose of this new green thread is to write the data to received client address
so we are using two green threads 
1. first one accepts the connection and reads the data
2. 2nd green thread will wait over channel for receipt of client details and will do the writing

explaining the code:
TcpListener's bind() call will bind the socket to given address and wait for any client connection to talk to
a channel is create whose purpose is to talk to two green threads
we spawn a green thread with tokio::spwan() in which we wait on channel to receive the data
and once data is received on the channel which is a client socket we will write on this socket
then we loop and insdie loop do the following
accept connection from any client socket
read data from that client
and then send this client socket (TcpStream) over channel to previous green thread

*/
//using channels for sync
#[tokio::main]
#[test]
async fn server_async() -> io::Result<()> {
    //create
    let server_socket = TcpListener::bind("127.0.0.1:3000").await?;
    let (ts, mut tr) = mpsc::channel::<TcpStream>(1000);
    //create a green thread for read/write to client socket - synchronised using channels
    tokio::spawn(async move {
        let mut client = tr.recv().await.unwrap();
        client.write(b"Hello from tokio tcp").await.unwrap();
    });

    let mut buf = [0; 100];
    loop {
        let (mut client, _) = server_socket.accept().await?;
        client.read(&mut buf).await?;
        ts.send(client).await.unwrap();
    }
}
/*
logic of code:
the best way to utilise the green thread would be if we spawn 
an individual green thread for handling (reading and writing data) each client
so after accept call is made spawn a green thread which will do the reading & writing on the client
I think this is the best possible scenario for a server using green thread

code explanation:
TcpListener bind() call will create a socket bound to this given address and waiting for any client connect
inside a loop doing the following
accepting the connection from client socket
spwan a green thread which consumes the client(TcpStream)
create a buffer inside green thread
read from the client socket
write to the socket after reading
and again wait for accepting another connection and do the same thing again
*/
//spawn a green thread for each client and process them in there
#[tokio::main]
#[test]
async fn green_thread_for_each_client() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    loop {
        let (mut client_socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            //TODO: avoid this allocation of buffer for each green thread and use 1 allocation possibly use byte crate
            let mut buf = [0; 100];
            client_socket.read(&mut buf).await.unwrap();
            client_socket.write(&buf).await.unwrap();
        });
    }
}
