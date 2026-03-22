# windows_sys::Win32::Networking::WinSock

Raw Winsock2 API for TCP/UDP networking.

## Key Types

| Type | Description |
|------|-------------|
| `SOCKET` | Socket handle (usize) |
| `SOCKADDR_IN` | IPv4 address |
| `SOCKADDR_IN6` | IPv6 address |
| `WSADATA` | Winsock initialization data |
| `ADDRINFOA` / `ADDRINFOW` | Address info for resolution |

## Core Functions

| Function | Description |
|----------|-------------|
| `WSAStartup` | Initialize Winsock |
| `WSACleanup` | Cleanup Winsock |
| `socket` | Create a socket |
| `bind` | Bind to address |
| `listen` | Start listening |
| `accept` | Accept connection |
| `connect` | Connect to remote |
| `send` / `recv` | Send/receive data |
| `sendto` / `recvfrom` | UDP send/receive |
| `closesocket` | Close socket |
| `getaddrinfo` | Resolve hostname |
| `setsockopt` | Set socket option |
| `ioctlsocket` | Control socket mode |

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `INVALID_SOCKET` | !0 | Invalid socket sentinel |
| `SOCKET_ERROR` | -1 | Error return value |
| `AF_INET` | 2 | IPv4 |
| `AF_INET6` | 23 | IPv6 |
| `SOCK_STREAM` | 1 | TCP |
| `SOCK_DGRAM` | 2 | UDP |
| `IPPROTO_TCP` | 6 | TCP protocol |
| `IPPROTO_UDP` | 17 | UDP protocol |
| `SD_BOTH` | 2 | Shutdown both |

## Example: TCP Client

```rust
use windows_sys::Win32::Networking::WinSock::*;
use std::mem::{size_of, zeroed};

unsafe {
    let mut wsa: WSADATA = zeroed();
    WSAStartup(0x0202, &mut wsa);

    let sock = socket(AF_INET as _, SOCK_STREAM as _, IPPROTO_TCP as _);

    let mut addr: SOCKADDR_IN = zeroed();
    addr.sin_family = AF_INET as _;
    addr.sin_port = 8080u16.to_be();
    addr.sin_addr.S_un.S_addr = 0x0100007F; // 127.0.0.1

    connect(
        sock,
        &addr as *const _ as *const SOCKADDR,
        size_of::<SOCKADDR_IN>() as _,
    );

    let msg = b"Hello";
    send(sock, msg.as_ptr(), msg.len() as _, 0);

    closesocket(sock);
    WSACleanup();
}
```

## Ioctl

```rust
// Set non-blocking mode
let mut mode: u32 = 1; // 1 = non-blocking
ioctlsocket(sock, FIONBIO, &mut mode);
```

## Errors

Use `WSAGetLastError()` to get the last socket error.
