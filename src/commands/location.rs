use anyhow::Result;
use serde_json::Value;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn location(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = match client.get("http://ip-api.com/json/").send().await {
        Ok(r) => r,
        Err(e) => {
            md::reply_error(bot, chat_id, reply_to, "Kết nối thất bại", e).await?;
            return Ok(());
        }
    };

    let body = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            md::reply_error(bot, chat_id, reply_to, "Không đọc được response", e).await?;
            return Ok(());
        }
    };

    let data: Value = match serde_json::from_str(&body) {
        Ok(d) => d,
        Err(e) => {
            md::reply_error(bot, chat_id, reply_to, "JSON không hợp lệ", e).await?;
            return Ok(());
        }
    };

    // ip-api.com returns {"status":"success",...} or {"status":"fail","message":"..."}
    if data["status"].as_str() == Some("fail") {
        let msg = data["message"].as_str().unwrap_or("unknown error");
        md::reply_error(bot, chat_id, reply_to, "API lỗi", msg).await?;
        return Ok(());
    }

    let ip = data["query"].as_str().unwrap_or("?");
    let country = data["country"].as_str().unwrap_or("?");
    let city = data["city"].as_str().unwrap_or("?");
    let isp = data["isp"].as_str().unwrap_or("?");
    let lat = data["lat"].as_f64().unwrap_or(0.0);
    let lon = data["lon"].as_f64().unwrap_or(0.0);

    let text = format!(
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
        md::escape(&lat.to_string()),
        md::escape(&lon.to_string()),
    );

    md::send(bot, chat_id, reply_to, text).await
}
