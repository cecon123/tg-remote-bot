use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};

use crate::bot::md;

pub async fn screenshot(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let screens = match screenshots::Screen::all() {
        Ok(s) => s,
        Err(e) => {
            md::send(bot, chat_id, reply_to, format!("❌ {}", md::escape(&format!("Lỗi chụp màn hình: {e}")))).await?;
            return Ok(());
        }
    };

    if let Some(screen) = screens.first() {
        let capture = match screen.capture() {
            Ok(c) => c,
            Err(e) => {
                md::send(bot, chat_id, reply_to, format!("❌ {}", md::escape(&format!("Lỗi capture: {e}")))).await?;
                return Ok(());
            }
        };
        let (width, height) = capture.dimensions();
        let rgba = capture.to_vec();

        let img = match image::RgbaImage::from_raw(width, height, rgba) {
            Some(i) => i,
            None => {
                md::send(bot, chat_id, reply_to, "❌ Lỗi xử lý ảnh".to_string()).await?;
                return Ok(());
            }
        };

        let rgb = image::DynamicImage::ImageRgba8(img).to_rgb8();
        let mut jpeg_buf = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_buf, 75);
        encoder.encode_image(&rgb)?;

        md::send_photo(bot, chat_id, reply_to, InputFile::memory(jpeg_buf)).await?;
    } else {
        md::send(bot, chat_id, reply_to, "❌ Không tìm thấy màn hình".to_string()).await?;
    }

    Ok(())
}
