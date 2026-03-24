use std::path::Path;

use anyhow::{Context, Result};
use screenshots::Screen;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn screenshot(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let jpeg_buf = if session::is_system_session() {
        let tmp = std::env::temp_dir().join("_screenshot_tmp.jpg");
        let tmp_path = tmp.to_string_lossy().to_string();
        let exe = std::env::current_exe().context("cannot get exe path")?;
        let args = vec!["--screenshot".into(), tmp_path.clone()];

        match tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 15000))
            .await
        {
            Ok(Ok(_)) => {
                let data = std::fs::read(&tmp).context("cannot read screenshot output")?;
                let _ = std::fs::remove_file(&tmp);
                data
            }
            Ok(Err(e)) => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    format!("❌ {}", md::escape(&format!("Lỗi capture: {e}"))),
                )
                .await?;
                return Ok(());
            }
            Err(e) => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    format!("❌ {}", md::escape(&format!("Task error: {e}"))),
                )
                .await?;
                return Ok(());
            }
        }
    } else {
        match capture_screen() {
            Ok(buf) => buf,
            Err(e) => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    format!("❌ {}", md::escape(&format!("Lỗi capture: {e}"))),
                )
                .await?;
                return Ok(());
            }
        }
    };

    md::send_photo(bot, chat_id, reply_to, InputFile::memory(jpeg_buf)).await?;
    Ok(())
}

fn capture_screen() -> Result<Vec<u8>> {
    let screens = Screen::all().context("Không tìm thấy màn hình")?;
    let screen = screens.first().context("Không có màn hình nào")?;
    let img = screen.capture().context("Capture màn hình thất bại")?;

    let rgb = screenshots::image::DynamicImage::ImageRgba8(img).to_rgb8();
    let mut jpeg_buf = Vec::new();
    let mut encoder =
        screenshots::image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_buf, 75);
    encoder.encode_image(&rgb)?;
    Ok(jpeg_buf)
}

pub fn capture_to_file(path: &str) -> Result<()> {
    let jpeg = capture_screen()?;
    std::fs::write(Path::new(path), &jpeg).context("cannot write screenshot file")?;
    Ok(())
}
