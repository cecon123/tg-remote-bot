use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn wallpaper(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let path = if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--wallpaper".to_string()];
        match tokio::task::spawn_blocking(move || {
            session::capture_in_user_session(&exe, args, 10000)
        })
        .await
        {
            Ok(Ok((_, output))) => {
                let p = String::from_utf8_lossy(&output).trim().to_string();
                if p.is_empty() {
                    md::send(bot, chat_id, reply_to, "❌ Không có wallpaper".to_string()).await?;
                    return Ok(());
                }
                p
            }
            _ => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    "❌ Không đọc được wallpaper".to_string(),
                )
                .await?;
                return Ok(());
            }
        }
    } else {
        match read_wallpaper_path() {
            Ok(p) => p,
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

fn read_wallpaper_path() -> Result<String> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(r"Control Panel\Desktop")?;
    let path: String = key.get_value("WallPaper")?;
    Ok(path)
}

pub fn print_wallpaper_path() {
    if let Ok(p) = read_wallpaper_path() {
        print!("{p}");
    }
}
