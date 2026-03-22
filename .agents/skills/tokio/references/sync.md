# tokio::sync - Synchronization primitives

Runtime-agnostic synchronization primitives for async contexts.

## Channels

### oneshot - Single value channel
One producer, one consumer, one value.

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    tx.send("result").unwrap();
});

let msg = rx.await?;
```

### mpsc - Multi-producer, single-consumer
Many producers, one consumer, many values.

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);  // buffer size 100

// Clone tx for multiple producers
let tx2 = tx.clone();

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

while let Some(msg) = rx.recv().await {
    println!("Received: {}", msg);
}
```

### broadcast - Multi-producer, multi-consumer
Many producers, many consumers, each consumer sees every value.

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tx.send(10).unwrap();

// Both rx1 and rx2 receive 10
```

### watch - Latest value only
Many producers, many consumers, only latest value retained.

```rust
use tokio::sync::watch;

let (tx, mut rx) = watch::channel("initial");

tokio::spawn(async move {
    if rx.changed().await.is_ok() {
        let val = *rx.borrow_and_update();
        println!("New value: {}", val);
    }
});

tx.send("updated").unwrap();
```

## Locks

### Mutex - Async mutual exclusion
Hold across `.await` points.

```rust
use tokio::sync::Mutex;

let data = Mutex::new(0);

let mut lock = data.lock().await;
*lock += 1;
// lock dropped here
```

### RwLock - Reader-writer lock
Multiple readers OR single writer.

```rust
use tokio::sync::RwLock;

let lock = RwLock::new(0);

// Multiple readers
let val = *lock.read().await;

// Single writer
let mut writer = lock.write().await;
*writer += 1;
```

### Semaphore - Permit-based concurrency limit

```rust
use tokio::sync::Semaphore;

let sem = Arc::new(Semaphore::new(3));  // max 3 concurrent

let permit = sem.acquire().await?;
// ... do work ...
drop(permit);
```

### Barrier - Synchronize multiple tasks

```rust
use tokio::sync::Barrier;

let barrier = Arc::new(Barrier::new(10));  // 10 tasks

// In each task:
barrier.wait().await;
// All tasks resume together
```

### Notify - Simple task notification

```rust
use tokio::sync::Notify;

let notify = Arc::new(Notify::new());

// Task 1: wait
notify.notified().await;

// Task 2: notify
notify.notify_one();
```

## Guards (RAII)

| Guard | Lock Type |
|-------|-----------|
| `MutexGuard` | Mutex (borrowed) |
| `OwnedMutexGuard` | Mutex (owned) |
| `RwLockReadGuard` | RwLock (read) |
| `RwLockWriteGuard` | RwLock (write) |
| `SemaphorePermit` | Semaphore (borrowed) |
| `OwnedSemaphorePermit` | Semaphore (owned) |
