use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

#[test]
fn client() -> io::Result<()> {
    //connect call create a tcp socket nad makes a connection request to this address
    //TODO: what address (ip + port) does the client gets I have not given any explicit address
    //I do have idea about ephemeral ports which randomly gets assigned by kernel but what about ip address
    // if we want it to give explicit address to client what we have to do
    let mut socket = TcpStream::connect("127.0.0.1:3000")?;
    //after printing I can see ip assigned to client was 127.0.0.1
    // print!("{:?}", socket);
    //write data for server if connection was successful
    socket.write(b"Hello World")?;
    //check the address given to client socket
    print!("{}", socket.local_addr().unwrap());
    Ok(())
}

#[test]
fn server() -> io::Result<()> {
    //create a tcp socket and bind the socket to this known address where other clients can connect to
    //this call creates the tcp socket and binds it to the given address
    let server_socket = TcpListener::bind("127.0.0.1:3000")?;
    //iterate over all clients accepting connection and write to them blocking for each write
    // for client_socket in server_socket.incoming() {
    //     client_socket?.write(b"Hello from tcp server")?;
    // }
    // we could have use the simple loop like this instead of iterator above
    let mut buf = [0; 100];
    loop {
        let (mut stream, _) = server_socket.accept()?;
        stream.read(&mut buf);
        stream.write(&buf)?;
    }
}
