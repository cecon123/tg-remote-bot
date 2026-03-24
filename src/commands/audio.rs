use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

fn run_powershell_sync(script: &str) -> Result<String> {
    let output = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
        .context("cannot run powershell")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("PowerShell failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub async fn mute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(|| {
        run_powershell_sync(
            r#"Add-Type -Name Audio -Namespace Win -MemberDefinition '[DllImport("winmm.dll")] public static extern int waveOutSetVolume(IntPtr h, uint v);'; [Win.Audio]::waveOutSetVolume([IntPtr]::Zero, 0)"#,
        )
    })
    .await?
    .context("cannot mute")?;
    md::send(bot, chat_id, reply_to, "🔇 Đã tắt âm".to_string()).await
}

pub async fn unmute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(|| {
        run_powershell_sync(
            r#"Add-Type -Name Audio -Namespace Win -MemberDefinition '[DllImport("winmm.dll")] public static extern int waveOutSetVolume(IntPtr h, uint v);'; [Win.Audio]::waveOutSetVolume([IntPtr]::Zero, 0xFFFF)"#,
        )
    })
    .await?
    .context("cannot unmute")?;
    md::send(bot, chat_id, reply_to, "🔊 Đã bật âm".to_string()).await
}

pub async fn set_volume(bot: &Bot, chat_id: ChatId, reply_to: MessageId, level: u8) -> Result<()> {
    let level = level.min(100);
    let vol = (level as u32 * 0xFFFF) / 100;
    let packed = (vol << 16) | vol;
    let script = format!(
        r#"Add-Type -Name Audio -Namespace Win -MemberDefinition '[DllImport("winmm.dll")] public static extern int waveOutSetVolume(IntPtr h, uint v);'; [Win.Audio]::waveOutSetVolume([IntPtr]::Zero, {packed})"#
    );

    tokio::task::spawn_blocking(move || run_powershell_sync(&script))
        .await?
        .context("cannot set volume")?;

    md::send(bot, chat_id, reply_to, format!("🔊 Âm lượng: {}\\%", level)).await
}
