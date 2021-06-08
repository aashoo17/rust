use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt,self};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

/*
udp client:
use bind() call to create a udp socket and bind to the given address
connect() call is not required explicitly in udp as udp is connectionless
it is useful though in terms we can call send()/recv() in place of sendto()/recvfrom() which requires explicit address
loop continuously and keep on sending data to server and receive back from server
client does this infinitely 
*/
#[tokio::main]
#[test]
async fn client() -> io::Result<()> {
    //create a udp socket
    let a = UdpSocket::bind("127.0.0.1:4000").await?;
    //connect() call tells socket about peer address/remote address in udp
    a.connect("127.0.0.1:3000").await?;
    //now use regular send() and recv() calls
    let mut buf = [0; 100];
    loop{
        a.send(b"Hello World\n").await?;
        a.recv(&mut buf).await?;
    }
}
/*
udp server:
use bind() call to create udp socket and bind to given address
loop continuously
receive data from a client into a memory buffer
and write back same data as filled in buffer to same client
this servers runs infinitely and keeps doing that
*/
#[tokio::main]
#[test]
async fn server() -> io::Result<()> {
    //create a udp socket and bind it to this address
    let a = UdpSocket::bind("127.0.0.1:3000").await?;
    //create a buffer - usually we will want buffer size of mtu of network (i.e. hardware native datagram size)
    let mut buf = [0; 100];
    //tell server to receive data from clients which connects
    //recv_from() will give back a tuple which will have connecting clients address also in 2nd field
    //whatever data clients send fill in the buf
    loop {
        let (_, addr) = a.recv_from(&mut buf).await?;
        a.send_to(&buf, addr).await?;
    }
}
/*
same as previous udp server
but idea was that for responding to each client we do that in seperate green thread of tokio
for green thread creation tokio:spawn() is used
but passing the UdpSocket to tokio::spawn() will consume the socket
and next iteration of loop will throw moved data is being used
so I have created a Arc filled with socket
and on each loop create a clone of Arc and pass it to tokio::spawn() for consumption
*/
#[tokio::main]
#[test]
async fn server_async() -> io::Result<()> {
    let server_socket = UdpSocket::bind("127.0.0.1:3000").await?;
    let arc_socket = Arc::new(server_socket);
    loop {
        //TODO: is there any other way apart for using clone of Arc
        //TODO: even socket can be cloned how expensive that will if used rather than Arc cloning
        let clone_socket = Arc::clone(&arc_socket);
        tokio::spawn(async move {
            //TODO: avoid buffer allocation in each green thread
            let mut buf = [0; 100];
            let (_, addr) = clone_socket.recv_from(&mut buf).await.unwrap();
            clone_socket.send_to(&buf, addr).await.unwrap();
        });
    }
}
