---
name: sysinfo
description: "Documentation for sysinfo crate. Keywords: system, process, cpu, memory, disk, network, component, temperature, load, uptime, refresh, pid, kill, signal, users"
---

# sysinfo

> **Version:** 0.38.4 | **Source:** docs.rs

## Overview

Cross-platform library for getting system information: processes, CPUs, disks, networks, components (temperature), and users. Supports Linux, macOS, Windows, FreeBSD, Android, iOS.

```toml
sysinfo = "0.38"
```

## Quick Start

```rust
use sysinfo::{System, Disks, Networks, Components};

let mut sys = System::new_all();

println!("total memory: {} bytes", sys.total_memory());
println!("used memory: {} bytes", sys.used_memory());
println!("NB CPUs: {}", sys.cpus().len());
println!("System name: {:?}", System::name());
println!("kernel version: {:?}", System::kernel_version());

for (pid, process) in sys.processes() {
    println!("[{}] {:?}", pid, process.name());
}
```

## Key Types

### System

Main struct for system info (memory, CPU, processes).

```rust
use sysinfo::System;

let mut sys = System::new_all();

// Refresh all
sys.refresh_all();

// Or refresh selectively
sys.refresh_memory();
sys.refresh_cpu_usage();
sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
```

| Method | Description |
|--------|-------------|
| `new()` | Empty system |
| `new_all()` | Fully loaded system |
| `refresh_all()` | Refresh everything |
| `refresh_memory()` | Refresh RAM/SWAP |
| `refresh_cpu_usage()` | Refresh CPU usage |
| `refresh_processes(to_update, remove_dead)` | Refresh processes |

#### Memory

| Method | Returns |
|--------|---------|
| `total_memory()` | Total RAM (bytes) |
| `used_memory()` | Used RAM (bytes) |
| `available_memory()` | Available RAM (bytes) |
| `free_memory()` | Free RAM (bytes) |
| `total_swap()` | Total swap (bytes) |
| `used_swap()` | Used swap (bytes) |
| `free_swap()` | Free swap (bytes) |

#### CPU

| Method | Returns |
|--------|---------|
| `cpus()` | Slice of `Cpu` |
| `global_cpu_usage()` | Overall CPU usage (%) |
| `physical_core_count()` | Physical cores |

#### Processes

| Method | Returns |
|--------|---------|
| `processes()` | `&HashMap<Pid, Process>` |
| `process(pid)` | `Option<&Process>` |
| `processes_by_name(name)` | Iterator over matching processes |
| `processes_by_exact_name(name)` | Iterator over exact name matches |

#### System Info (static methods)

| Method | Returns |
|--------|---------|
| `System::name()` | `Option<String>` (e.g., "Linux") |
| `System::kernel_version()` | `Option<String>` |
| `System::os_version()` | `Option<String>` |
| `System::long_os_version()` | `Option<String>` |
| `System::host_name()` | `Option<String>` |
| `System::cpu_arch()` | `Option<String>` |
| `System::boot_time()` | Boot time (seconds since epoch) |
| `System::uptime()` | Uptime (seconds) |
| `System::load_average()` | `LoadAvg` struct |

### Process

Represents a running process.

| Method | Description |
|--------|-------------|
| `pid()` | Process ID |
| `name()` | Process name (`OsStr`) |
| `exe()` | Executable path (`Option<&Path>`) |
| `cmd()` | Command line args (`&[OsString]`) |
| `cwd()` | Current working directory |
| `root()` | Root directory |
| `environ()` | Environment variables |
| `memory()` | RSS memory (bytes) |
| `virtual_memory()` | Virtual memory (bytes) |
| `cpu_usage()` | CPU usage (%) |
| `parent()` | Parent PID (`Option<Pid>`) |
| `status()` | `ProcessStatus` enum |
| `start_time()` | Start time (seconds since epoch) |
| `run_time()` | Running time (seconds) |
| `disk_usage()` | `DiskUsage` struct |
| `user_id()` | `Option<&Uid>` |
| `group_id()` | `Option<&Gid>` |
| `session_id()` | `Option<Pid>` |
| `kill()` | Send SIGKILL |
| `kill_with(signal)` | Send specific signal |
| `kill_and_wait()` | Kill and wait for exit |
| `wait()` | Wait for process to exit |
| `exists()` | Check if process exists |

### Cpu

```rust
use sysinfo::Cpu;

let cpu = &sys.cpus()[0];
cpu.name();       // &str
cpu.cpu_usage();  // f32 (%)
cpu.frequency();  // u64 (MHz)
cpu.vendor_id();  // &str
cpu.brand();      // &str
```

### Disk / Disks

```rust
use sysinfo::Disks;

let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{}: {} GB / {} GB",
        disk.name().to_string_lossy(),
        disk.available_space() / 1_000_000_000,
        disk.total_space() / 1_000_000_000,
    );
}
```

| Method | Description |
|--------|-------------|
| `name()` | Disk name |
| `file_system()` | File system type |
| `mount_point()` | Mount path |
| `total_space()` | Total size (bytes) |
| `available_space()` | Available space (bytes) |
| `kind()` | `DiskKind` enum |

### Network / Networks

```rust
use sysinfo::Networks;

let networks = Networks::new_with_refreshed_list();
for (name, data) in &networks {
    println!("{}: {} down / {} up",
        name,
        data.total_received(),
        data.total_transmitted(),
    );
}
```

| Method | Description |
|--------|-------------|
| `received()` | Bytes received since last refresh |
| `total_received()` | Total bytes received |
| `transmitted()` | Bytes transmitted since last refresh |
| `total_transmitted()` | Total bytes transmitted |
| `packets_received()` | Packets received |
| `packets_transmitted()` | Packets transmitted |
| `errors_on_received()` | Receive errors |
| `errors_on_transmitted()` | Transmit errors |
| `mac_address()` | `MacAddr` |

### Component

Temperature sensors and fans.

```rust
use sysinfo::Components;

let components = Components::new_with_refreshed_list();
for component in &components {
    println!("{}: {}°C", component.label(), component.temperature());
}
```

### Users / Groups

```rust
use sysinfo::Users;

let users = Users::new_with_refreshed_list();
for user in users.iter() {
    println!("{}: groups={:?}", user.name(), user.groups());
}
```

## Constants

| Constant | Description |
|----------|-------------|
| `IS_SUPPORTED_SYSTEM` | `bool` - Is OS supported |
| `MINIMUM_CPU_UPDATE_INTERVAL` | Min interval for CPU updates |
| `SUPPORTED_SIGNALS` | `&[Signal]` - Available signals |

## Functions

| Function | Description |
|----------|-------------|
| `get_current_pid()` | Get current process PID |
| `set_open_files_limit(n)` | Set file descriptor limit (Linux) |

## Refresh Kinds (Selective Refresh)

```rust
use sysinfo::{RefreshKind, ProcessRefreshKind, CpuRefreshKind, MemoryRefreshKind};

// Only refresh processes
sys.refresh_specifics(
    RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
);

// Only refresh CPU usage (not frequency)
sys.refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());

// Only refresh RAM (not swap)
sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
```

## Process Status

```rust
use sysinfo::ProcessStatus;

match process.status() {
    ProcessStatus::Run => { /* running */ }
    ProcessStatus::Sleep => { /* sleeping */ }
    ProcessStatus::Stop => { /* stopped */ }
    ProcessStatus::Zombie => { /* zombie */ }
    ProcessStatus::Idle => { /* idle (macOS) */ }
    ProcessStatus::Tracing => { /* being traced */ }
    ProcessStatus::Dead => { /* dead */ }
    ProcessStatus::Wakekill => { /* wakekill (Linux) */ }
    ProcessStatus::Waking => { /* waking */ }
    ProcessStatus::Parked => { /* parked */ }
    ProcessStatus::UninterruptibleDiskSleep => { /* D state */ }
    ProcessStatus::Unknown(n) => { /* unknown */ }
}
```

## Signals

```rust
use sysinfo::{Signal, SUPPORTED_SIGNALS};

// Kill a process with a specific signal
if let Some(result) = process.kill_with(Signal::Interrupt) {
    println!("Signal sent: {}", result);
}

// List supported signals
for signal in SUPPORTED_SIGNALS {
    println!("{:?}", signal);
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | Includes `multithread` |
| `multithread` | Use multiple threads for data collection |
| `serde` | Serde serialization support |
| `apple-app-store` | Disable prohibited Apple APIs |
| `apple-sandbox` | Avoid sandbox policy violations |
| `linux-netdev` | Use netdev for network info |
| `linux-pid` | Use /proc/<pid> for process info |

## Serde Support

```rust
use sysinfo::System;

let mut sys = System::new_all();
sys.refresh_all();

println!("{}", serde_json::to_string(&sys).unwrap());
```

## Best Practices

1. **Reuse System instance** - Creating a new `System` is expensive; reuse it
2. **Use selective refresh** - `refresh_specifics()` is faster than `refresh_all()`
3. **Wait before first CPU read** - CPU usage needs two measurements to be accurate
4. **Remove dead processes** - Use `remove_dead_processes: true` in `refresh_processes`

## Documentation

- `./references/system.md` - System struct details
- `./references/process.md` - Process struct and management

## Links

- [docs.rs](https://docs.rs/sysinfo)
- [crates.io](https://crates.io/crates/sysinfo)
- [GitHub](https://github.com/GuillaumeGomez/sysinfo)
- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
