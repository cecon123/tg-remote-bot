use anyhow::{Context, Result};
use screenshots::Screen;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;

pub async fn screenshot(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let jpeg_buf = match capture_screen() {
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
    let mut encoder = screenshots::image::codecs::jpeg::JpegEncoder::new_with_quality(
        &mut jpeg_buf,
        75,
    );
    encoder.encode_image(&rgb)?;

    Ok(jpeg_buf)
}
