use std::path::Path;

use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::{md, truncate_and_escape};

/// Max file size for /getfile (50 MB).
const MAX_GETFILE_BYTES: u64 = 50 * 1024 * 1024;

pub async fn listfiles(bot: &Bot, chat_id: ChatId, reply_to: MessageId, path: &str) -> Result<()> {
    let dir = Path::new(path);
    if !dir.exists() {
        md::send(bot, chat_id, reply_to, "❌ Đường dẫn không tồn tại".to_string()).await?;
        return Ok(());
    }
    if !dir.is_dir() {
        md::send(bot, chat_id, reply_to, "❌ Không phải thư mục".to_string()).await?;
        return Ok(());
    }

    let mut entries = Vec::new();
    for e in dir.read_dir().context("cannot read dir")?.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let (kind, size) = match e.metadata() {
            Ok(m) => {
                let kind = if m.is_dir() { "📁" } else { "📄" };
                (kind, m.len())
            }
            Err(_) => ("❓", 0),
        };
        entries.push(format!("{kind} {name} ({size} bytes)"));
    }

    if entries.is_empty() {
        md::send(bot, chat_id, reply_to, "📭 Thư mục trống".to_string()).await?;
    } else {
        let escaped = truncate_and_escape(&entries.join("\n"), md::MAX_MSG_BYTES);
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("*📂 {}*\n\n{escaped}", md::escape(path)),
        )
        .await?;
    }

    Ok(())
}

pub async fn getfile(bot: &Bot, chat_id: ChatId, reply_to: MessageId, path: &str) -> Result<()> {
    let file = Path::new(path);
    if !file.exists() || !file.is_file() {
        md::send(bot, chat_id, reply_to, "❌ File không tồn tại".to_string()).await?;
        return Ok(());
    }

    if file.metadata()?.len() > MAX_GETFILE_BYTES {
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("❌ File quá lớn \\(>{}MB\\)", MAX_GETFILE_BYTES / 1024 / 1024),
        )
        .await?;
        return Ok(());
    }

    let filename = file
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    md::send_document(bot, chat_id, reply_to, InputFile::file(file), format!("📄 {}", md::escape(&filename))).await
}
