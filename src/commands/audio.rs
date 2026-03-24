use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn mute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--audio".to_string(), "mute".to_string()];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 5000))
            .await??;
    } else {
        do_mute()?;
    }
    md::send(bot, chat_id, reply_to, "🔇 Đã tắt âm".to_string()).await
}

pub async fn unmute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--audio".to_string(), "unmute".to_string()];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 5000))
            .await??;
    } else {
        do_unmute()?;
    }
    md::send(bot, chat_id, reply_to, "🔊 Đã bật âm".to_string()).await
}

pub async fn set_volume(bot: &Bot, chat_id: ChatId, reply_to: MessageId, level: u8) -> Result<()> {
    let level = level.min(100);
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--audio".to_string(), "set".to_string(), level.to_string()];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 5000))
            .await??;
    } else {
        do_set_volume(level)?;
    }
    md::send(bot, chat_id, reply_to, format!("🔊 Âm lượng: {}\\%", level)).await
}

pub fn do_mute() -> Result<()> {
    send_volume_keys("{VOLUME_MUTE}")
}

pub fn do_unmute() -> Result<()> {
    send_volume_keys("{VOLUME_MUTE}")
}

pub fn do_set_volume(level: u8) -> Result<()> {
    let down_count = 50;
    let up_count = (level as f64 / 3.23) as u32;
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
for ($i = 0; $i -lt {down_count}; $i++) {{
    [System.Windows.Forms.SendKeys]::SendWait('{{VOLUME_DOWN}}')
    Start-Sleep -Milliseconds 5
}}
Start-Sleep -Milliseconds 50
for ($i = 0; $i -lt {up_count}; $i++) {{
    [System.Windows.Forms.SendKeys]::SendWait('{{VOLUME_UP}}')
    Start-Sleep -Milliseconds 10
}}
"#
    );
    run_powershell(&script)
}

fn send_volume_keys(key: &str) -> Result<()> {
    let script = format!(
        r#"Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('{key}')"#
    );
    run_powershell(&script)
}

fn run_powershell(script: &str) -> Result<()> {
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
    Ok(())
}
