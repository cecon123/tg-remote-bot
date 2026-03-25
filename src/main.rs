use std::env;
use std::sync::mpsc;
use std::thread;

use anyhow::Result;

mod bot;
mod commands;
mod security;
mod service;
mod updater;

/// Ensure only one instance is running via a named mutex.
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

/// Run the bot with a shutdown channel that can be triggered externally.
fn run_bot(cfg: service::config::AppConfig, enable_ctrlc: bool) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _keep = tx;
        thread::park();
    });
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(bot::run_until(rx, cfg, enable_ctrlc))
}

fn run_daemon() -> Result<()> {
    let home = security::obfuscation::install_home();
    if let Err(e) = service::logging::init_logger(home, service::logging::LogMode::Service) {
        eprintln!("Cannot init logger: {e:?}");
    }

    check_already_running()?;
    service::install::cleanup_old_files();

    let cfg = service::config::load()?;
    let rt = tokio::runtime::Runtime::new()?;

    match rt.block_on(updater::self_update::auto_update()) {
        Ok(true) => {
            log::info!("Updated, process exiting for restart");
            return Ok(());
        }
        Err(e) => log::warn!("Auto-update failed: {e:?}"),
        Ok(false) => {}
    }

    run_bot(cfg, false)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--daemon") => run_daemon(),

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
            run_bot(cfg, true)
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
            service::install::install(&args[2], args[3].parse()?)
        }

        Some("--reinstall") => {
            if args.len() < 4 {
                anyhow::bail!("Usage: --reinstall TOKEN UID");
            }
            init_foreground_logger();
            service::config::save_to_registry(&args[2], args[3].parse()?)?;
            log::info!("Registry updated");
            Ok(())
        }

        Some("--uninstall") => {
            init_foreground_logger();
            service::install::uninstall()
        }

        Some("--help") | Some("-h") => {
            println!("TG Remote Bot v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage:");
            println!("  --daemon                 Run as daemon (Task Scheduler)");
            println!("  --run TOKEN UID          Run in foreground (debug)");
            println!("  --install TOKEN UID      Install via Task Scheduler (admin)");
            println!("  --reinstall TOKEN UID    Update registry config");
            println!("  --uninstall              Remove Task Scheduler task");
            println!("  --help                   Show this help");
            Ok(())
        }

        _ => {
            println!("No command specified. Use --help for usage.");
            Ok(())
        }
    }
}
