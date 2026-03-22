# nokhwa::error

> Reference documentation for error types

## NokhwaError

All errors in nokhwa are represented by this enum.

```rust
use nokhwa::NokhwaError;
```

### Variants

| Variant | Fields | Description |
|---------|--------|-------------|
| `UnitializedError` | - | Not initialized (macOS) |
| `InitializeError` | `backend, error` | Backend initialization failed |
| `ShutdownError` | `backend, error` | Backend shutdown failed |
| `GeneralError` | `error` | General error |
| `StructureError` | `structure, error` | Data structure issue |
| `OpenDeviceError` | `device, error` | Cannot open device |
| `GetPropertyError` | `property, error` | Cannot get property |
| `SetPropertyError` | `property, value, error` | Cannot set property |
| `OpenStreamError` | `error` | Cannot open stream |
| `ReadFrameError` | `error` | Cannot read frame |
| `ProcessFrameError` | `src, destination, error` | Frame processing failed |
| `StreamShutdownError` | `error` | Stream shutdown failed |
| `UnsupportedOperationError` | `backend` | Operation not supported |
| `NotImplementedError` | `error` | Feature not implemented |

### Error Handling Patterns

```rust
use nokhwa::{Camera, NokhwaError};

match camera.frame() {
    Ok(buffer) => { /* process */ }
    Err(NokhwaError::ReadFrameError(e)) => {
        eprintln!("Frame read failed: {}", e);
    }
    Err(NokhwaError::StreamShutdownError(e)) => {
        eprintln!("Stream closed: {}", e);
    }
    Err(e) => {
        eprintln!("Other nokhwa error: {}", e);
    }
}
```

### Common Error Scenarios

| Scenario | Likely Error |
|----------|--------------|
| Camera in use | `OpenDeviceError` |
| Permission denied | `OpenStreamError` or `InitializeError` |
| Device disconnected | `ReadFrameError` |
| Unsupported format | `UnsupportedOperationError` |
| Wrong platform | `InitializeError` |
| macOS uninitialized | `UnitializedError` |

### Trait Implementations

- `Clone` - Can be cloned
- `Debug` - Debug formatting
- `Display` - Human-readable message
- `Error` - Standard Error trait
- `Send` + `Sync` - Thread-safe
