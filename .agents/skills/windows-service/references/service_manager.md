# windows_service::service_manager

Service Control Manager (SCM) interface for installing and managing services.

## Structs

| Struct | Description |
|--------|-------------|
| `ServiceManager` | Connection to the Service Control Manager |
| `ServiceManagerAccess` | Access permission flags |

## ServiceManagerAccess Flags

| Flag | Description |
|------|-------------|
| `CONNECT` | Connect to SCM |
| `CREATE_SERVICE` | Create new services |
| `ENUMERATE_SERVICE` | List services |
| `LOCK` | Lock the SCM database |
| `QUERY_LOCK_STATUS` | Query lock status |
| `MODIFY_BOOT_CONFIG` | Modify boot configuration |

## Usage

### Open SCM

```rust
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

// Local computer, no database name
let manager = ServiceManager::local_computer(
    None::<&str>,
    ServiceManagerAccess::CONNECT,
)?;
```

### Create Service

```rust
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::service::{ServiceAccess, ServiceInfo, ServiceStartType, ServiceType};

let manager = ServiceManager::local_computer(
    None::<&str>,
    ServiceManagerAccess::CREATE_SERVICE,
)?;

let service_info = ServiceInfo {
    name: "myservice".into(),
    display_name: "My Service".into(),
    service_type: ServiceType::OWN_PROCESS,
    start_type: ServiceStartType::OnDemand,
    error_control: ServiceErrorControl::Normal,
    executable_path: PathBuf::from(r"C:\path\to\service.exe"),
    launch_arguments: vec![],
    dependencies: vec![],
    account_name: None,
    account_password: None,
};

let service = manager.create_service(&service_info, ServiceAccess::empty())?;
```

### Open Existing Service

```rust
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::service::ServiceAccess;

let manager = ServiceManager::local_computer(
    None::<&str>,
    ServiceManagerAccess::CONNECT,
)?;

let service = manager.open_service("myservice", ServiceAccess::all())?;
```
