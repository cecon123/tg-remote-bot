# TG Remote Bot

Windows remote-control agent chạy như Windows Service, điều khiển qua Telegram Bot API.

Kiến trúc: **1 máy = 1 bot token**. Super user gửi lệnh từ bất cứ đâu (DM, group).

## Tính năng

| Lệnh | Mô tả |
|------|--------|
| `/ping` | Pong + IP + uptime + version |
| `/status` | PID + uptime + job status |
| `/screenshot` | Chụp màn hình JPEG |
| `/camera` | Chụp webcam |
| `/sysinfo` | CPU, RAM, disk, network |
| `/clipboard` | Đọc clipboard |
| `/location` | Vị trí IP (đã định dạng) |
| `/netstat` | Danh sách network interfaces |
| `/wifi` | WiFi đã lưu + mật khẩu |
| `/wallpaper` | Lấy hình nền desktop |
| `/shell <cmd>` | Chạy lệnh shell (cmd /C) |
| `/cancel` | Hủy shell job đang chạy |
| `/listfiles <path>` | Liệt kê file trong thư mục |
| `/getfile <path>` | Tải file về (max 50MB) |
| `/procs` | Danh sách process |
| `/kill <pid>` | Kill process |
| `/run <path>` | Chạy chương trình |
| `/lock` | Khóa màn hình |
| `/mute` | Tắt âm hệ thống |
| `/unmute` | Bật âm hệ thống |
| `/volume <0-100>` | Chỉnh âm lượng hệ thống |
| `/msgbox <text>` | Hiện MessageBox (blocking) |
| `/shutdown` | Tắt máy (30s delay) |
| `/restart` | Khởi động lại (30s delay) |
| `/abortshutdown` | Hủy lệnh tắt máy |
| `/update [url]` | Cập nhật agent (không URL = auto-check GitHub) |
| `/uninstall` | Gỡ agent |
| `/help` | Hiển thị trợ giúp |

## Cài đặt

### Yêu cầu

- Windows 10/11
- Rust (edition 2024)
- Telegram Bot Token (tạo qua [@BotFather](https://t.me/BotFather))
- Telegram User ID (lấy qua [@userinfobot](https://t.me/userinfobot))

### Build

```powershell
# Build debug
cargo build

# Build release
cargo build --release
```

### Chạy

#### Mode debug (foreground)

```powershell
cargo run -- --run <BOT_TOKEN> <USER_ID>
```

#### Cài Windows Service (production)

Mở **Command Prompt** với quyền **Administrator**:

```powershell
.\target\release\wininit.exe --install <BOT_TOKEN> <USER_ID>
```

Service tự start (AutoStart). Hoặc khởi động thủ công:

```powershell
sc start TgRemoteAgent
```

### CLI Arguments

| Argument | Mô tả |
|----------|--------|
| `--run TOKEN UID` | Chạy bot foreground (debug/test) |
| `--install TOKEN UID` | Cài Windows Service (cần admin) |
| `--reinstall TOKEN UID` | Cập nhật registry config |
| `--uninstall` | Gỡ Windows Service |
| `--help` | Hiển thị trợ giúp |

## Auto Update

Agent tự động kiểm tra version từ GitHub khi service start:

1. Fetch `tag_name` từ [GitHub Releases API](https://api.github.com/repos/cecon123/tg-remote-bot/releases/latest)
2. So sánh với `env!("CARGO_PKG_VERSION")`
3. Nếu khác → download `wininit.exe` → stop service → swap binary → start service

### Tạo release mới

```bash
# 1. Bump version trong Cargo.toml
version = "1.2.0"

# 2. Commit + push
git add Cargo.toml && git commit -m "bump: 1.2.0" && git push

# 3. Tag + push tag
git tag v1.2.0 && git push origin v1.2.0

# → GitHub Actions tự build + tạo release v1.2.0 với wininit.exe
# → Service đang chạy sẽ tự update khi restart
```

### Manual update từ Telegram

```
/update                    # Auto-check GitHub, download nếu có version mới
/update <url>              # Download từ URL cụ thể
```

## Kiến trúc

```
┌─────────────────────────────────────────┐
│  Telegram User (DM/Group)               │
│  Gửi lệnh: /screenshot, /shell ls, ... │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│  Telegram Bot API (Long Polling)        │
│  getUpdates dispatch reply              │
│  Retry loop khi bị terminated           │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│  wininit.exe                            │
│  ┌───────────────────────────────┐      │
│  │ Auth: check super_user_id    │      │
│  │ Rate Limit: token bucket     │      │
│  │ Router: dispatch to handler  │      │
│  └───────────┬───────────────────┘      │
│              │                           │
│  ┌───────────▼───────────────────┐      │
│  │ Command Handlers              │      │
│  │ screenshot, shell, sysinfo... │      │
│  └───────────┬───────────────────┘      │
│              │                           │
│  ┌───────────▼───────────────────┐      │
│  │ Windows APIs                  │      │
│  │ LockWorkStation, shutdown,    │      │
│  │ Win32, DPAPI, Registry        │      │
│  └───────────────────────────────┘      │
└─────────────────────────────────────────┘
```

## Cấu hình

Config được lưu trong Windows Registry, mã hóa qua DPAPI:

- **Registry path:** `HKCU\SOFTWARE\TgRemoteAgent`
- **Keys:** `Token` (encrypted), `SuperUserId` (encrypted)

Config được set lần đầu qua `--install TOKEN UID` và có thể cập nhật qua `--reinstall TOKEN UID`.

## Service Management

```powershell
# Kiểm tra trạng thái
sc query TgRemoteAgent

# Start
sc start TgRemoteAgent

# Stop
sc stop TgRemoteAgent

# Gỡ bỏ
.\wininit.exe --uninstall
```

Service chạy dưới tài khoản **LocalSystem** (SYSTEM privilege).

## Logging

Log files được ghi vào `{home}/logs/agent_YYYY-MM-DD.log` với daily rotation tự động.

- **Service mode:** Log vào file (không có console)
- **Foreground mode (`--run`):** Log ra file + stderr

Log format: `[2026-03-22 10:15:30] [INFO ] [module_path] message`

## Bảo mật

- **Auth:** Chỉ `super_user_id` mới có thể gửi lệnh
- **Rate limiting:** Token bucket per-command, chống spam
- **DPAPI:** Token lưu trong Registry được mã hóa qua Windows DPAPI
- **Mutex:** Chỉ 1 instance chạy trên mỗi máy
- **Obfuscation:** Service name, registry path được obfuscate bằng `obfstr`
- **Admin check:** `--install` yêu cầu quyền Administrator

## CI/CD

GitHub Actions workflows:

| Workflow | Trigger | Mô tả |
|----------|---------|--------|
| `ci.yml` | Push/PR to main | fmt check, clippy, test |
| `release.yml` | Tag push `v*` | Build release, create GitHub Release with `wininit.exe` |

## Phát triển

```powershell
# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --all

# Format check
cargo fmt --all -- --check

# Check types
cargo check

# Run tests
cargo test

# Run single test
cargo test test_name_here
```

## Cấu trúc dự án

```
src/
├── main.rs           # CLI, mutex check, admin check, SCM dispatch
├── bot/
│   ├── mod.rs        # AgentState, run_until() with retry loop, truncate_str()
│   ├── auth.rs       # is_authorized()
│   ├── md.rs         # MarkdownV2 escape/send helpers
│   ├── router.rs     # Command enum, handler functions, ensure_authorized()
│   └── rate_limit.rs # Token bucket per-command rate limiter
├── commands/         # One file per Telegram command group
├── machine/
│   ├── mod.rs
│   └── session.rs    # User session process spawning (CreateProcessAsUserW)
├── security/         # DPAPI, obfuscation, install_home
├── service/
│   ├── config.rs     # Registry-only config (DPAPI encrypted)
│   ├── install.rs    # SCM install/uninstall + cleanup_old_files()
│   ├── logging.rs    # DailyFile writer + init_logger()
│   └── windows_svc.rs # define_windows_service! + SCM dispatch
└── updater/          # Self-update + auto-update from GitHub
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| teloxide 0.17 | Telegram Bot API framework |
| tokio 1.x | Async runtime |
| sysinfo 0.38 | System info (CPU, RAM, disk) |
| screenshots 0.8 | Screen capture |
| nokhwa 0.10 | Webcam capture |
| image 0.25 | Image processing (JPEG) |
| reqwest 0.13 | HTTP client |
| serde_json 1.0 | JSON parsing (GitHub API) |
| clipboard-win 5.4 | Clipboard access |
| windows-sys 0.61 | Win32 API bindings |
| windows-service 0.8 | Windows Service management |
| winreg 0.56 | Windows Registry |
| obfstr 0.4 | String obfuscation |
| base64 0.22 | Base64 encoding |
| sha2 0.10 | SHA256 hashing |
| fern 0.7 | File logging with daily rotation |
| chrono 0.4 | Timestamp for logs |

## Changelog

### v1.2.0

**Bug fixes:**
- Fix unsigned integer underflow in disk usage calculation (`sysinfo`)
- Fix `wifi` password parsing: `split(':').nth(1)` → `split_once(':')` to handle passwords containing `:`
- Fix `unmute` command: now correctly sends `VOLUME_MUTE` to toggle mute off (was incorrectly sending `VOLUME_UP`)
- Fix `LockWorkStation` API path: moved from `Win32::UI::WindowsAndMessaging` to `Win32::System::Shutdown`
- Fix `DuplicateTokenEx` module path: use `Security` module instead of `Threading`

**Performance & reliability:**
- Shell command: read stdout/stderr concurrently to prevent pipe deadlock
- Location command: use HTTPS endpoint for IP geolocation API
- Location command: parse and format JSON response instead of raw output

**Code quality:**
- Extract auth check duplication into `ensure_authorized()` helper (was duplicated 30+ times)
- Refactor `update` handler: reduce nesting, extract `fetch_github_update_url()`
- Refactor `shutdown`/`restart`/`abort` into shared `run_shutdown_cmd()` helper
- Extract duplicated `cleanup_old_files()` to `service::install` module
- Replace `.unwrap()` on mutex locks with `.unwrap_or_else(|e| e.into_inner())` throughout
- Fix `session.rs` function signatures: `&PathBuf` → `&Path`
- Fix `session.rs` function signatures: `Vec<String>` callers consistency
- Remove dead code: `audio_powershell()` function (had logic error and referenced nonexistent function)
- Clean up `audio.rs`: replace 30 copy-pasted `VOLUME_DOWN` lines with PowerShell loop
- Extract duplicated PowerShell execution into `run_powershell()` helper
- Add missing rate limits for `wifi`, `mute`, `unmute`, `volume`, `msgbox`, `help`, `exit` commands

**API improvements:**
- `/location` now returns formatted data (IP, country, city, ISP, coordinates) instead of raw JSON

### v1.1.1

Previous stable release.

## License

Private project.
