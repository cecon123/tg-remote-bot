# tokio::runtime - The Tokio runtime

Runtime for executing asynchronous tasks.

## Quick Start

### Using the macro (recommended)

```rust
#[tokio::main]
async fn main() {
    // Your async code here
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Single-threaded runtime
}
```

### Manual runtime creation

```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        // Your async code here
    });
}
```

## Runtime Types

### Runtime
The main runtime instance.

```rust
let rt = Runtime::new()?;
rt.block_on(async { /* work */ });
```

### Builder
Configure runtime before building.

```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("my-worker")
    .thread_stack_size(3 * 1024 * 1024)
    .enable_io()
    .enable_time()
    .build()?;
```

### Handle
Reference to the runtime (can be cloned and shared).

```rust
use tokio::runtime::Handle;

let handle = Handle::current();
handle.spawn(async { /* work */ });
```

## Scheduler Flavors

### Multi-thread (default)
Work-stealing thread pool. Best for most applications.

```rust
Builder::new_multi_thread()
    .worker_threads(4)  // default: num CPUs
    .build()
```

### Current-thread
Single-threaded executor. Good for `!Send` futures.

```rust
Builder::new_current_thread()
    .build()
```

## Builder Options

| Method | Description |
|--------|-------------|
| `worker_threads(n)` | Number of worker threads |
| `thread_name(name)` | Thread name prefix |
| `thread_stack_size(size)` | Stack size per thread |
| `enable_io()` | Enable I/O driver |
| `enable_time()` | Enable timer driver |
| `enable_all()` | Enable both I/O and time |
| `global_queue_interval(n)` | Check global queue every N polls |
| `event_interval(n)` | Check events every N polls |
| `max_blocking_threads(n)` | Max blocking threads |
| `on_thread_start(f)` | Callback when thread starts |
| `on_thread_stop(f)` | Callback when thread stops |
| `on_thread_park(f)` | Callback when thread parks |
| `on_thread_unpark(f)` | Callback when thread unparks |

## Runtime Behavior

### Task Scheduling
- Tasks are scheduled fairly (no starvation guarantee)
- Multi-thread: work-stealing between workers
- Current-thread: global + local queues

### Blocking Operations
- Use `spawn_blocking()` for blocking I/O or CPU-heavy work
- Blocking tasks run on a separate thread pool
- Default max blocking threads: 512

### Shutdown
- Runtime shuts down when dropped
- In-progress tasks may be cancelled
- Use `shutdown_timeout()` to wait for graceful shutdown

```rust
use tokio::runtime::Builder;
use std::time::Duration;

let rt = Builder::new_multi_thread()
    .enable_all()
    .build()?;

// Graceful shutdown
rt.shutdown_timeout(Duration::from_secs(30));
```
