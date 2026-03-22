---
name: winreg
description: "Documentation for winreg crate. Keywords: windows, registry, HKEY, RegKey, open_subkey, create_subkey, get_value, set_value, enum_keys, delete_subkey, windows-sys, Win32"
---

# winreg

> **Version:** 0.56.0 | **Source:** docs.rs | **License:** MIT

Rust bindings to the MS Windows Registry API.

## Overview

winreg provides safe, ergonomic access to the Windows Registry. It wraps `windows-sys` handles with an RAII-based `RegKey` type that automatically closes handles on drop. Supports typed value reading/writing, key/subkey iteration, and optional serde serialization.

## Dependencies

- `windows-sys` >=0.59, <=0.61
- Optional: `serde` (feature `serialization-serde`), `chrono` (feature `chrono`)

## Key Types

### `RegKey`
Handle to an opened registry key. Implements `Drop` (closes handle) and `Send` (but not `Sync`).

**Opening keys:**
```rust
use winreg::enums::*;
use winreg::{HKCU, HKLM};

// Open with KEY_READ (default)
let key = HKCU.open_subkey("Software\\MyProduct")?;

// Open with custom permissions
let key = HKLM.open_subkey_with_flags("SOFTWARE\\Microsoft", KEY_READ)?;
```

**Creating keys:**
```rust
let (key, disp) = HKCU.create_subkey("Software\\MyProduct\\Settings")?;
match disp {
    REG_CREATED_NEW_KEY => println!("Created new key"),
    REG_OPENED_EXISTING_KEY => println!("Opened existing key"),
}
```

**Reading values:**
```rust
let server: String = key.get_value("Server")?;
let port: u32 = key.get_value("Port")?;
let raw: RegValue = key.get_raw_value("Data")?;
```

**Writing values:**
```rust
key.set_value("Server", &"example.com")?;
key.set_value("Port", &8080u32)?;
key.set_raw_value("Data", &RegValue { vtype: REG_BINARY, bytes: vec![1,2,3].into() })?;
```

**Iterating:**
```rust
for name in key.enum_keys().map(|x| x.unwrap()) {
    println!("Subkey: {}", name);
}
for (name, value) in key.enum_values().map(|x| x.unwrap()) {
    println!("{} = {:?}", name, value);
}
```

**Deleting:**
```rust
key.delete_value("OldValue")?;
HKCU.delete_subkey("Software\\MyProduct\\OldSubkey")?;
HKCU.delete_subkey_all("Software\\MyProduct")?;  // recursive
```

### `RegValue`
Raw registry value with `vtype: RegType` and `bytes: Cow<[u8]>`.

### `RegKeyMetadata`
Returned by `key.query_info()`. Contains subkey/value counts and last write time.

### Predefined Keys
`HKCR`, `HKCU`, `HKLM`, `HKU`, `HKCC` — shorthand constants for `RegKey::predef(HKEY_*)`.

## Enums (`winreg::enums`)

### `RegDisposition`
- `REG_CREATED_NEW_KEY` — key was created
- `REG_OPENED_EXISTING_KEY` — key already existed

### `RegType`
Registry value types: `REG_SZ`, `REG_DWORD`, `REG_QWORD`, `REG_BINARY`, `REG_MULTI_SZ`, `REG_EXPAND_SZ`, etc.

### Permission Flags
| Constant | Description |
|----------|-------------|
| `KEY_READ` | Standard read access |
| `KEY_WRITE` | Standard write access |
| `KEY_ALL_ACCESS` | Full access |
| `KEY_WOW64_32KEY` | 32-bit registry view |
| `KEY_WOW64_64KEY` | 64-bit registry view |

## Common Patterns

**Read system info:**
```rust
let cur_ver = HKLM.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
let program_files: String = cur_ver.get_value("ProgramFilesDir")?;
```

**Check key disposition:**
```rust
let (_key, disp) = HKCU.create_subkey("Software\\MyApp")?;
if disp == REG_CREATED_NEW_KEY { /* first run */ }
```

**Multi-string values:**
```rust
key.set_value("Paths", &vec!["C:\\a", "C:\\b"])?;
let paths: Vec<String> = key.get_value("Paths")?;
```

**Serde serialization (feature `serialization-serde`):**
```rust
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Settings { width: u32, height: u32 }

let s = Settings { width: 800, height: 600 };
key.encode(&s)?;               // preserve existing subkeys
key.encode_destructive(&s)?;   // wipe before writing
let loaded: Settings = key.decode()?;
```

## Error Handling

All methods return `io::Result<T>`. Common errors:
- `NotFound` — key or value does not exist
- `PermissionDenied` — insufficient registry permissions

## Documentation

- [docs.rs/winreg](https://docs.rs/winreg/latest/winreg/)
- [crates.io/winreg](https://crates.io/crates/winreg)
- [GitHub](https://github.com/gentoo90/winreg-rs)
