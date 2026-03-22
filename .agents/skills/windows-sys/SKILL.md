---
name: windows-sys
description: "Documentation for windows-sys crate. Keywords: windows, win32, ffi, bindings, api, raw, handle, createfile, registry, process, thread, service, networking, file, io, overlapped"
---

# windows-sys

> **Version:** 0.61.2 | **Source:** docs.rs

## Overview

Raw FFI bindings to the Windows API. Auto-generated from Windows metadata (`.winmd`). For safe, idiomatic wrappers, use the `windows` crate instead.

```toml
[dependencies]
windows-sys = { version = "0.61", features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_IO",
] }
```

## Crate Structure

```
windows_sys
├── core          # Core types: HRESULT, BOOL, GUID, PCSTR, PCWSTR
├── Win32         # Win32 API bindings
│   ├── Foundation    # Handles, RECT, POINT, error codes
│   ├── System        # Services, Threading, IO, Registry, etc.
│   ├── Networking    # Winsock
│   ├── Storage       # File system
│   ├── Security      # ACLs, tokens
│   └── UI            # Windows, messages
└── Wdk           # Windows Driver Kit bindings
```

## Core Types

| Type | Description |
|------|-------------|
| `HRESULT` | 32-bit error/success code |
| `BOOL` | Windows boolean (i32, 0 = false) |
| `GUID` | 128-bit unique identifier |
| `HANDLE` | Generic handle (opaque pointer) |
| `PCSTR` / `PCWSTR` | Const string pointers |
| `PSTR` / `PWSTR` | Mutable string pointers |

## Macros

| Macro | Description |
|-------|-------------|
| `s!("text")` | UTF-8 string with null terminator |
| `w!("text")` | UTF-16 wide string with null terminator |

```rust
use windows_sys::core::{PCSTR, PCWSTR};
use windows_sys::{s, w};

let narrow: PCSTR = s!("hello");
let wide: PCWSTR = w!("hello");
```

## Feature Flags Pattern

Feature flags mirror module paths with `::` replaced by `_`:

| Module Path | Feature Flag |
|-------------|--------------|
| `Win32::Foundation` | `Win32_Foundation` |
| `Win32::System::Threading` | `Win32_System_Threading` |
| `Win32::System::IO` | `Win32_System_IO` |
| `Win32::Networking::WinSock` | `Win32_Networking_WinSock` |

Use the [feature search tool](https://microsoft.github.io/windows-rs/features/) to find required features.

## Common Patterns

### Error Handling

```rust
use windows_sys::Win32::Foundation::GetLastError;

unsafe {
    let result = SomeWindowsApi(...);
    if result == 0 {
        let err = GetLastError();
        eprintln!("Error: {err:#x}");
    }
}
```

### String Conversion

```rust
use std::ffi::CString;
use windows_sys::core::PCSTR;

let msg = CString::new("Hello").unwrap();
let pcstr: PCSTR = msg.as_ptr() as _;
```

### Handle Lifecycle

```rust
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};

unsafe {
    let h: HANDLE = CreateFileW(...);
    if h == INVALID_HANDLE_VALUE { return Err(...); }

    // Use handle...

    CloseHandle(h);
}
```

## Quick Reference

| Task | Module |
|------|--------|
| Create/open files | `Win32::Storage::FileSystem` |
| Processes & threads | `Win32::System::Threading` |
| Async IO (overlapped) | `Win32::System::IO` |
| Windows services | `Win32::System::Services` |
| Registry | `Win32::System::Registry` |
| TCP/UDP sockets | `Win32::Networking::WinSock` |
| Mutex/Event/Semaphore | `Win32::System::Threading` |
| DLL loading | `Win32::System::LibraryLoader` |

## Documentation

- `./references/core.md` - Core types and macros
- `./references/foundation.md` - Foundation handles and types
- `./references/threading.md` - Process/thread/sync primitives
- `./references/io.md` - Overlapped IO
- `./references/services.md` - Windows services (raw)
- `./references/networking.md` - WinSock basics

## Links

- [docs.rs](https://docs.rs/windows-sys)
- [crates.io](https://crates.io/crates/windows-sys)
- [GitHub](https://github.com/microsoft/windows-rs)
- [Feature Search](https://microsoft.github.io/windows-rs/features/)
