use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn wallpaper(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let path = match read_wallpaper_path() {
        Ok(p) => p,
        Err(e) => {
            md::reply_error(bot, chat_id, reply_to, "Không đọc được registry", e).await?;
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

    md::send_document(
        bot,
        chat_id,
        reply_to,
        teloxide::types::InputFile::file(file),
        "🖼️ *Desktop wallpaper*",
    )
    .await
}

fn read_wallpaper_path() -> Result<String> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(r"Control Panel\Desktop")?;
    let path: String = key.get_value("WallPaper")?;
    Ok(path)
}
