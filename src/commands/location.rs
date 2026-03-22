use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn location(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let resp = client
        .get("http://ip-api.com/json/")
        .send()
        .await?
        .text()
        .await?;

    md::send(bot, chat_id, reply_to, format!("*📍 Vị trí IP:*\n\n{}", md::escape(&resp))).await?;
    Ok(())
}
