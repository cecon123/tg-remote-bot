use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;

pub async fn wallpaper(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let key = match hkcu.open_subkey(r"Control Panel\Desktop") {
        Ok(k) => k,
        Err(e) => {
            md::send(
                bot,
                chat_id,
                reply_to,
                format!(
                    "❌ {}",
                    md::escape(&format!("Không đọc được registry: {e}"))
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let path: String = match key.get_value("WallPaper") {
        Ok(p) => p,
        Err(e) => {
            md::send(
                bot,
                chat_id,
                reply_to,
                format!(
                    "❌ {}",
                    md::escape(&format!("Không tìm thấy wallpaper: {e}"))
                ),
            )
            .await?;
            return Ok(());
        }
    };

    if path.is_empty() {
        md::send(bot, chat_id, reply_to, "❌ Không có wallpaper".to_string()).await?;
        return Ok(());
    }

    let file = std::path::Path::new(&path);
    if !file.exists() {
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("❌ File không tồn tại: {}", md::escape(&path)),
        )
        .await?;
        return Ok(());
    }

    bot.send_document(chat_id, InputFile::file(file))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_parameters(teloxide::types::ReplyParameters::new(reply_to))
        .caption("🖼️ *Desktop wallpaper*")
        .await?;

    Ok(())
}
