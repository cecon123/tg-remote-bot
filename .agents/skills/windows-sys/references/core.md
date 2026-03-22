# windows_sys::core

Core types and macros for Windows FFI.

## Types

| Type | C Equivalent | Description |
|------|--------------|-------------|
| `HRESULT` | `HRESULT` | 32-bit return code (success/error) |
| `BOOL` | `BOOL` | i32, 0 = FALSE, non-zero = TRUE |
| `GUID` | `GUID` | 128-bit globally unique identifier |
| `BSTR` | `BSTR` | COM binary string |
| `HSTRING` | `HSTRING` | Windows Runtime string |
| `PCSTR` | `const u8*` | Const narrow string pointer |
| `PCWSTR` | `const u16*` | Const wide string pointer |
| `PSTR` | `u8*` | Mutable narrow string pointer |
| `PWSTR` | `u16*` | Mutable wide string pointer |
| `IUnknown_Vtbl` | `IUnknownVtbl` | COM IUnknown vtable |
| `IInspectable_Vtbl` | - | WinRT IInspectable vtable |

## Macros

### `s!()` - Narrow string literal

```rust
use windows_sys::s;
let ptr: *const u8 = s!("hello");
```

### `w!()` - Wide string literal

```rust
use windows_sys::w;
let ptr: *const u16 = w!("hello");
```

## Constants

| Constant | Description |
|----------|-------------|
| `IID_IUnknown` | GUID for IUnknown interface |
| `IID_IInspectable` | GUID for IInspectable interface |

## HRESULT Helpers

```rust
// Check success
if result >= 0 { /* succeeded */ }

// Common codes
// S_OK = 0x00000000
// E_FAIL = 0x80004005
// E_INVALIDARG = 0x80070057
```
