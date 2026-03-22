---
name: windows-service
description: "Documentation for windows-service crate. Keywords: windows, service, service-control, scm, windows-service, daemon, background-service, service-manager, service-dispatcher"
---

# Windows Service

> **Version:** 0.8.0 | **Source:** docs.rs

## Overview

A crate that provides facilities for management and implementation of Windows services. Built on top of `windows-sys`, it offers safe Rust abstractions for the Windows Service Control Manager (SCM) API.

```toml
[dependencies]
windows-service = "0.8.0"
```

## Key Modules

### service_manager - Service installation and management

Manage Windows services through the Service Control Manager.

| Struct | Description |
|--------|-------------|
| `ServiceManager` | Connection to the Service Control Manager |
| `ServiceManagerAccess` | Access permission flags |

```rust
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

let manager = ServiceManager::local_computer(
    None::<&str>,
    ServiceManagerAccess::CREATE_SERVICE,
)?;
```

### service - Service types and configuration

Core types for service configuration and status.

| Struct | Description |
|--------|-------------|
| `Service` | Handle to a system service |
| `ServiceInfo` | Service configuration |
| `ServiceStatus` | Current service state |
| `ServiceAccess` | Access permission flags |
| `ServiceType` | Service type (OWN_PROCESS, etc.) |

| Enum | Description |
|------|-------------|
| `ServiceState` | Running, Stopped, StartPending, etc. |
| `ServiceStartType` | Auto, Manual, Disabled |
| `ServiceControl` | Stop, Pause, Interrogate, etc. |
| `ServiceErrorControl` | Error handling strategy |

### service_control_handler - Event handling

Handle service control events from the SCM.

| Type | Description |
|------|-------------|
| `ServiceStatusHandle` | Token for updating service status |
| `ServiceControlHandlerResult` | Handler return value |
| `register()` | Register event handler |

```rust
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::service::ServiceControl;

let handler = move |event| match event {
    ServiceControl::Stop => ServiceControlHandlerResult::NoError,
    ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
    _ => ServiceControlHandlerResult::NotImplemented,
};

let status_handle = service_control_handler::register("my_service", handler)?;
```

### service_dispatcher - Entry point

Start the service control dispatcher.

| Function | Description |
|----------|-------------|
| `start()` | Register service entry and block until stopped |

```rust
use windows_service::service_dispatcher;

define_windows_service!(ffi_service_main, my_service_main);

fn main() -> windows_service::Result<()> {
    service_dispatcher::start("myservice", ffi_service_main)?;
    Ok(())
}
```

## Macros

| Macro | Description |
|-------|-------------|
| `define_windows_service!` | Generate service entry function boilerplate |

## Complete Example

```rust
#[macro_use]
extern crate windows_service;

use std::ffi::OsString;
use std::time::Duration;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState,
    ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(_arguments: Vec<OsString>) {
    if let Err(_e) = run_service() {
        // Handle error
    }
}

fn run_service() -> windows_service::Result<()> {
    let handler = move |event| -> ServiceControlHandlerResult {
        match event {
            ServiceControl::Stop => ServiceControlHandlerResult::NoError,
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register("my_service", handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Service work loop here

    Ok(())
}

fn main() -> windows_service::Result<()> {
    service_dispatcher::start("my_service", ffi_service_main)?;
    Ok(())
}
```

## Documentation

- `./references/service.md` - Service module types and enums
- `./references/service_manager.md` - Service manager for installation
- `./references/service_control_handler.md` - Event handler types
- `./references/service_dispatcher.md` - Dispatcher entry point

## Links

- [docs.rs](https://docs.rs/windows-service)
- [crates.io](https://crates.io/crates/windows-service)
- [GitHub](https://github.com/mullvad/windows-service-rs)
