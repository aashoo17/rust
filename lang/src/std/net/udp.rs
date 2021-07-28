use std::error::Error;
use std::io;
use std::net::UdpSocket;

/*
udp client:
bind() call creates a udp socket and binds to known address here localhost:4000
I have made a explicit call to connect here this is just a helping function so that I can call
send() and recv() in place of send_to() or recv_from() which takes explicit address
since we pass address in connect already send() and recv() call will use that address
 */

#[test]
fn client() -> io::Result<()> {
    //create a udp client socket on this address
    let a = UdpSocket::bind("127.0.0.1:4000")?;
    //connect() call tells socket about peer server address/remote address in udp to connect to by default
    // this will let us use send()/recv() in place of send_to()/recv_from() which takes explicit server address
    a.connect("127.0.0.1:3000")?;
    //now use regular send() and recv() calls
    a.send(b"Hello World")?;
    let mut buf = [0u8; 100];
    a.recv(&mut buf)?;
    Ok(())
}

/*
udp server:
bind() call creates a udp socket and binds to known address here localhost:3000
after bind() call we can send/receive the data from any udp client using send_to() and recv_from() - in these calls we do have to pass address of the clients
both send_to() and recv_from() calls are blocking by default
I have tried to receive data from a client into a buffer (memory on stack in array form here)
if any client tries to send data to this udp server in recv_from() apart from data we will get its address also
then i have tried to send some data to that client of which I know the address now
 */

#[test]
fn server() -> io::Result<()> {
    //create a udp socket and bind it to this address
    let a = UdpSocket::bind("127.0.0.1:3000")?;
    //create a buffer - usually we will want buffer size of mtu of network (i.e. hardware native datagram size)
    let mut buf = [0; 100];
    //tell server to receive data from clients which connects
    //recv_from() will give back a tuple which will have connecting clients address also in 2nd field
    //whatever data clients send fill in the buf
    loop {
        let b = a.recv_from(&mut buf)?;
        a.send_to(&buf, b.1)?;
        //clear the buffer otherwise some part of old value will reflect
        buf.fill_with(||{0})
    }
}
