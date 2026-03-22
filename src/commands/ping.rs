use std::net::UdpSocket;

use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::{md, AgentState};

pub async fn ping(bot: &Bot, chat_id: ChatId, reply_to: MessageId, state: &AgentState) -> Result<()> {
    let ip = local_ip().unwrap_or_else(|| "unknown".to_string());
    let uptime = crate::bot::format_duration(state.start_time.elapsed().as_secs());

    let text = format!(
        "*🏓 PONG\\!*\n\n\
        🌐 IP: `{}`\n\
        ⏱️ Uptime: {}\n\
        📦 Version: `{}`",
        md::escape(&ip),
        md::escape(&uptime),
        md::escape(state.agent_version)
    );

    md::send(bot, chat_id, reply_to, text).await
}

fn local_ip() -> Option<String> {
    let s = UdpSocket::bind("0.0.0.0:0").ok()?;
    s.connect("8.8.8.8:80").ok()?;
    Some(s.local_addr().ok()?.ip().to_string())
}
