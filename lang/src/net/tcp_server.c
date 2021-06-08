#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <stdio.h>

/*
tcp server:
1. create socket
2. bind a well known ip address to the socket
addresses can be many types - look for one which is written for ipv4 named as => struct sockaddr_in
this address needs to be converted to generic address struct => struct sockaddr
all ports and ip has to be in network byte order (big endian) - so use htons(), htonl() functions for conversion
3. bind the socket to this address
4. use listen which tells that this socket is ready to listen to other sockets
a backlash is also passed which indicates how many pending connections can be attached if we are blocked on read/write
5. loop and accept connections from any client socket 
we have to pass a address struct which accept will fill with clients address
accept will also return the client fd on which 
*/

//TODO: write error handling code
int main() {
  // create the socket
  int socket_fd = socket(AF_INET, SOCK_STREAM, 0);

  // create a ipv4 address
  struct sockaddr_in addr;
  addr.sin_addr.s_addr = INADDR_ANY;    //assign address 0.0.0.0
  addr.sin_port = htons(3000); // convert port to betwork byte order

  // bind to address 127.0.0.1:3000
  bind(socket_fd, (struct sockaddr *)&addr, sizeof(addr));

  // listen call - show willingness to connect to tcp clients
  listen(socket_fd, 128);

  // create variable to receive => client address and address length
  struct sockaddr client_addr;
  socklen_t len;

  // buffers to read/write to
  char buf[] = "Hello TCP";
  char buf_read[10];

  // loop and accept connections - write/read to clients
  while (1) {
    // accept call returns client fd which was connected
    // client address were filled in variable lient_addr
    int client_fd = accept(socket_fd, &client_addr, &len);
    write(client_fd, buf, strlen(buf));
    //FIXME: read is not working as buffer is not printing anything - is it because null byte is not there due
    // to receipt of bigger buffer getting overwritten
    read(client_fd, buf_read, strlen(buf_read));
    printf("%s",buf_read);
  }
  close(socket_fd);
}
