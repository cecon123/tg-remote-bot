use std::path::Path;

use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;

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
    for entry in dir.read_dir().context("cannot read dir")? {
        if let Ok(e) = entry {
            let name = e.file_name().to_string_lossy().to_string();
            let meta = e.metadata().ok();
            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let kind = if meta.as_ref().map(|m| m.is_dir()).unwrap_or(false) {
                "📁"
            } else {
                "📄"
            };
            entries.push(format!("{} {} \\({} bytes\\)", kind, md::escape(&name), size));
        }
    }

    if entries.is_empty() {
        md::send(bot, chat_id, reply_to, "📭 Thư mục trống".to_string()).await?;
    } else {
        let text = entries.join("\n");
        let text = crate::bot::truncate_str(&text, 3800);
        md::send(bot, chat_id, reply_to, format!("*📂 {}*\n\n{}", md::escape(path), text)).await?;
    }

    Ok(())
}

pub async fn getfile(bot: &Bot, chat_id: ChatId, reply_to: MessageId, path: &str) -> Result<()> {
    let file = Path::new(path);
    if !file.exists() || !file.is_file() {
        md::send(bot, chat_id, reply_to, "❌ File không tồn tại".to_string()).await?;
        return Ok(());
    }

    if file.metadata()?.len() > 50 * 1024 * 1024 {
        md::send(bot, chat_id, reply_to, "❌ File quá lớn \\(\\>50MB\\)".to_string()).await?;
        return Ok(());
    }

    let filename = file
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    bot.send_document(chat_id, InputFile::file(file))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_parameters(teloxide::types::ReplyParameters::new(reply_to))
        .caption(format!("📄 {}", md::escape(&filename)))
        .await?;

    Ok(())
}
