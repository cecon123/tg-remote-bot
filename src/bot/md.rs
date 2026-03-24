use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId, ParseMode, ReplyParameters};

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

#[allow(dead_code)]
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
