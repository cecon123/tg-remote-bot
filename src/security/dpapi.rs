use anyhow::{Context, Result};
use base64::Engine;
use windows_sys::Win32::Foundation::LocalFree;
use windows_sys::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB,
};

pub fn protect(data: &[u8]) -> Result<String> {
    let blob_in = CRYPT_INTEGER_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut blob_out = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptProtectData(
            &blob_in,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            0,
            &mut blob_out,
        )
    };
    if ok == 0 {
        anyhow::bail!("CryptProtectData failed");
    }

    let slice = unsafe { std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize) };
    let encoded = base64::engine::general_purpose::STANDARD.encode(slice);
    unsafe { LocalFree(blob_out.pbData as _) };

    Ok(encoded)
}

pub fn unprotect(encoded: &str) -> Result<Vec<u8>> {
    let raw = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .context("base64 decode failed")?;

    let blob_in = CRYPT_INTEGER_BLOB {
        cbData: raw.len() as u32,
        pbData: raw.as_ptr() as *mut u8,
    };
    let mut blob_out = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let ok = unsafe {
        CryptUnprotectData(
            &blob_in,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            0,
            &mut blob_out,
        )
    };
    if ok == 0 {
        anyhow::bail!("CryptUnprotectData failed");
    }

    let result =
        unsafe { std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize) }.to_vec();
    unsafe { LocalFree(blob_out.pbData as _) };

    Ok(result)
}
