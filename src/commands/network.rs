use anyhow::Result;
use sysinfo::Networks;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn netstat(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let mut lines = Vec::new();
    for (name, data) in &Networks::new_with_refreshed_list() {
        let ips: Vec<String> = data
            .ip_networks()
            .iter()
            .map(|ip| md::escape(&ip.addr.to_string()))
            .collect();
        lines.push(format!(
            "*🌐 {}*\n  {}\n  ↓{} MB ↑{} MB",
            md::escape(name),
            ips.join("\\, "),
            data.total_received() / 1024 / 1024,
            data.total_transmitted() / 1024 / 1024,
        ));
    }

    let text = if lines.is_empty() {
        "📭 Không có interface mạng nào".to_string()
    } else {
        lines.join("\n\n")
    };

    md::send(bot, chat_id, reply_to, text).await
}
