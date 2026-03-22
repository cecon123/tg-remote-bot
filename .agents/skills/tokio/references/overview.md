# Tokio - Overview

An event-driven, non-blocking I/O platform for writing asynchronous applications with Rust.

## Installation

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## Core Concepts

### Async/Await
```rust
async fn fetch_data() -> Result<String, Error> {
    // async work
    Ok("data".to_string())
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
}
```

### Spawning Tasks
```rust
let handle = tokio::spawn(async {
    "result"
});

let result = handle.await?;
```

### Channels
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

let msg = rx.recv().await.unwrap();
```

### Networking
```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

let listener = TcpListener::bind("127.0.0.1:8080").await?;
let (mut socket, _) = listener.accept().await?;

let mut buf = [0; 1024];
let n = socket.read(&mut buf).await?;
socket.write_all(&buf[..n]).await?;
```

### Timers
```rust
use tokio::time::{sleep, timeout, Duration};

// Sleep for 1 second
sleep(Duration::from_secs(1)).await;

// Timeout after 5 seconds
let result = timeout(Duration::from_secs(5), async_op()).await?;
```

## Macro Usage

### tokio::main
```rust
#[tokio::main]
async fn main() {
    // async code
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // single-threaded
}
```

### tokio::select!
```rust
tokio::select! {
    val = future1() => println!("future1: {}", val),
    val = future2() => println!("future2: {}", val),
}
```

### tokio::join!
```rust
let (a, b) = tokio::join!(future1(), future2());
```

### tokio::try_join!
```rust
let (a, b) = tokio::try_join!(fallible1(), fallible2())?;
```

## Common Patterns

### Graceful Shutdown
```rust
use tokio::signal;

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = run_server() => {},
        _ = signal::ctrl_c() => {
            println!("Shutting down...");
        }
    }
}
```

### Shared State
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let state = Arc::new(Mutex::new(Vec::new()));

for _ in 0..10 {
    let state = state.clone();
    tokio::spawn(async move {
        let mut lock = state.lock().await;
        lock.push(42);
    });
}
```

### Request/Response Pattern
```rust
use tokio::sync::{mpsc, oneshot};

enum Command {
    Get {
        key: String,
        resp: oneshot::Sender<Option<String>>,
    },
}

let (tx, mut rx) = mpsc::channel(32);

tokio::spawn(async move {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get { key, resp } => {
                resp.send(Some(format!("value for {}", key))).unwrap();
            }
        }
    }
});

let (resp_tx, resp_rx) = oneshot::channel();
tx.send(Command::Get { key: "foo".into(), resp: resp_tx }).await?;
let value = resp_rx.await?;
```
