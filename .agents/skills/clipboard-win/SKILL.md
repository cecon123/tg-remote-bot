---
name: clipboard-win
description: "Documentation for clipboard-win crate. Keywords: clipboard, copy, paste, windows, text, bitmap, image, html, file, format, unicode, raw"
---

# clipboard-win

> **Version:** 5.4.1 | **Source:** docs.rs

## Overview

Simple clipboard interaction for Windows. Provides getters, setters, and raw bindings for reading/writing clipboard data.

```toml
clipboard-win = "5"

# With clipboard monitoring
clipboard-win = { version = "5", features = ["monitor"] }
```

**Windows-only.** Returns empty results on non-Windows platforms.

## Quick Start

### Simple API (Recommended)

```rust
use clipboard_win::{formats, get_clipboard, set_clipboard};

// Set text
set_clipboard(formats::Unicode, "Hello!").expect("To set clipboard");

// Get text (type annotation required)
let text: String = get_clipboard(formats::Unicode).expect("To get clipboard");
println!("{}", text);
```

### String Shortcuts

```rust
use clipboard_win::{set_clipboard_string, get_clipboard_string};

set_clipboard_string("Hello!").expect("To set clipboard");
let text = get_clipboard_string().expect("To get clipboard");
```

### With Clipboard Lock

```rust
use clipboard_win::{Clipboard, formats, Getter, Setter};

// Open clipboard (keeps lock until dropped)
let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

// Write
formats::Unicode.write_clipboard("Hello!").expect("Write");

// Read
let mut output = String::new();
formats::Unicode.read_clipboard(&mut output).expect("Read");
```

## Key Types

### Clipboard

Opens the clipboard for operations. Only one application can have clipboard open at a time.

```rust
use clipboard_win::Clipboard;

// Open with retries
let clip = Clipboard::new_attempts(10)?;

// Or with custom retry count
let clip = Clipboard::new_attempts(100)?;

// Clipboard closes automatically when dropped
```

**Important:** Close clipboard ASAP. Keep it open only while reading/writing.

### Formats

| Format | Type | Description |
|--------|------|-------------|
| `formats::Unicode` | `String` | UTF-16 text (CF_UNICODETEXT) |
| `formats::Bitmap` | `Vec<u8>` | RGB bitmap image |
| `formats::FileList` | `Vec<PathBuf>` | File list from drag & drop |
| `formats::Html` | `String` | HTML content |
| `formats::RawData(u32)` | `Vec<u8>` | Raw bytes for format ID |

### Getter / Setter Traits

```rust
use clipboard_win::{Getter, Setter};

// Getter - reads from clipboard
let mut buf = String::new();
let bytes_read = formats::Unicode.read_clipboard(&mut buf)?;

// Setter - writes to clipboard
formats::Unicode.write_clipboard("text")?;

// Generic get/set
let text: String = get_clipboard(formats::Unicode)?;
set_clipboard(formats::Unicode, "text")?;
```

## Common Patterns

### Copy/Paste Text

```rust
use clipboard_win::{get_clipboard_string, set_clipboard_string};

// Copy
set_clipboard_string("Hello, clipboard!")?;

// Paste
let text = get_clipboard_string()?;
```

### Copy/Paste HTML

```rust
use clipboard_win::{formats, get_clipboard, set_clipboard};

let html = r#"<html><body><b>Bold text</b></body></html>"#;
set_clipboard(formats::Html, html)?;

let result: String = get_clipboard(formats::Html)?;
```

### Copy/Paste Files

```rust
use clipboard_win::{formats, get_clipboard, set_clipboard};
use std::path::PathBuf;

// Get file list from clipboard (e.g., after Ctrl+C in Explorer)
let files: Vec<PathBuf> = get_clipboard(formats::FileList)?;
for file in &files {
    println!("{}", file.display());
}
```

### Copy Image (Bitmap)

```rust
use clipboard_win::{formats, get_clipboard, set_clipboard};

// Read bitmap as RGB bytes
let rgb_data: Vec<u8> = get_clipboard(formats::Bitmap)?;

// Write bitmap (RGB bytes)
set_clipboard(formats::Bitmap, rgb_data)?;
```

### Copy/Paste Raw Data

```rust
use clipboard_win::{formats, get_clipboard, set_clipboard};

// Custom format (register first or use known ID)
let my_format = formats::RawData(0xC001);  // Custom format ID

// Write raw bytes
set_clipboard(my_format, vec![1, 2, 3, 4])?;

// Read raw bytes
let data: Vec<u8> = get_clipboard(my_format)?;
```

### Using with_clipboard (Scoped)

```rust
use clipboard_win::with_clipboard;

with_clipboard(|| {
    // Clipboard is open during this closure
    let text = clipboard_win::get_clipboard_string()?;
    println!("{}", text);
    Ok(())
})?;
```

## Raw Module

Low-level bindings for advanced use.

```rust
use clipboard_win::raw;

// Check format availability
if raw::is_format_avail(formats::CF_UNICODETEXT) {
    println!("Text available");
}

// Get clipboard owner
let owner = raw::get_owner();

// Get clipboard sequence number (changes on each update)
let seq = raw::seq_num();

// Empty clipboard
raw::empty()?;

// Get data size
let size = raw::size(format_id)?;

// Enumerate available formats
for format_id in raw::EnumFormats::new() {
    println!("Format: {}", format_id);
}

// Register custom format
let custom_id = raw::register_format("MyCustomFormat")?;
```

## Clipboard Monitor

Requires `monitor` feature.

```rust
use clipboard_win::monitor::Monitor;

let monitor = Monitor::new(|seq_num| {
    println!("Clipboard changed! Sequence: {}", seq_num);
})?;

// Monitor runs until dropped
std::thread::sleep(std::time::Duration::from_secs(10));
```

## Error Handling

```rust
use clipboard_win::ErrorCode;

match clipboard_win::get_clipboard_string() {
    Ok(text) => println!("{}", text),
    Err(code) => {
        // ErrorCode wraps Windows error codes
        println!("Clipboard error: {:?}", code);
    }
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` | Enable `std::error::Error` trait (default) |
| `monitor` | Clipboard monitoring support |

## Windows Format Constants

```rust
use clipboard_win::formats;

formats::CF_UNICODETEXT    // UTF-16 text
formats::CF_TEXT           // ANSI text
formats::CF_BITMAP         // Bitmap image
formats::CF_HDROP          // File list (HDROP)
formats::CF_DIB            // Device-independent bitmap
formats::CF_DIBV5          // DIB v5
formats::CF_OEMTEXT        // OEM text
formats::CF_LOCALE         // Locale identifier
formats::CF_RIFF           // RIFF audio
formats::CF_WAVE           // WAV audio
formats::CF_TIFF           // TIFF image
formats::CF_ENHMETAFILE    // Enhanced metafile
formats::CF_METAFILEPICT   // Metafile picture
formats::CF_SYLK           // SYLK format
formats::CF_DIF            // DIF format
formats::CF_PALETTE        // Color palette
formats::CF_PENDATA        // Pen data
```

## Links

- [docs.rs](https://docs.rs/clipboard-win)
- [crates.io](https://crates.io/crates/clipboard-win)
- [GitHub](https://github.com/DoumanAsh/clipboard-win)
- [MSDN: Standard Clipboard Formats](https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats)
