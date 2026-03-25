use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId};

use crate::bot::md;

pub async fn camera(bot: &Bot, chat_id: ChatId, reply_to: MessageId) -> Result<()> {
    let jpeg_buf = match tokio::task::spawn_blocking(capture_camera).await {
        Ok(Ok(buf)) => buf,
        Ok(Err(e)) => {
            md::reply_error(bot, chat_id, reply_to, "Camera lỗi", e).await?;
            return Ok(());
        }
        Err(e) => {
            md::reply_error(bot, chat_id, reply_to, "Task error", e).await?;
            return Ok(());
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
