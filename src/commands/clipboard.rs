use anyhow::Result;
use clipboard_win::{formats, get_clipboard};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::{md, truncate_and_escape};

pub async fn clipboard(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let text: String = get_clipboard(formats::Unicode).unwrap_or_default();

    if text.is_empty() {
        md::send(bot, chat_id, reply_to, "📭 Clipboard trống".to_string()).await?;
    } else {
        let escaped = truncate_and_escape(&text, md::MAX_MSG_BYTES);
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
