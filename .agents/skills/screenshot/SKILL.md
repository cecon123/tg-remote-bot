---
name: screenshot
description: "Windows screen capture via screenshots crate for Telegram bot. Keywords: screenshot, screen capture, screenshots crate, JPEG, display, monitor, capture, bitmap, desktop"
---

# Screenshot Capture

> **Module:** `src/commands/screenshot.rs`
> **Dependencies:** `screenshots`, `image` (re-exported), `teloxide`, `anyhow`

## Overview

Captures the primary screen as JPEG and sends it via Telegram.
Uses the `screenshots` crate which handles platform-specific capture internally.

## Architecture

```
Telegram /screenshot
    │
    ▼
screenshot(bot, chat_id, reply_to)
    │
    ▼
capture_screen()
    │
    ├─ Screen::all() → get all screens
    ├─ screens.first() → primary screen
    ├─ screen.capture() → RgbaImage
    ├─ img.to_rgb8() → convert BGRA to RGB
    ├─ JpegEncoder(quality: 75) → JPEG bytes
    │
    ▼
md::send_photo(bot, chat_id, reply_to, InputFile::memory(jpeg_buf))
```

## Key API

```rust
use screenshots::Screen;

// Get all screens
let screens: Vec<Screen> = Screen::all()?;

// Capture primary screen (returns RgbaImage)
let img = screen.capture()?;

// Capture specific area
let img = screen.capture_area(x, y, width, height)?;

// Get screen info
let info = screen.display_info;  // DisplayInfo struct
```

## Common Issues

### "Không tìm thấy màn hình"
- **Cause:** `Screen::all()` returns empty — no display connected or headless server.
- **Fix:** Ensure a display or virtual framebuffer is available.

### "Capture màn hình thất bại"
- **Cause:** Screen is locked, UAC secure desktop is active, or no user session.
- **Fix:** The `screenshots` crate handles session impersonation internally, but secure desktop (`Winlogon`) blocks capture.

### Black screen returned
- **Cause:** DWM (Desktop Window Manager) compositing — hardware-accelerated windows may appear black.
- **Fix:** The `screenshots` crate uses platform APIs (DXGI on Windows 8+) which handle this better than raw GDI.

## Image Encoding

- Quality: 75 (balance of size vs clarity for Telegram)
- Format: JPEG (Telegram accepts natively, smaller than PNG)
- Uses `screenshots::image::codecs::jpeg::JpegEncoder` (re-exported from `image 0.24`)

## Modifying This Command

| Change | Location |
|--------|----------|
| JPEG quality | `JpegEncoder::new_with_quality(&mut jpeg_buf, 75)` |
| Output format | Replace with PNG encoder |
| Multi-monitor | Use `Screen::all()` and iterate screens |
| Capture area | Use `screen.capture_area(x, y, w, h)` |
| Capture delay | Add `tokio::time::sleep()` before `capture_screen()` |

## Migration from GDI

Previous implementation used raw Win32 GDI calls:
- `WTSQueryUserToken` / `ImpersonateLoggedOnUser` — no longer needed
- `OpenInputDesktop` / `SetThreadDesktop` — no longer needed
- `BitBlt` / `GetDIBits` — replaced by `screen.capture()`
- `windows-sys` GDI features — no longer needed for screenshot

The `screenshots` crate handles all platform-specific logic internally.
