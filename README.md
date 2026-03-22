# TG Remote Bot

Windows remote-control agent chạy như Windows Service, điều khiển qua Telegram Bot API.

Kiến trúc: **1 máy = 1 bot token**. Super user gửi lệnh từ bất cứ đâu (DM, group).

## Tính năng

| Lệnh | Mô tả |
|------|--------|
| `/ping` | Pong + IP + uptime + version |
| `/status` | PID + uptime + job status |
| `/screenshot` | Chụp màn hình → JPEG |
| `/camera` | Chụp webcam |
| `/sysinfo` | CPU, RAM, disk, network |
| `/clipboard` | Đọc clipboard |
| `/location` | Vị trí IP |
| `/netstat` | Danh sách network interfaces |
| `/wallpaper` | Lấy hình nền desktop |
| `/shell <cmd>` | Chạy lệnh shell (cmd /C) |
| `/cancel` | Hủy shell job đang chạy |
| `/listfiles <path>` | Liệt kê file trong thư mục |
| `/getfile <path>` | Tải file về (max 50MB) |
| `/procs` | Danh sách process |
| `/kill <pid>` | Kill process |
| `/run <path>` | Chạy chương trình |
| `/lock` | Khóa màn hình |
| `/shutdown` | Tắt máy (30s delay) |
| `/restart` | Khởi động lại (30s delay) |
| `/abortshutdown` | Hủy lệnh tắt máy |
| `/update <url>` | Cập nhật agent |
| `/uninstall` | Gỡ agent |
| `/help` | Hiển thị trợ giúp |

## Cài đặt

### Yêu cầu

- Windows 10/11
- Rust (edition 2024)
- Telegram Bot Token (tạo qua [@BotFather](https://t.me/BotFather))
- Telegram User ID (lấy qua [@userinfobot](https://t.me/userinfobot))

### Bước 1: Tạo file .env

```env
TG_BOT_TOKEN=your_bot_token_here
TG_SUPER_USER_ID=your_telegram_user_id
```

Hoặc file `.env.example` đã có sẵn, copy và sửa:

```powershell
copy .env.example .env
notepad .env
```

### Bước 2: Build

```powershell
# Build debug (test nhanh)
cargo build

# Build release (deploy)
cargo build --release
```

Config được bake vào binary qua `build.rs` + `dotenvy`. Không cần file .env khi chạy binary đã build.

### Bước 3: Chạy

#### Mode debug (foreground)

```powershell
.\target\debug\tg-remote-bot.exe --run
```

#### Cài Windows Service (production)

Mở **Command Prompt** với quyền **Administrator**:

```powershell
.\target\release\tg-remote-bot.exe --install
```

Sau đó start service:

```powershell
sc start TgRemoteAgent
```

Hoặc reboot → service tự start (AutoStart).

### CLI Arguments

| Argument | Mô tả |
|----------|--------|
| `--run` | Chạy bot foreground (debug/test) |
| `--install [TOKEN UID]` | Cài Windows Service (cần admin) |
| `--reinstall TOKEN UID` | Cập nhật registry config |
| `--uninstall` | Gỡ Windows Service |
| `--help` | Hiển thị trợ giúp |

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
│  Retry loop khi bị terminated           │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│  tg-remote-bot.exe                      │
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

Config được load theo thứ tự ưu tiên:

1. **Env vars** (`TG_BOT_TOKEN`, `TG_SUPER_USER_ID`) — build time
2. **`.env` file** — build time, qua `dotenvy`
3. **Hardcoded fallback** trong `build.rs`

Sau khi build, config được bake vào binary qua `env!("BAKED_TOKEN")`.

## Service Management

```powershell
# Kiểm tra trạng thái
sc query TgRemoteAgent

# Start
sc start TgRemoteAgent

# Stop
sc stop TgRemoteAgent

# Gỡ bỏ
.\tg-remote-bot.exe --uninstall
```

Service chạy dưới tài khoản **LocalSystem** (SYSTEM privilege).

## Bảo mật

- **Auth:** Chỉ `super_user_id` mới có thể gửi lệnh
- **Rate limiting:** Token bucket per-command, chống spam
- **DPAPI:** Token lưu trong Registry được mã hóa qua Windows DPAPI
- **Mutex:** Chỉ 1 instance chạy trên mỗi máy
- **Obfuscation:** Service name, registry path được obfuscate bằng `obfstr`
- **Admin check:** `--install` yêu cầu quyền Administrator

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
│   ├── mod.rs        # AgentState, run_until() with retry loop
│   ├── auth.rs       # is_authorized()
│   ├── md.rs         # MarkdownV2 escape/send helpers
│   ├── router.rs     # Command enum, all handler functions
│   └── rate_limit.rs # Token bucket per-command rate limiter
├── commands/         # One file per Telegram command group
├── machine/          # Machine identity (SHA256 hostname+MAC)
├── security/         # DPAPI, obfuscation
├── service/          # Windows Service config, install, SCM
└── updater/          # Self-update (stub)
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
| clipboard-win 5.4 | Clipboard access |
| windows-sys 0.61 | Win32 API bindings |
| windows-service 0.8 | Windows Service management |
| winreg 0.56 | Windows Registry |
| obfstr 0.4 | String obfuscation |
| base64 0.22 | Base64 encoding |
| sha2 0.10 | SHA256 hashing |
| dotenvy 0.15 | .env file loading (build-time) |

## License

Private project.
