# netcat

command line tcp/udp server-client creation  
this will help in testing server/client

## installation
[installation for arch linux](https://wiki.archlinux.org/index.php/Network_tools)

## server

**nc -l -p 3000**  
-l = start listening  
-p = on this port   
no host is given means localhost(127.0.0.1)  
by default tcp server is created  

**nc -u -l -p 3000**  
-u = start a udp server rather than tcp

## client
**nc localhost 3000**  
start a client on tcp over localhost and port 3000  
**nc -u localhost 3000**   
start a client on udp over localhost and port 3000  

## how to do
open two terminals create tcp/udp server in one and same protocol  
client in another terminal if both are connected they will block in  
the terminal, and you can write any buffer on server/client side same will be  
echoed on other side  