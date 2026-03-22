# windows_service::service_dispatcher

Service control dispatcher entry point.

## Functions

| Function | Description |
|----------|-------------|
| `start()` | Register service entry function and block until service stops |

## Usage

### Basic Entry Point

```rust
#[macro_use]
extern crate windows_service;

use windows_service::service_dispatcher;

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<std::ffi::OsString>) {
    // Service implementation
}

fn main() -> windows_service::Result<()> {
    service_dispatcher::start("myservice", ffi_service_main)?;
    Ok(())
}
```

### Error Handling

```rust
fn main() {
    if let Err(e) = service_dispatcher::start("myservice", ffi_service_main) {
        eprintln!("Failed to start service dispatcher: {:?}", e);
    }
}
```

## Notes

- `start()` blocks until the service stops
- Must be called from the application's `main` function
- The service name must match the registered service name
- Only works when running as a Windows service (not from console)
