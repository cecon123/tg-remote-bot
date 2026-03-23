use std::process::Stdio;

use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::bot::{md, ActiveJob, RunningJob};

pub async fn shell(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    active_job: &ActiveJob,
    cmd: &str,
) -> Result<()> {
    let already_running = {
        let job = active_job.lock().unwrap();
        job.is_some()
    };
    if already_running {
        md::send(bot, chat_id, reply_to, "⚠️ Job đang chạy\\. Dùng /cancel trước".to_string()).await?;
        return Ok(());
    }

    let mut child = match Command::new("cmd")
        .args(["/C", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            md::send(bot, chat_id, reply_to, format!("❌ {}", md::escape(&format!("Không thể chạy lệnh: {e}")))).await?;
            return Ok(());
        }
    };

    let pid = child.id().unwrap_or(0);
    md::send(bot, chat_id, reply_to, format!("▶️ Đang chạy\\.\\.\\. PID: `{pid}`")).await?;

    let bot_clone = bot.clone();
    let chat = chat_id;
    let rto = reply_to;

    let handle = tokio::spawn(async move {
        let mut output = String::new();

        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                output.push_str(&line);
                output.push('\n');
                if output.len() > 3800 { break; }
            }
        }

        if let Some(stderr) = child.stderr.take() {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                output.push_str("ERR: ");
                output.push_str(&line);
                output.push('\n');
                if output.len() > 3800 { break; }
            }
        }

        let _ = child.wait().await;

        if output.is_empty() { output = "(no output)".to_string(); }
        let truncated = crate::bot::truncate_str(&output, 3800);
        let suffix = if truncated.len() < output.len() { "\n...(truncated)" } else { "" };
        let escaped = md::escape(&format!("{truncated}{suffix}"));

        let _ = md::send(&bot_clone, chat, rto, format!("📤 *Output:*\n\n{escaped}")).await;
    });

    {
        let mut job = active_job.lock().unwrap();
        *job = Some(RunningJob { pid, handle });
    }

    Ok(())
}
