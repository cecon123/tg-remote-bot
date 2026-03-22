use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn msgbox(
    bot: &Bot,
    chat_id: ChatId,
    reply_to: MessageId,
    text: &str,
) -> Result<()> {
    md::send(bot, chat_id, reply_to, "💬 MessageBox đang hiện\\.\\.\\.".to_string()).await?;

    let title = "TgRemoteAgent";
    let wide_text: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    let result = tokio::task::spawn_blocking(move || unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
            std::ptr::null_mut(),
            wide_text.as_ptr(),
            wide_title.as_ptr(),
            windows_sys::Win32::UI::WindowsAndMessaging::MB_OK
                | windows_sys::Win32::UI::WindowsAndMessaging::MB_TOPMOST,
        )
    })
    .await?;

    let response = match result {
        1 => "✅ User đã bấm OK",
        _ => "ℹ️ MessageBox đã đóng",
    };

    md::send(bot, chat_id, reply_to, response.to_string()).await
}
