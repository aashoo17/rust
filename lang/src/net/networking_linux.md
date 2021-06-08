# Networking

## socket creation
socket creation usually requires these  

**domain**  
1. local on same computer
2. internet over IPv4
3. internet over IPv6

**type** 
1. byte stream
2. datagrams

# TCP 

## Server
1. socket creation
2. bind the socket to an address
3. listen - socket will start listening for other client/socket connections
4. accept the connection
5. read/write
6. close the connection
## Client
1. socket creation
2. connect to server socket on some address
3. read/write
4. close the socket

TODO: show how two tcp connected computer look at kernel level

# UDP

## Server

## Client

MTU


## What parts can be independent in tcp/udp server/client model and how async can help in networking
