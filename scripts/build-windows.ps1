<#
.SYNOPSIS
    Builds Antares ThreadClip Studio on Windows (Local Developer Script)
#>

$ErrorActionPreference = "Stop"

Write-Host "=== Antares ThreadClip Studio - Windows Build Script ===" -ForegroundColor Cyan

# 1. Check Node.js and npm
Write-Host "[1/5] Checking Node.js..." -ForegroundColor Yellow
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Error "Node.js is not found. Please install Node.js 20+."
    exit 1
}
$nodeVer = node -v
Write-Host "Node.js version: $nodeVer" -ForegroundColor Green

# 2. Check Rust & Linker
Write-Host "[2/5] Checking Rust compiler..." -ForegroundColor Yellow
$cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoCmd) {
    $cargoCmd = "$env:USERPROFILE\.cargo\bin\cargo.exe"
}
if (-not (Test-Path $cargoCmd)) {
    Write-Error "Cargo/Rust is not found. Please install Rust via https://rustup.rs"
    exit 1
}
$rustVer = & $cargoCmd --version
Write-Host "Rust version: $rustVer" -ForegroundColor Green

# Check if Visual C++ linker is available
if (-not (Get-Command link.exe -ErrorAction SilentlyContinue)) {
    Write-Warning @"
Visual Studio C++ Linker (link.exe) was not found in PATH.
If local cargo build fails, please install Visual Studio Build Tools via winget:
  winget install Microsoft.VisualStudio.2022.BuildTools --force --override "--passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
OR download the pre-built Windows package automatically built by GitHub Actions:
  https://github.com/dhapidpad/antares-threadclip/actions
"@
}

# 3. Build UI
Write-Host "[3/5] Building React Studio Frontend..." -ForegroundColor Yellow
Push-Location "ui"
try {
    npm install
    npm run build
} finally {
    Pop-Location
}
Write-Host "UI build completed: ui/dist" -ForegroundColor Green

# 4. Build Rust Backend
Write-Host "[4/5] Building Rust Backend (tc-app)..." -ForegroundColor Yellow
& $cargoCmd build --release --bin tc-app

# 5. Assemble Distribution Folder
Write-Host "[5/5] Assembling Windows Distribution Package..." -ForegroundColor Yellow
$distDir = "dist\antares-threadclip-windows-x64"
New-Item -ItemType Directory -Force -Path $distDir | Out-Null
New-Item -ItemType Directory -Force -Path "$distDir\bin" | Out-Null

Copy-Item "target\release\tc-app.exe" "$distDir\tc-app.exe"
Copy-Item -Recurse -Force "ui\dist" "$distDir\ui"
Copy-Item -Recurse -Force "assets" "$distDir\assets"
if (Test-Path "testdata") { Copy-Item -Recurse -Force "testdata" "$distDir\testdata" }
Copy-Item -Force "README.md" "$distDir\README.md"
if (Test-Path "LICENSE") { Copy-Item -Force "LICENSE" "$distDir\LICENSE" }

# Copy local ffmpeg if found
$localFfmpeg = Get-Command ffmpeg -ErrorAction SilentlyContinue
if ($localFfmpeg) {
    Copy-Item $localFfmpeg.Source "$distDir\bin\ffmpeg.exe"
    Write-Host "Bundled local FFmpeg: $($localFfmpeg.Source)" -ForegroundColor Green
} else {
    Write-Warning "FFmpeg not found in PATH. Please place ffmpeg.exe inside $distDir\bin\ before rendering."
}

@"
@echo off
title Antares ThreadClip Studio v3.0.0
cd /d "%~dp0"
echo ===============================================================
echo       Antares ThreadClip Studio v3.0.0 (Windows Launcher)
echo ===============================================================
echo.
echo Dang khoi dong may chu backend va giao dien Studio...
echo (Vui long KHONG dong cua so nay trong khi su dung Studio)
echo.
"%~dp0tc-app.exe"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [LOI] Studio da dung voi ma loi %ERRORLEVEL%.
)
echo.
echo Studio da dung.
pause
"@ | Out-File -Encoding utf8 "$distDir\run.bat"

Write-Host "SUCCESS! Windows Studio package assembled at: $distDir" -ForegroundColor Cyan
Write-Host "Run '$distDir\run.bat' to launch the Studio." -ForegroundColor Cyan
