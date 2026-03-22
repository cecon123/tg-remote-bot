# tokio::time - Timers and scheduling

Utilities for tracking time and scheduling work.

## Functions

### sleep
Wait for a duration to elapse.

```rust
use tokio::time::{sleep, Duration};

sleep(Duration::from_secs(1)).await;
println!("1 second elapsed");
```

### sleep_until
Wait until a specific instant.

```rust
use tokio::time::{sleep_until, Instant};

let deadline = Instant::now() + Duration::from_secs(5);
sleep_until(deadline).await;
```

### timeout
Require a future to complete within a duration.

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), async_op()).await {
    Ok(result) => println!("Completed: {:?}", result),
    Err(_) => println!("Timed out"),
}
```

### timeout_at
Like `timeout` but with an `Instant` deadline.

```rust
use tokio::time::{timeout_at, Instant};

let deadline = Instant::now() + Duration::from_secs(5);
timeout_at(deadline, async_op()).await?;
```

### interval
Repeatedly yield at fixed periods.

```rust
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_secs(1));

loop {
    interval.tick().await;
    println!("Tick!");
}
```

**Note:** First tick completes immediately. Use `interval_at` to delay first tick.

### interval_at
Like `interval` but first tick at a specific instant.

```rust
use tokio::time::{interval_at, Instant, Duration};

let start = Instant::now() + Duration::from_secs(1);
let mut interval = interval_at(start, Duration::from_secs(1));

// First tick after 1 second
interval.tick().await;
```

## Structs

### Sleep
Future returned by `sleep` and `sleep_until`.

### Interval
Stream returned by `interval` and `interval_at`.

Methods:
- `tick()` - Wait for next tick
- `reset()` - Reset the interval
- `set_missed_tick_behavior()` - Configure behavior when ticks are missed

### Timeout
Future returned by `timeout` and `timeout_at`.

### Instant
Monotonically nondecreasing clock measurement.

```rust
use tokio::time::Instant;

let start = Instant::now();
// ... do work ...
let elapsed = start.elapsed();
```

## MissedTickBehavior

Configure what happens when `Interval::tick()` is called late:

| Variant | Behavior |
|---------|----------|
| `Burst` | Tick immediately as many times as needed |
| `Delay` | Reset interval from now |
| `Skip` | Skip missed ticks, next tick on schedule |

```rust
use tokio::time::{interval, MissedTickBehavior};

let mut interval = interval(Duration::from_secs(1));
interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
```

## Test Utilities (requires `test-util` feature)

- `pause()` - Pause time for testing
- `resume()` - Resume time
- `advance(duration)` - Advance time by a duration

```rust
#[tokio::test]
async fn test_with_time() {
    tokio::time::pause();
    
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        "done"
    });
    
    tokio::time::advance(Duration::from_secs(10)).await;
    
    assert_eq!(handle.await.unwrap(), "done");
}
```
