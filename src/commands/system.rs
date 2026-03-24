use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn lock_screen(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    std::process::Command::new("rundll32.exe")
        .args(["user32.dll,LockWorkStation"])
        .spawn()?;
    md::send(bot, chat_id, reply_to, "🔒 Màn hình đã khóa".to_string()).await
}

pub async fn shutdown(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let status = std::process::Command::new("shutdown")
        .args(["/s", "/t", "30", "/c", "Remote shutdown via Telegram"])
        .status()?;
    if status.success() {
        md::send(
            bot,
            chat_id,
            reply_to,
            "⏻ Tắt máy sau 30 giây\\.\\.\\.".to_string(),
        )
        .await?;
    } else {
        md::send(bot, chat_id, reply_to, "❌ Tắt máy thất bại".to_string()).await?;
    }
    Ok(())
}

pub async fn restart(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let status = std::process::Command::new("shutdown")
        .args(["/r", "/t", "30", "/c", "Remote restart via Telegram"])
        .status()?;
    if status.success() {
        md::send(
            bot,
            chat_id,
            reply_to,
            "🔄 Khởi động lại sau 30 giây\\.\\.\\.".to_string(),
        )
        .await?;
    } else {
        md::send(
            bot,
            chat_id,
            reply_to,
            "❌ Khởi động lại thất bại".to_string(),
        )
        .await?;
    }
    Ok(())
}

pub async fn abort_shutdown(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let status = std::process::Command::new("shutdown")
        .args(["/a"])
        .status()?;
    if status.success() {
        md::send(bot, chat_id, reply_to, "✅ Đã hủy tắt máy".to_string()).await?;
    } else {
        md::send(
            bot,
            chat_id,
            reply_to,
            "ℹ️ Không có lệnh tắt máy nào".to_string(),
        )
        .await?;
    }
    Ok(())
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
