# tokio::net - Networking types

TCP/UDP/Unix socket bindings for async I/O.

## TCP

### TcpListener
Accept incoming TCP connections.

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

let listener = TcpListener::bind("127.0.0.1:8080").await?;

loop {
    let (mut socket, addr) = listener.accept().await?;
    
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => return,  // connection closed
                Ok(n) => {
                    socket.write_all(&buf[..n]).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
        }
    });
}
```

### TcpStream
Connected TCP stream.

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

stream.write_all(b"hello").await?;

let mut buf = [0; 1024];
let n = stream.read(&mut buf).await?;
```

### TcpSocket
Configure socket before connecting/listening.

```rust
use tokio::net::TcpSocket;

let socket = TcpSocket::new_v4()?;
socket.set_reuseaddr(true)?;
socket.set_nodelay(true)?;

// For client
let stream = socket.connect("127.0.0.1:8080".parse()?).await?;

// For server
socket.bind("127.0.0.1:8080".parse()?)?;
let listener = socket.listen(128)?;
```

## UDP

### UdpSocket
UDP socket for datagram communication.

```rust
use tokio::net::UdpSocket;

let socket = UdpSocket::bind("127.0.0.1:8080").await?;

// Send
socket.send_to(b"hello", "127.0.0.1:8081").await?;

// Receive
let mut buf = [0; 1024];
let (len, addr) = socket.recv_from(&mut buf).await?;

// Connect for send/recv without specifying address each time
socket.connect("127.0.0.1:8081").await?;
socket.send(b"hello").await?;
let len = socket.recv(&mut buf).await?;
```

## Unix Sockets (Unix only)

### UnixListener / UnixStream / UnixDatagram
Similar to TCP/UDP but for local IPC.

```rust
use tokio::net::UnixListener;

let listener = UnixListener::bind("/tmp/mysocket")?;

let (mut stream, _) = listener.accept().await?;
```

## Utility

### ToSocketAddrs trait
Resolve addresses without blocking.

```rust
use tokio::net::ToSocketAddrs;

let addrs = "www.example.com:80".to_socket_addrs()?;
```

### lookup_host
DNS resolution.

```rust
use tokio::net::lookup_host;

let addrs = lookup_host("www.example.com:80").await?;
for addr in addrs {
    println!("{}", addr);
}
```
