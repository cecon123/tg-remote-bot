---
name: tokio
description: "Documentation for tokio crate. Keywords: async, runtime, spawn, task, sync, mpsc, oneshot, broadcast, watch, mutex, rwlock, tcp, udp, socket, sleep, timeout, interval, timer, io, net, fs, process, signal"
---

# Tokio

> **Version:** 1.50.0 | **Source:** docs.rs

## Overview

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with Rust. It provides a runtime, synchronization primitives, networking, timers, and more.

```toml
# Enable all features
tokio = { version = "1", features = ["full"] }

# Minimal features for spawning tasks and networking
tokio = { version = "1", features = ["rt", "net"] }
```

## Key Modules

### tokio::task - Asynchronous green-threads

Spawn and manage concurrent tasks.

| Function | Description |
|----------|-------------|
| `spawn` | Spawn a new async task, returns `JoinHandle` |
| `spawn_blocking` | Run blocking code on a dedicated thread pool |
| `spawn_local` | Spawn `!Send` future on `LocalSet` |
| `block_in_place` | Transition current thread to blocking (multi-thread only) |
| `yield_now` | Yield execution back to the runtime |

```rust
use tokio::task;

let handle = task::spawn(async {
    "result"
});

let result = handle.await?;
```

### tokio::sync - Synchronization primitives

**Channels:**
| Channel | Producers | Consumers | Values |
|---------|-----------|-----------|--------|
| `oneshot` | 1 | 1 | 1 |
| `mpsc` | Many | 1 | Many |
| `broadcast` | Many | Many | Many |
| `watch` | Many | Many | 1 (latest) |

**Locks:**
| Type | Description |
|------|-------------|
| `Mutex` | Async mutual exclusion (hold across `.await`) |
| `RwLock` | Multiple readers, single writer |
| `Semaphore` | Limit concurrency with permits |
| `Barrier` | Synchronize multiple tasks |
| `Notify` | Wake a single task without data |

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

let msg = rx.recv().await.unwrap();
```

### tokio::net - TCP/UDP/Unix sockets

| Type | Description |
|------|-------------|
| `TcpListener` | Accept TCP connections |
| `TcpStream` | TCP connection |
| `TcpSocket` | Configure before connecting/listening |
| `UdpSocket` | UDP socket |
| `UnixListener/Stream` | Unix domain sockets (Unix only) |

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

let listener = TcpListener::bind("127.0.0.1:8080").await?;
let (mut socket, _) = listener.accept().await?;

let mut buf = [0; 1024];
let n = socket.read(&mut buf).await?;
socket.write_all(&buf[..n]).await?;
```

### tokio::time - Timers and scheduling

| Function | Description |
|----------|-------------|
| `sleep` | Wait for a duration |
| `sleep_until` | Wait until an instant |
| `timeout` | Cancel future if it takes too long |
| `interval` | Repeatedly yield at fixed periods |
| `interval_at` | First tick at a specific instant |

```rust
use tokio::time::{sleep, timeout, Duration};

// Sleep
sleep(Duration::from_secs(1)).await;

// Timeout
let result = timeout(Duration::from_secs(5), async_op()).await?;
```

### tokio::runtime - Runtime configuration

| Type | Description |
|------|-------------|
| `Runtime` | The Tokio runtime |
| `Builder` | Configure runtime (scheduler, threads, etc.) |
| `Handle` | Reference to the runtime |

```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()?;

rt.block_on(async {
    // async code here
});
```

## Feature Flags

| Feature | Enables |
|---------|---------|
| `full` | All stable features |
| `rt` | `tokio::spawn`, current-thread scheduler |
| `rt-multi-thread` | Multi-thread work-stealing scheduler |
| `macros` | `#[tokio::main]`, `#[tokio::test]` |
| `sync` | All sync primitives |
| `net` | TCP/UDP/Unix sockets |
| `time` | Timers and intervals |
| `io-util` | AsyncRead/Write combinators |
| `io-std` | Stdin/Stdout/Stderr |
| `fs` | Async filesystem |
| `process` | Async process management |
| `signal` | Async signal handling |

## Macros

| Macro | Description |
|-------|-------------|
| `#[tokio::main]` | Async main entry point |
| `#[tokio::test]` | Async test function |
| `tokio::select!` | Race multiple futures, cancel losers |
| `tokio::join!` | Run futures concurrently, return all results |
| `tokio::try_join!` | Like join! but short-circuits on Err |
| `tokio::pin!` | Pin a value to the stack |

```rust
#[tokio::main]
async fn main() {
    // Your async code
}

tokio::select! {
    val = future1() => { /* future1 won */ }
    val = future2() => { /* future2 won */ }
}
```

## Documentation

- `./references/overview.md` - Main overview
- `./references/task.md` - Task module
- `./references/sync.md` - Sync module
- `./references/net.md` - Networking module
- `./references/time.md` - Time module
- `./references/runtime.md` - Runtime module

## Links

- [docs.rs](https://docs.rs/tokio)
- [crates.io](https://crates.io/crates/tokio)
- [Homepage](https://tokio.rs)
- [Tutorial](https://tokio.rs/tokio/tutorial)
- [GitHub](https://github.com/tokio-rs/tokio)
