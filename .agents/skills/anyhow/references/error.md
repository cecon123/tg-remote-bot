# anyhow::Error

The core error type wrapping a dynamic trait object.

## Constructors

```rust
use anyhow::anyhow;

// From string
let err = anyhow!("something went wrong");

// From format string
let err = anyhow!("failed to read {path}");

// From existing error
let err = anyhow!(std::io::Error::new(...));
```

## Methods

| Method | Description |
|--------|-------------|
| `source()` | Get the underlying cause |
| `root_cause()` | Get the innermost error |
| `chain()` | Iterate all causes |
| `downcast_ref::<T>()` | Try to borrow inner as `&T` |
| `downcast::<T>()` | Try to extract inner as `T` |
| `context(msg)` | Add context |
| `with_context(f)` | Add lazy context |
| `is::<T>()` | Check if inner is `T` |

## Downcasting

```rust
use anyhow::anyhow;

#[derive(Debug)]
struct TimeoutError;
impl std::fmt::Display for TimeoutError { ... }
impl std::error::Error for TimeoutError {}

let err = anyhow!(TimeoutError);

// Check type
if err.is::<TimeoutError>() { ... }

// Borrow
if let Some(timeout) = err.downcast_ref::<TimeoutError>() { ... }

// Take ownership
let timeout: TimeoutError = err.downcast::<TimeoutError>().unwrap();
```

## Error Chain

```rust
use anyhow::{Context, Result};

fn run() -> Result<()> {
    step1().context("step 1")?;
    Ok(())
}

fn step1() -> Result<()> {
    step2().context("step 2")?;
    Ok(())
}

fn step2() -> Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "disk full"))?;
    Ok(())
}

// When error occurs:
// Error: step 1
//
// Caused by:
//     0: step 2
//     1: disk full
```

### Iterating Causes

```rust
for cause in err.chain() {
    eprintln!("cause: {cause}");
}
```

## Display vs Debug

- `Display` - Clean, chain format for end users
- `Debug` - Includes backtrace (if captured)

```rust
eprintln!("{err}");   // Clean chain
eprintln!("{err:?}");  // With backtrace
```

## From Implementations

```rust
// From any Error impl
From<E: std::error::Error + Send + Sync + 'static>

// From String
From<String>

// From &str
From<&str>
```
