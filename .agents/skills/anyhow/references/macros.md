# anyhow Macros

## `anyhow!` (also available as `format_err!`)

Construct an ad-hoc error from a string or existing error value.

```rust
use anyhow::anyhow;

// String interpolation
return Err(anyhow!("Missing key: {key}"));

// Wrap existing error
let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
let err: anyhow::Error = anyhow!(io_err);

// Wrap with context
let err = anyhow!(io_err).context("Failed to save file");
```

## `bail!`

Early return with an error. Shorthand for `return Err(anyhow!(...))`.

```rust
use anyhow::{bail, Result};

fn process(value: i32) -> Result<()> {
    if value < 0 {
        bail!("Value must be non-negative, got {value}");
    }
    // continues here only if value >= 0
    Ok(())
}
```

Equivalent to:

```rust
if value < 0 {
    return Err(anyhow!("Value must be non-negative, got {value}"));
}
```

## `ensure!`

Early return if condition is false. Like `assert!` but returns `Err`.

```rust
use anyhow::{ensure, Result};

fn divide(a: f64, b: f64) -> Result<f64> {
    ensure!(b != 0.0, "Division by zero: {a} / {b}");
    Ok(a / b)
}
```

With formatted message:

```rust
ensure!(
    users.len() > 0,
    "Expected at least 1 user, got {}",
    users.len()
);
```

## Comparison

| Macro | Purpose | Equivalent |
|-------|---------|------------|
| `anyhow!("msg {x}")` | Create error | `Err(anyhow::Error::from(format!("msg {x}")))` |
| `bail!("msg {x}")` | Return error | `return Err(anyhow!("msg {x}"))` |
| `ensure!(cond, "msg")` | Guard | `if !cond { bail!("msg") }` |
