# windows_sys::Win32::Foundation

Foundation types, handles, and error codes.

## Key Structs

| Struct | Description |
|--------|-------------|
| `HANDLE` | Generic handle type |
| `HWND` | Window handle |
| `HINSTANCE` | Module instance handle |
| `HMODULE` | Module handle |
| `HDC` | Device context handle |
| `HICON`, `HCURSOR`, `HBRUSH`, `HPEN` | GDI object handles |
| `RECT` | Rectangle (left, top, right, bottom) |
| `POINT` | Point (x, y) |
| `SIZE` | Size (cx, cy) |
| `FILETIME` | 64-bit file time |
| `SYSTEMTIME` | Date/time structure |
| `LUID` | Locally unique identifier |

## Common Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `TRUE` | 1 | Boolean true |
| `FALSE` | 0 | Boolean false |
| `INVALID_HANDLE_VALUE` | -1 (as ptr) | Invalid handle sentinel |
| `S_OK` | 0 | Success |
| `E_FAIL` | 0x80004005 | Generic failure |
| `E_INVALIDARG` | 0x80070057 | Invalid argument |
| `E_OUTOFMEMORY` | 0x8007000E | Out of memory |

## Common Functions

| Function | Description |
|----------|-------------|
| `CloseHandle` | Close a handle |
| `GetLastError` | Get last error code |
| `SetLastError` | Set error code |
| `FormatMessageW` | Format error message string |

## Error Handling Pattern

```rust
use windows_sys::Win32::Foundation::{GetLastError, ERROR_SUCCESS};

unsafe {
    let result = SomeApiCall();
    if result == 0 {
        let err = GetLastError();
        if err != ERROR_SUCCESS {
            // Handle error
        }
    }
}
```

## Handle Cleanup

```rust
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};

struct SafeHandle(HANDLE);
impl Drop for SafeHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0); }
        }
    }
}
```
