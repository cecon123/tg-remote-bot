use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

async fn run_powershell(script: &str) -> Result<String> {
    let output = tokio::task::spawn_blocking({
        let script = script.to_string();
        move || {
            std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &script])
                .output()
        }
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("PowerShell failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub async fn mute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    run_powershell("(New-Object -ComObject WScript.Shell).SendKeys([char]173)")
        .await
        .context("cannot send mute key")?;
    md::send(bot, chat_id, reply_to, "🔇 Đã tắt âm".to_string()).await
}

pub async fn unmute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let check = run_powershell(
        "(Get-AudioDevice -Playback).Mute -eq 'True'",
    )
    .await
    .unwrap_or_default();

    if check == "True" {
        run_powershell("(New-Object -ComObject WScript.Shell).SendKeys([char]173)")
            .await
            .context("cannot send mute key")?;
    }
    md::send(bot, chat_id, reply_to, "🔊 Đã bật âm".to_string()).await
}

pub async fn set_volume(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    level: u8,
) -> Result<()> {
    let level = level.min(100);
    let script = format!(
        r#"
        $vol = [Math]::Round({level} * 655.35)
        $wsh = New-Object -ComObject WScript.Shell
        for ($i = 0; $i -lt 50; $i++) {{ $wsh.SendKeys([char]174) }}
        for ($i = 0; $i -lt [Math]::Floor($vol / 655.35); $i++) {{ $wsh.SendKeys([char]175) }}
        "#
    );

    run_powershell(&script)
        .await
        .context("cannot set volume")?;

    md::send(
        bot,
        chat_id,
        reply_to,
        format!("🔊 Âm lượng: `{level}`%"),
    )
    .await
}
