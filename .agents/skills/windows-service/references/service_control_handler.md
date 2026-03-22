# windows_service::service_control_handler

Handle service control events from the Service Control Manager.

## Types

| Type | Description |
|------|-------------|
| `ServiceStatusHandle` | Token for updating service status (can be cloned) |
| `ServiceControlHandlerResult` | Handler return value |

## Functions

| Function | Description |
|----------|-------------|
| `register()` | Register a closure to receive service events |

## ServiceControlHandlerResult

| Variant | Meaning |
|---------|---------|
| `NoError` | Event handled successfully |
| `NotImplemented` | Event not supported |

## Usage

### Register Handler

```rust
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::service::ServiceControl;

let handler = move |control_event| -> ServiceControlHandlerResult {
    match control_event {
        ServiceControl::Stop => {
            // Signal service to stop
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Pause => {
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Continue => {
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => {
            ServiceControlHandlerResult::NoError
        }
        _ => ServiceControlHandlerResult::NotImplemented,
    }
};

let status_handle = service_control_handler::register("my_service", handler)?;
```

### Update Service Status

```rust
use std::time::Duration;
use windows_service::service::{
    ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};

status_handle.set_service_status(ServiceStatus {
    service_type: ServiceType::OWN_PROCESS,
    current_state: ServiceState::Running,
    controls_accepted: ServiceControlAccept::STOP,
    exit_code: ServiceExitCode::Win32(0),
    checkpoint: 0,
    wait_hint: Duration::default(),
    process_id: None,
})?;
```

### Pending States with Wait Hint

```rust
// For long initialization, report progress
status_handle.set_service_status(ServiceStatus {
    service_type: ServiceType::OWN_PROCESS,
    current_state: ServiceState::StartPending,
    controls_accepted: ServiceControlAccept::empty(),
    exit_code: ServiceExitCode::Win32(0),
    checkpoint: 1,
    wait_hint: Duration::from_secs(30),
    process_id: None,
})?;
```
