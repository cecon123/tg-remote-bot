# windows_service::service

Core types and enums for Windows service configuration and status.

## Structs

| Struct | Description |
|--------|-------------|
| `Service` | Handle to a system service |
| `ServiceAccess` | Access permission flags for services |
| `ServiceAction` | Action SCM can perform on failure |
| `ServiceConfig` | Service configuration descriptor |
| `ServiceControlAccept` | Accepted control event types |
| `ServiceFailureActions` | Actions on service crash |
| `ServiceInfo` | Service installation parameters |
| `ServiceStatus` | Current service state |
| `ServiceType` | Service type flags |
| `SessionChangeParam` | Session change event data |
| `SessionNotification` | Remote desktop session notification |
| `UserEventCode` | User-defined control code (128-255) |

## Enums

| Enum | Description |
|------|-------------|
| `ServiceState` | Running, Stopped, StartPending, PausePending, etc. |
| `ServiceStartType` | Boot, System, AutoStart, DemandStart, Disabled |
| `ServiceControl` | Stop, Pause, Continue, Interrogate, etc. |
| `ServiceActionType` | None, Restart, Reboot, RunCommand |
| `ServiceDependency` | Service or group dependency |
| `ServiceErrorControl` | Ignore, Normal, Severe, Critical |
| `ServiceExitCode` | Win32 or ServiceSpecific exit code |
| `ServiceFailureResetPeriod` | Failure counter reset period |
| `ServiceSidType` | Service SID configuration |
| `SessionChangeReason` | Session lock/unlock/connect/disconnect |
| `PowerEventParam` | Power event types |
| `HardwareProfileChangeParam` | Hardware profile change events |

## Key Types

### ServiceStatus

```rust
pub struct ServiceStatus {
    pub service_type: ServiceType,
    pub current_state: ServiceState,
    pub controls_accepted: ServiceControlAccept,
    pub exit_code: ServiceExitCode,
    pub checkpoint: u32,
    pub wait_hint: Duration,
    pub process_id: Option<u32>,
}
```

### ServiceInfo

```rust
pub struct ServiceInfo {
    pub name: OsString,
    pub display_name: OsString,
    pub service_type: ServiceType,
    pub start_type: ServiceStartType,
    pub error_control: ServiceErrorControl,
    pub executable_path: PathBuf,
    pub launch_arguments: Vec<OsString>,
    pub dependencies: Vec<ServiceDependency>,
    pub account_name: Option<OsString>,
    pub account_password: Option<OsString>,
}
```

### ServiceState Transitions

```
StartPending --> Running
Running --> StopPending --> Stopped
Running --> PausePending --> Paused
Paused --> ContinuePending --> Running
```
