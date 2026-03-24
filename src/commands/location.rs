use anyhow::Result;
use serde_json::Value;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn location(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let resp = client
        .get("https://ip-api.com/json/")
        .send()
        .await?
        .text()
        .await?;

    let text = match serde_json::from_str::<Value>(&resp) {
        Ok(data) => {
            let ip = data["query"].as_str().unwrap_or("?");
            let country = data["country"].as_str().unwrap_or("?");
            let city = data["city"].as_str().unwrap_or("?");
            let isp = data["isp"].as_str().unwrap_or("?");
            let lat = data["lat"].as_f64().unwrap_or(0.0);
            let lon = data["lon"].as_f64().unwrap_or(0.0);
            format!(
                "*📍 Vị trí IP:*\n\n\
                🌐 IP: `{}`\n\
                🏳️ Quốc gia: {}\n\
                🏙️ Thành phố: {}\n\
                🏢 ISP: {}\n\
                📌 Tọa độ: {}\\, {}",
                md::escape(ip),
                md::escape(country),
                md::escape(city),
                md::escape(isp),
                lat,
                lon,
            )
        }
        Err(_) => format!("*📍 Vị trí IP:*\n\n{}", md::escape(&resp)),
    };

    md::send(bot, chat_id, reply_to, text).await?;
    Ok(())
}
