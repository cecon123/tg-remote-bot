# tokio::task - Asynchronous green-threads

Tasks are lightweight, non-blocking units of execution managed by the Tokio runtime.

## Key Concepts

- **Light weight**: Creating/switching tasks has low overhead compared to OS threads
- **Cooperatively scheduled**: Tasks yield at `.await` points
- **Non-blocking**: Tasks should not perform blocking operations

## Functions

### spawn
Spawn a new asynchronous task, returns `JoinHandle<T>`.

```rust
use tokio::task;

let handle = task::spawn(async {
    "hello world!"
});

let result = handle.await?;
```

### spawn_blocking
Run blocking code on a dedicated thread pool.

```rust
let result = task::spawn_blocking(|| {
    // CPU-intensive or blocking I/O work
    42
}).await?;
```

### block_in_place
Transition current worker thread to blocking mode (multi-thread runtime only).

```rust
let result = task::block_in_place(|| {
    // blocking work
});
```

### spawn_local
Spawn `!Send` future on a `LocalSet` or `LocalRuntime`.

```rust
use tokio::task;

let local = task::LocalSet::new();
local.run_until(async {
    task::spawn_local(async {
        // !Send future
    }).await;
}).await;
```

### yield_now
Yield execution back to the runtime scheduler.

```rust
task::yield_now().await;
```

## Structs

### JoinHandle<T>
Owned permission to join on a task. Await to get result.

```rust
let handle = task::spawn(async { 42 });
let result = handle.await?;  // Result<T, JoinError>
```

Methods:
- `abort()` - Request cancellation
- `abort_handle()` - Get `AbortHandle` for later cancellation
- `is_finished()` - Check if task completed

### JoinSet
Collection of tasks spawned on the runtime.

```rust
let mut set = task::JoinSet::new();

for i in 0..10 {
    set.spawn(async move { i * 2 });
}

while let Some(result) = set.join_next().await {
    println!("Task returned: {}", result?);
}
```

### LocalSet
Execute `!Send` futures on the same thread.

```rust
let local = task::LocalSet::new();
local.spawn_local(async { /* !Send work */ });
local.await;
```

### AbortHandle
Cancel a task without awaiting its completion.

```rust
let handle = task::spawn(async { /* long work */ });
let abort = handle.abort_handle();

// Later...
abort.abort();
```

## Cancellation

Tasks can be cancelled via `JoinHandle::abort()` or `AbortHandle::abort()`.

- Task stops at the next `.await` point
- `spawn_blocking` tasks cannot be aborted after starting
- Awaiting cancelled task returns `JoinError` with `is_cancelled()` = true
