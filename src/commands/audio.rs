use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

// winmm.dll FFI for volume control
unsafe extern "system" {
    fn waveOutSetVolume(hwo: *const u8, dw_volume: u32) -> i32;
}

/// Set system volume to `level` (0-100).
fn do_set_volume(level: u8) -> Result<()> {
    let val = (level.min(100) as u32) * 0xFFFF / 100;
    let packed = (val << 16) | val; // left + right channels
    let rc = unsafe { waveOutSetVolume(std::ptr::null(), packed) };
    if rc != 0 {
        anyhow::bail!("waveOutSetVolume failed: {rc}");
    }
    Ok(())
}

/// Toggle mute via VK_VOLUME_MUTE key simulation.
fn do_toggle_mute() -> Result<()> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VK_VOLUME_MUTE,
    };

    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_VOLUME_MUTE, wScan: 0, dwFlags: 0, time: 0, dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_VOLUME_MUTE, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0,
                },
            },
        },
    ];

    let sent = unsafe { SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32) };
    if sent == 0 {
        anyhow::bail!("SendInput failed");
    }
    Ok(())
}

pub async fn mute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(do_toggle_mute).await??;
    md::send(bot, chat_id, reply_to, "🔇 Đã tắt âm".to_string()).await
}

pub async fn unmute(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    tokio::task::spawn_blocking(do_toggle_mute).await??;
    md::send(bot, chat_id, reply_to, "🔊 Đã bật âm".to_string()).await
}

pub async fn set_volume_cmd(bot: &Bot, chat_id: ChatId, reply_to: MessageId, level: u8) -> Result<()> {
    let level = level.min(100);
    tokio::task::spawn_blocking(move || do_set_volume(level)).await??;
    md::send(bot, chat_id, reply_to, format!("🔊 Âm lượng: {}\\%", level)).await
}
