use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn lock_screen(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    // LockWorkStation is synchronous and instant — no need for spawn_blocking.
    unsafe {
        windows_sys::Win32::System::Shutdown::LockWorkStation();
    }
    md::send(bot, chat_id, reply_to, "🔒 Màn hình đã khóa".to_string()).await
}

/// Run a shutdown.exe command and reply with success/failure message.
async fn run_shutdown_cmd(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    args: &[&str],
    success_msg: &str,
    fail_msg: &str,
) -> Result<()> {
    let args: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();
    let status = tokio::task::spawn_blocking(move || {
        std::process::Command::new("shutdown").args(&args).status()
    })
    .await??;
    md::send(bot, chat_id, reply_to, if status.success() { success_msg } else { fail_msg }.to_string()).await
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
    let child = std::process::Command::new(path).spawn()?;
    md::send(
        bot,
        chat_id,
        reply_to,
        format!("▶️ Đã chạy: PID `{}`", child.id()),
    )
    .await
}
