---
name: anyhow
description: "Documentation for anyhow crate. Keywords: error, error-handling, result, context, bail, ensure, anyhow, backtrace, downcast, chain, error-type, application-error"
---

# anyhow

> **Version:** 1.0.102 | **Source:** docs.rs

## Overview

Flexible concrete error type built on `std::error::Error`. Uses trait objects for easy error handling in applications (not libraries).

```toml
[dependencies]
anyhow = "1"
```

## Quick Start

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config")?;
    Ok(())
}
```

## Key Types

| Type | Description |
|------|-------------|
| `anyhow::Error` | Dynamic error wrapper |
| `anyhow::Result<T>` | Alias for `Result<T, anyhow::Error>` |
| `anyhow::Chain` | Iterator over error source chain |

## Macros

| Macro | Usage | Description |
|-------|-------|-------------|
| `anyhow!(...)` | `Err(anyhow!("msg"))` | Create ad-hoc error |
| `bail!(...)` | `bail!("msg")` | Early return with error |
| `ensure!(cond, ...)` | `ensure!(x > 0, "bad")` | Assert with error |

## Context Trait

| Method | When to use |
|--------|-------------|
| `.context(msg)` | Add static string context |
| `.with_context(\|\| format!(...))` | Add lazy-computed context |

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("Failed to read config")  // static
}

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {path}"))  // lazy
}
```

## Downcasting

```rust
use anyhow::anyhow;

#[derive(Debug)]
struct MyError { code: u32 }
impl std::fmt::Display for MyError { ... }
impl std::error::Error for MyError {}

let err: anyhow::Error = anyhow!(MyError { code: 42 });

// By reference
if let Some(e) = err.downcast_ref::<MyError>() {
    eprintln!("code: {}", e.code);
}

// By value
let e: MyError = err.downcast::<MyError>()?;
```

## Error Chains

```rust
use anyhow::Context;

fn run() -> anyhow::Result<()> {
    step1().context("step1 failed")?;
    step2().context("step2 failed")?;
    Ok(())
}

// Iterate causes
for cause in err.chain() {
    eprintln!("caused by: {cause}");
}
```

## Backtrace (Rust >= 1.65)

Automatically captured when `RUST_LIB_BACKTRACE=1` or `RUST_BACKTRACE=1`.

```bash
RUST_LIB_BACKTRACE=1 cargo run
```

## When to Use anyhow vs thiserror

| Use | When |
|-----|------|
| `anyhow` | Applications, binaries, tests - don't need programmatic matching |
| `thiserror` | Libraries - expose typed errors for callers to match |

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | Yes | Use std (disable for no_std) |
| `backtrace` | No | Capture backtraces |

## No-std

```toml
[dependencies]
anyhow = { version = "1", default-features = false }
```

## Documentation

- `./references/macros.md` - anyhow!, bail!, ensure!
- `./references/context.md` - Context trait details
- `./references/error.md` - Error type methods

## Links

- [docs.rs](https://docs.rs/anyhow)
- [crates.io](https://crates.io/crates/anyhow)
- [GitHub](https://github.com/dtolnay/anyhow)
