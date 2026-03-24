use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn msgbox(bot: &Bot, chat_id: ChatId, reply_to: MessageId, text: &str) -> Result<()> {
    md::send(
        bot,
        chat_id,
        reply_to,
        "💬 MessageBox đang hiện\\.\\.\\.".to_string(),
    )
    .await?;

    let text = text.to_string();
    if session::is_system_session() {
        let exe = std::env::current_exe()?;
        let args = vec!["--msgbox".into(), text];
        tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 30000))
            .await??;
    } else {
        show_blocking(&text);
    }

    md::send(bot, chat_id, reply_to, "✅ MessageBox đã đóng".to_string()).await
}

pub fn show_blocking(text: &str) {
    let wide_text: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let title = "Meow Meow~";
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
            std::ptr::null_mut(),
            wide_text.as_ptr(),
            wide_title.as_ptr(),
            windows_sys::Win32::UI::WindowsAndMessaging::MB_OK
                | windows_sys::Win32::UI::WindowsAndMessaging::MB_TOPMOST,
        );
    }
}
