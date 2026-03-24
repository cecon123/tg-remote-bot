use std::process::Stdio;

use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::bot::{ActiveJob, RunningJob, md};

pub async fn shell(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    active_job: &ActiveJob,
    cmd: &str,
) -> Result<()> {
    let already_running = {
        let job = active_job.lock().unwrap_or_else(|e| e.into_inner());
        job.is_some()
    };
    if already_running {
        md::send(
            bot,
            chat_id,
            reply_to,
            "⚠️ Job đang chạy\\. Dùng /cancel trước".to_string(),
        )
        .await?;
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
            md::send(
                bot,
                chat_id,
                reply_to,
                format!("❌ {}", md::escape(&format!("Không thể chạy lệnh: {e}"))),
            )
            .await?;
            return Ok(());
        }
    };

    let pid = child.id().unwrap_or(0);
    md::send(
        bot,
        chat_id,
        reply_to,
        format!("▶️ Đang chạy\\.\\.\\. PID: `{pid}`"),
    )
    .await?;

    let bot_clone = bot.clone();
    let chat = chat_id;
    let rto = reply_to;

    let handle = tokio::spawn(async move {
        let mut stdout_buf = String::new();
        let mut stderr_buf = String::new();

        let stdout_handle = child.stdout.take().map(|stdout| {
            tokio::spawn(async move {
                let mut buf = String::new();
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    buf.push_str(&line);
                    buf.push('\n');
                    if buf.len() > 3800 {
                        break;
                    }
                }
                buf
            })
        });

        let stderr_handle = child.stderr.take().map(|stderr| {
            tokio::spawn(async move {
                let mut buf = String::new();
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    buf.push_str("ERR: ");
                    buf.push_str(&line);
                    buf.push('\n');
                    if buf.len() > 3800 {
                        break;
                    }
                }
                buf
            })
        });

        if let Some(h) = stdout_handle {
            stdout_buf = h.await.unwrap_or_default();
        }
        if let Some(h) = stderr_handle {
            stderr_buf = h.await.unwrap_or_default();
        }

        let _ = child.wait().await;

        let mut output = stdout_buf;
        output.push_str(&stderr_buf);

        if output.is_empty() {
            output = "(no output)".to_string();
        }
        let truncated = crate::bot::truncate_str(&output, 3800);
        let suffix = if truncated.len() < output.len() {
            "\n...(truncated)"
        } else {
            ""
        };
        let escaped = md::escape(&format!("{truncated}{suffix}"));

        let _ = md::send(&bot_clone, chat, rto, format!("📤 *Output:*\n\n{escaped}")).await;
    });

    {
        let mut job = active_job.lock().unwrap_or_else(|e| e.into_inner());
        *job = Some(RunningJob { pid, handle });
    }

    Ok(())
}
