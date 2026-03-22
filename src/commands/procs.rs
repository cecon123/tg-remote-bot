use anyhow::Result;
use sysinfo::{Process, System};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn procs(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<&Process> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    let lines: Vec<String> = processes
        .iter()
        .take(30)
        .map(|p| {
            format!(
                "`{}` {} \\- {} MB",
                p.pid(),
                md::escape(&p.name().to_string_lossy()),
                p.memory() / 1024 / 1024
            )
        })
        .collect();

    let text = if lines.is_empty() {
        "📭 Không có process nào".to_string()
    } else {
        format!("*📋 Process List*\n\n{}", lines.join("\n"))
    };

    md::send(bot, chat_id, reply_to, text).await
}

pub async fn kill_process(bot: &Bot, chat_id: ChatId, reply_to: MessageId, pid: u32) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        let name = process.name().to_string_lossy().to_string();
        if process.kill() {
            md::send(bot, chat_id, reply_to, format!("💀 Đã kill: {} \\(PID `{pid}`\\)", md::escape(&name))).await?;
        } else {
            md::send(bot, chat_id, reply_to, format!("❌ Kill thất bại: PID `{pid}`")).await?;
        }
    } else {
        md::send(bot, chat_id, reply_to, format!("❌ Không tìm thấy PID `{pid}`")).await?;
    }

    Ok(())
}
