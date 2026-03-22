# sysinfo::System

> Reference documentation for the System struct

## System Creation

```rust
use sysinfo::System;

// Empty (no data loaded)
let sys = System::new();

// Load everything
let sys = System::new_all();

// Load only what you need
let sys = System::new_with_specifics(
    RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
);
```

## Refresh Methods

| Method | What it refreshes |
|--------|-------------------|
| `refresh_all()` | Everything (CPU, memory, processes) |
| `refresh_specifics(kind)` | Selective based on `RefreshKind` |
| `refresh_memory()` | RAM and swap |
| `refresh_memory_specifics(kind)` | Selective memory (RAM/swap) |
| `refresh_cpu_usage()` | CPU usage (needs 2+ calls for accuracy) |
| `refresh_cpu_frequency()` | CPU frequency |
| `refresh_cpu_all()` | All CPU info |
| `refresh_cpu_specifics(kind)` | Selective CPU info |
| `refresh_cpu_list(kind)` | Re-detect CPUs |
| `refresh_processes(to_update, remove_dead)` | Processes |

## System Information (Static)

| Method | Returns |
|--------|---------|
| `System::name()` | `Option<String>` OS name |
| `System::kernel_version()` | `Option<String>` |
| `System::kernel_long_version()` | `String` |
| `System::os_version()` | `Option<String>` |
| `System::long_os_version()` | `Option<String>` |
| `System::host_name()` | `Option<String>` |
| `System::distribution_id()` | `String` |
| `System::distribution_id_like()` | `String` |
| `System::cpu_arch()` | `Option<String>` |
| `System::physical_core_count()` | `Option<usize>` |
| `System::boot_time()` | `u64` seconds since epoch |
| `System::uptime()` | `u64` seconds |
| `System::load_average()` | `LoadAvg` |

## Memory

| Method | Unit |
|--------|------|
| `total_memory()` | bytes |
| `used_memory()` | bytes |
| `available_memory()` | bytes |
| `free_memory()` | bytes |
| `total_swap()` | bytes |
| `used_swap()` | bytes |
| `free_swap()` | bytes |

## Cgroup Limits

```rust
if let Some(limits) = sys.cgroup_limits() {
    println!("memory limit: {} bytes", limits.memory_limit);
    println!("memory usage: {} bytes", limits.mem_usage);
}
```

## CPU Access

| Method | Returns |
|--------|---------|
| `cpus()` | `&[Cpu]` |
| `global_cpu_usage()` | `f32` overall % |

## Process Access

| Method | Returns |
|--------|---------|
| `processes()` | `&HashMap<Pid, Process>` |
| `process(pid)` | `Option<&Process>` |
| `processes_by_name(&OsStr)` | `Iterator<Item = &Process>` |
| `processes_by_exact_name(&OsStr)` | `Iterator<Item = &Process>` |

## RefreshKind Builder

```rust
use sysinfo::{RefreshKind, ProcessRefreshKind, CpuRefreshKind, MemoryRefreshKind};

// Nothing
RefreshKind::nothing()

// Everything
RefreshKind::everything()

// Selective
RefreshKind::nothing()
    .with_processes(ProcessRefreshKind::everything())
    .with_memory(MemoryRefreshKind::everything())
    .with_cpu(CpuRefreshKind::everything())
```

## ProcessRefreshKind

```rust
use sysinfo::{ProcessRefreshKind, UpdateKind};

ProcessRefreshKind::everything()

ProcessRefreshKind::nothing()
    .with_memory()
    .with_cpu()
    .with_disk_usage()
    .with_exe(UpdateKind::OnlyIfNotSet)
    .without_tasks()  // Skip Linux task listing (performance)
```

## CpuRefreshKind

```rust
use sysinfo::CpuRefreshKind;

CpuRefreshKind::nothing().with_cpu_usage()
CpuRefreshKind::nothing().with_frequency()
CpuRefreshKind::everything()
```

## MemoryRefreshKind

```rust
use sysinfo::MemoryRefreshKind;

MemoryRefreshKind::nothing().with_ram()
MemoryRefreshKind::nothing().with_swap()
MemoryRefreshKind::everything()
```

## ProcessesToUpdate

```rust
use sysinfo::ProcessesToUpdate;

ProcessesToUpdate::All               // All processes
ProcessesToUpdate::Some(&[pid1, pid2])  // Specific PIDs
```
