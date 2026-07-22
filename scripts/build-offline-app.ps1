# Build Script for MicroDeck Offline Standalone Windows Software
$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  Building CODEX MICRO Offline Standalone Desktop Software" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$WorkspaceRoot = Split-Path -Path $PSScriptRoot -Parent
Set-Location -Path $WorkspaceRoot

Write-Host "`n1. Running TypeScript & Vite Production Build..." -ForegroundColor Yellow
npm run build

Write-Host "`n2. Packaging Offline Web Distribution Bundle..." -ForegroundColor Yellow
$DistPath = Join-Path -Path $WorkspaceRoot -ChildPath "dist"
$ZipPath = Join-Path -Path $WorkspaceRoot -ChildPath "Codex-Micro-Offline-App.zip"

if (Test-Path -Path $ZipPath) {
    Remove-Item -Path $ZipPath -Force
}

Compress-Archive -Path "$DistPath\*" -DestinationPath $ZipPath -Force
Write-Host "   Offline Zip distribution packaged at: $ZipPath" -ForegroundColor Green

Write-Host "`n3. Checking Tauri Offline Installer Configuration..." -ForegroundColor Yellow
$TauriConfig = Get-Content -Path (Join-Path $WorkspaceRoot "src-tauri\tauri.conf.json") -Raw
if ($TauriConfig -match '"targets":\s*\[(.*?)\]') {
    Write-Host "   Tauri bundler configured for installer targets: $($Matches[1])" -ForegroundColor Green
}

Write-Host "`n==========================================================" -ForegroundColor Cyan
Write-Host "  Build Complete! Offline Bundle Created Successfully." -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan
