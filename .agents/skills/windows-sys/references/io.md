# windows_sys::Win32::System::IO

Asynchronous I/O with overlapped structures and I/O completion ports.

## Key Structs

| Struct | Description |
|--------|-------------|
| `OVERLAPPED` | Async I/O context |
| `OVERLAPPED_ENTRY` | Completed I/O entry |
| `IO_STATUS_BLOCK` | NT-style I/O status |

## I/O Completion Port

```rust
use windows_sys::Win32::System::IO::{
    CreateIoCompletionPort, GetQueuedCompletionStatus,
};
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};

unsafe {
    // Create completion port
    let iocp = CreateIoCompletionPort(
        INVALID_HANDLE_VALUE,
        0,
        0,
        0, // 0 = use default (number of CPUs)
    );

    // Associate file handle with port
    CreateIoCompletionPort(file_handle, iocp, completion_key, 0);

    // Get completed I/O
    let mut bytes: u32 = 0;
    let mut key: usize = 0;
    let mut overlapped: *mut OVERLAPPED = std::ptr::null_mut();

    GetQueuedCompletionStatus(
        iocp,
        &mut bytes,
        &mut key,
        &mut overlapped,
        0xFFFFFFFF, // INFINITE
    );
}
```

## Key Functions

| Function | Description |
|----------|-------------|
| `CreateIoCompletionPort` | Create or associate IOCP |
| `GetQueuedCompletionStatus` | Dequeue completed I/O |
| `GetQueuedCompletionStatusEx` | Batch dequeue |
| `PostQueuedCompletionStatus` | Post custom completion |
| `CancelIo` | Cancel pending I/O |
| `CancelIoEx` | Cancel specific I/O |
| `GetOverlappedResult` | Wait for overlapped completion |
| `DeviceIoControl` | Send IOCTL to device driver |

## OVERLAPPED Usage

```rust
use windows_sys::Win32::System::IO::OVERLAPPED;
use std::mem::zeroed;

unsafe {
    let mut ov: OVERLAPPED = zeroed();
    ov.Anonymous.Anonymous.Offset = 0;
    ov.Anonymous.Anonymous.OffsetHigh = 0;

    // Pass to ReadFile/WriteFile/etc
    ReadFile(handle, buf, len, std::ptr::null_mut(), &mut ov);
}
```
