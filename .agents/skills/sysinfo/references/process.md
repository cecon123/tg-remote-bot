# sysinfo::Process

> Reference documentation for Process struct and process management

## Process Fields

| Method | Returns | Description |
|--------|---------|-------------|
| `pid()` | `Pid` | Process ID |
| `name()` | `&OsStr` | Process name (Linux: max 15 chars) |
| `exe()` | `Option<&Path>` | Executable path |
| `cmd()` | `&[OsString]` | Command line |
| `cwd()` | `Option<&Path>` | Working directory |
| `root()` | `Option<&Path>` | Root directory |
| `environ()` | `&[OsString]` | Environment variables |
| `parent()` | `Option<Pid>` | Parent PID |
| `status()` | `ProcessStatus` | Process state |
| `user_id()` | `Option<&Uid>` | User ID |
| `group_id()` | `Option<&Gid>` | Group ID |
| `effective_user_id()` | `Option<&Uid>` | Effective UID |
| `effective_group_id()` | `Option<&Gid>` | Effective GID |
| `session_id()` | `Option<Pid>` | Session ID |

## Memory and CPU

| Method | Returns | Description |
|--------|---------|-------------|
| `memory()` | `u64` | RSS memory (bytes) |
| `virtual_memory()` | `u64` | Virtual memory (bytes) |
| `cpu_usage()` | `f32` | CPU usage (%) |
| `accumulated_cpu_time()` | `u64` | Total CPU time |

## Timing

| Method | Returns | Description |
|--------|---------|-------------|
| `start_time()` | `u64` | Start (seconds since epoch) |
| `run_time()` | `u64` | How long running (seconds) |

## Disk Usage

```rust
let usage = process.disk_usage();
usage.written_bytes;   // u64
usage.read_bytes;      // u64
usage.total_written_bytes;  // u64
usage.total_read_bytes;     // u64
```

## Process Control

| Method | Description |
|--------|-------------|
| `kill()` | Send SIGKILL |
| `kill_with(signal)` | Send specific signal, returns `Option<bool>` |
| `kill_and_wait()` | Kill and wait for exit |
| `kill_with_and_wait(signal)` | Signal and wait |
| `wait()` | Wait for exit, returns `Option<ExitStatus>` |
| `exists()` | Check if process still exists |

## ProcessStatus

```rust
use sysinfo::ProcessStatus;

ProcessStatus::Run
ProcessStatus::Sleep
ProcessStatus::Stop
ProcessStatus::Zombie
ProcessStatus::Idle
ProcessStatus::Tracing
ProcessStatus::Dead
ProcessStatus::Wakekill
ProcessStatus::Waking
ProcessStatus::Parked
ProcessStatus::UninterruptibleDiskSleep
ProcessStatus::Unknown(u32)
```

## ThreadKind

```rust
use sysinfo::ThreadKind;

ThreadKind::Userland   // Regular thread
ThreadKind::Kernel     // Kernel thread
```

## Common Patterns

### Find Process by Name

```rust
let sys = System::new_all();

// Exact name match
for proc in sys.processes_by_exact_name("chrome".as_ref()) {
    println!("[{}] {}", proc.pid(), proc.name().to_string_lossy());
}

// Partial name match
for proc in sys.processes_by_name("chrom".as_ref()) {
    println!("[{}] {}", proc.pid(), proc.name().to_string_lossy());
}
```

### Kill Process

```rust
let sys = System::new_all();
if let Some(proc) = sys.process(Pid::from(1234)) {
    proc.kill();  // Send SIGKILL
}
```

### Check Process Status

```rust
if let Some(proc) = sys.process(pid) {
    match proc.status() {
        ProcessStatus::Run => println!("Running"),
        ProcessStatus::Sleep => println!("Sleeping"),
        ProcessStatus::Zombie => println!("Zombie"),
        _ => println!("Other: {:?}", proc.status()),
    }
}
```

### Monitor Process Resources

```rust
let mut sys = System::new_all();
loop {
    sys.refresh_processes(ProcessesToUpdate::All, true);
    if let Some(proc) = sys.process(pid) {
        println!("CPU: {:.1}%  Memory: {} MB",
            proc.cpu_usage(),
            proc.memory() / 1_000_000,
        );
    }
    std::thread::sleep(Duration::from_secs(1));
}
```

### Kill Process by Name

```rust
let sys = System::new_all();
for proc in sys.processes_by_exact_name("myapp".as_ref()) {
    proc.kill();
}
```
