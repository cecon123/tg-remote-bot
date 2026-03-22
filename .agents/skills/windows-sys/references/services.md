# windows_sys::Win32::System::Services

Raw Windows Service Control Manager (SCM) API.

## Key Types

| Type | Description |
|------|-------------|
| `SC_HANDLE` | Handle to SCM or service |
| `SERVICE_STATUS` | Service state information |
| `SERVICE_TABLE_ENTRYW` | Service dispatcher table |

## SCM Functions

| Function | Description |
|----------|-------------|
| `OpenSCManagerW` | Connect to SCM |
| `CreateServiceW` | Install a service |
| `OpenServiceW` | Open existing service |
| `DeleteService` | Mark service for deletion |
| `CloseServiceHandle` | Close SCM/service handle |
| `StartServiceW` | Start a service |
| `ControlService` | Send control code |
| `EnumServicesStatusW` | List services |
| `QueryServiceStatus` | Get service status |

## Service Entry Functions

| Function | Description |
|----------|-------------|
| `StartServiceCtrlDispatcherW` | Register service main |
| `RegisterServiceCtrlHandlerExW` | Register control handler |
| `SetServiceStatus` | Update service status |

## Access Constants

| Constant | Description |
|----------|-------------|
| `SC_MANAGER_ALL_ACCESS` | Full SCM access |
| `SC_MANAGER_CONNECT` | Connect to SCM |
| `SC_MANAGER_CREATE_SERVICE` | Create services |
| `SC_MANAGER_ENUMERATE_SERVICE` | List services |
| `SERVICE_ALL_ACCESS` | Full service access |
| `SERVICE_START` | Start service |
| `SERVICE_STOP` | Stop service |

## Service Types

| Constant | Description |
|----------|-------------|
| `SERVICE_WIN32_OWN_PROCESS` | Runs in own process |
| `SERVICE_WIN32_SHARE_PROCESS` | Shares process (svchost) |
| `SERVICE_KERNEL_DRIVER` | Kernel driver |

## Start Types

| Constant | Description |
|----------|-------------|
| `SERVICE_AUTO_START` | Start at boot |
| `SERVICE_DEMAND_START` | Start on demand |
| `SERVICE_DISABLED` | Disabled |

## Service States

| Constant | Description |
|----------|-------------|
| `SERVICE_RUNNING` | Running |
| `SERVICE_STOPPED` | Stopped |
| `SERVICE_START_PENDING` | Starting |
| `SERVICE_STOP_PENDING` | Stopping |
| `SERVICE_PAUSED` | Paused |

## Example: Create and Start Service

```rust
use windows_sys::Win32::System::Services::*;
use windows_sys::w;

unsafe {
    let scm = OpenSCManagerW(
        std::ptr::null(),
        std::ptr::null(),
        SC_MANAGER_CREATE_SERVICE,
    );

    let svc = CreateServiceW(
        scm,
        w!("MyService\0"),
        w!("My Service Display\0"),
        SERVICE_ALL_ACCESS,
        SERVICE_WIN32_OWN_PROCESS,
        SERVICE_DEMAND_START,
        SERVICE_ERROR_NORMAL,
        w!("C:\\path\\to\\service.exe\0"),
        std::ptr::null(),
        std::ptr::null_mut(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    );

    StartServiceW(svc, 0, std::ptr::null());

    CloseServiceHandle(svc);
    CloseServiceHandle(scm);
}
```

> **Note:** For safe wrappers, use the `windows-service` crate.
