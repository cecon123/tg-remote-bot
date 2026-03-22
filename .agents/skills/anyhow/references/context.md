# anyhow Context Trait

The `Context` trait adds human-readable context to errors.

## Methods

### `.context(value)`

Adds a static string or displayable value as context.

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    Ok(())
}
```

Error output:

```
Error: Failed to read config file

Caused by:
    No such file or directory (os error 2)
```

### `.with_context(closure)`

Adds lazily-evaluated context. Only called if the result is `Err`.

```rust
use anyhow::{Context, Result};

fn read_user(id: u64) -> Result<String> {
    std::fs::read_to_string(format!("users/{id}.json"))
        .with_context(|| format!("Failed to read user {id}"))?;
    Ok(())
}
```

## Implementations

```rust
// For Result<T, E> where E: std::error::Error + Send + Sync + 'static
impl<T, E> Context<T, E> for Result<T, E>

// For Option<T>
impl<T> Context<T> for Option<T>
```

## Option Context

Convert `None` to an error:

```rust
use anyhow::{Context, Result};

fn find_user(name: &str) -> Result<User> {
    db.query(name)
        .context("User not found")?
}
```

## Multiple Context Layers

```rust
fn load_and_parse(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Reading {path}"))?;
    
    let config: Config = toml::from_str(&content)
        .context("Parsing config")?;
    
    Ok(config)
}
```

Error chain:

```
Error: Parsing config

Caused by:
    0: Reading ./config.toml
    1: No such file or directory (os error 2)
```
