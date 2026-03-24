use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn lock_screen(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--lock".to_string()];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 5000))
            .await??;
    } else {
        lock_workstation();
    }
    md::send(bot, chat_id, reply_to, "🔒 Màn hình đã khóa".to_string()).await
}

pub fn lock_workstation() {
    unsafe {
        windows_sys::Win32::System::Shutdown::LockWorkStation();
    }
}

async fn run_shutdown_cmd(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    args: &[&str],
    success_msg: &str,
    fail_msg: &str,
) -> Result<()> {
    let status = std::process::Command::new("shutdown").args(args).status()?;
    let msg = if status.success() {
        success_msg
    } else {
        fail_msg
    };
    md::send(bot, chat_id, reply_to, msg.to_string()).await
}

pub async fn shutdown(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    run_shutdown_cmd(
        bot,
        chat_id,
        reply_to,
        &["/s", "/t", "30", "/c", "Remote shutdown via Telegram"],
        "⏻ Tắt máy sau 30 giây\\.\\.\\.",
        "❌ Tắt máy thất bại",
    )
    .await
}

pub async fn restart(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    run_shutdown_cmd(
        bot,
        chat_id,
        reply_to,
        &["/r", "/t", "30", "/c", "Remote restart via Telegram"],
        "🔄 Khởi động lại sau 30 giây\\.\\.\\.",
        "❌ Khởi động lại thất bại",
    )
    .await
}

pub async fn abort_shutdown(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    run_shutdown_cmd(
        bot,
        chat_id,
        reply_to,
        &["/a"],
        "✅ Đã hủy tắt máy",
        "ℹ️ Không có lệnh tắt máy nào",
    )
    .await
}

pub async fn run_program(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    path: &str,
) -> Result<()> {
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--run-program".to_string(), path.to_string()];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 0)).await??;
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("▶️ Đã chạy: `{}`", md::escape(path)),
        )
        .await?;
    } else {
        let child = std::process::Command::new(path).spawn()?;
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("▶️ Đã chạy: PID `{}`", child.id()),
        )
        .await?;
    }
    Ok(())
}
