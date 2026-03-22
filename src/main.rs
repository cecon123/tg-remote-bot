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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--run") => {
            check_already_running()?;
            let (tx, rx) = mpsc::channel();
            // Keep tx alive on a background thread so channel stays open
            thread::spawn(move || {
                let _keep = tx;
                thread::park();
            });
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(bot::run_until(rx))
        }
        Some("--install") => {
            if !is_admin() {
                anyhow::bail!("--install cần quyền Administrator. Hãy chạy cmd với Run as administrator.");
            }
            check_already_running()?;
            if args.len() >= 4 {
                let token = &args[2];
                let uid: i64 = args[3].parse()?;
                service::install::install(token, uid)?;
            } else {
                let cfg = service::config::load()?;
                service::install::install(&cfg.bot_token, cfg.super_user_id)?;
            }
            Ok(())
        }
        Some("--reinstall") => {
            if args.len() < 4 {
                anyhow::bail!("Usage: --reinstall TOKEN UID");
            }
            let token = &args[2];
            let uid: i64 = args[3].parse()?;
            service::config::save_to_registry(token, uid)?;
            println!("Registry updated");
            Ok(())
        }
        Some("--uninstall") => {
            service::install::uninstall()?;
            Ok(())
        }
        Some("--help") | Some("-h") => {
            println!("TG Remote Bot v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage:");
            println!("  --run                    Run bot in foreground (debug)");
            println!("  --install [TOKEN UID]    Install as Windows Service (admin required)");
            println!("  --reinstall TOKEN UID    Update registry config");
            println!("  --uninstall              Remove Windows Service");
            println!("  --help                   Show this help");
            Ok(())
        }
        _ => {
            service::windows_svc::dispatch()
        }
    }
}
