use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::{AgentState, md};

pub async fn status(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    state: &AgentState,
) -> Result<()> {
    let pid = std::process::id();
    let uptime = crate::bot::format_duration(state.start_time.elapsed().as_secs());

    let job_status = {
        let job = state.active_job.lock().unwrap();
        if job.is_some() {
            "🔴 *Running*"
        } else {
            "🟢 *Idle*"
        }
    };

    let text = format!(
        "*📊 Agent Status*\n\n\
        🆔 PID: `{pid}`\n\
        ⏱️ Uptime: {}\n\
        💻 Shell: {job_status}\n\
        📦 Version: `{}`",
        md::escape(&uptime),
        md::escape(state.agent_version)
    );

    md::send(bot, chat_id, reply_to, text).await
}
