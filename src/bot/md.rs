use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId, ParseMode, ReplyParameters};

/// Max byte length for Telegram messages to avoid truncation at API level.
pub const MAX_MSG_BYTES: usize = 3800;

/// Telegram MarkdownV2 special characters that require escaping.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 10);
    for ch in text.chars() {
        match ch {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

/// Send a MarkdownV2 message, replying to the given message.
pub async fn send(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    text: impl Into<String>,
) -> anyhow::Result<()> {
    bot.send_message(chat_id, text.into())
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(reply_to))
        .await?;
    Ok(())
}

/// Reply with an error message. Automatically escapes dynamic content.
pub async fn reply_error(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    label: &str,
    err: impl std::fmt::Display,
) -> anyhow::Result<()> {
    send(
        bot,
        chat_id,
        reply_to,
        format!("❌ {}", escape(&format!("{label}: {err}"))),
    )
    .await
}

/// Send a photo, replying to the given message.
pub async fn send_photo(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    photo: InputFile,
) -> anyhow::Result<()> {
    bot.send_photo(chat_id, photo)
        .reply_parameters(ReplyParameters::new(reply_to))
        .await?;
    Ok(())
}

/// Send a document with MarkdownV2 caption, replying to the given message.
pub async fn send_document(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    doc: InputFile,
    caption: impl Into<String>,
) -> anyhow::Result<()> {
    bot.send_document(chat_id, doc)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(reply_to))
        .caption(caption.into())
        .await?;
    Ok(())
}
