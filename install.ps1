# One-step installer for Bolo on Windows: builds the app, installs it with a Start Menu shortcut,
# and installs the AI parts (voice, speech recognition, chat model).
# Usage: .\install.ps1            full install
#        .\install.ps1 -NoAi      app only (skip the ~2 GB AI download)
param([switch]$NoAi)
$ErrorActionPreference = "Stop"
if ($env:BOLO_NO_AI -eq "1") { $NoAi = $true }
# Run as "irm ... | iex": there is no local copy yet, so fetch the source first.
if (-not $PSScriptRoot -or -not (Test-Path (Join-Path $PSScriptRoot "Cargo.toml"))) {
  if (-not (Get-Command git -ErrorAction SilentlyContinue)) { Write-Output "Missing: git. Install it with:  winget install -e --id Git.Git"; exit 1 }
  $src = Join-Path $env:LOCALAPPDATA "bolo-src"
  if (Test-Path (Join-Path $src ".git")) { git -C $src pull --ff-only -q } else { git clone --depth 1 -q https://github.com/rave10x/bolo.git $src }
  if ($NoAi) { & (Join-Path $src "install.ps1") -NoAi } else { & (Join-Path $src "install.ps1") }
  return
}
Set-Location $PSScriptRoot

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  Write-Output "Missing: Rust. Install it from https://rustup.rs (it also asks for the Visual Studio C++ Build Tools)."
  exit 1
}
Write-Output "== Building Bolo"
cargo build --release
if ($LASTEXITCODE -ne 0) { exit 1 }
$target = (cargo metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
$exe = Join-Path $target "release\bolo.exe"

Write-Output "== Installing app"
$dest = Join-Path $env:LOCALAPPDATA "Programs\Bolo"
New-Item -ItemType Directory -Force -Path $dest | Out-Null
Copy-Item $exe (Join-Path $dest "bolo.exe") -Force
$menu = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
$lnk = (New-Object -ComObject WScript.Shell).CreateShortcut((Join-Path $menu "Bolo.lnk"))
$lnk.TargetPath = Join-Path $dest "bolo.exe"
$lnk.WorkingDirectory = $dest
$lnk.Description = "Offline English speaking practice"
$lnk.Save()

if (-not $NoAi) {
  Write-Output "== Installing AI parts (about 2 GB, once)"
  powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot "scripts\install-ai.ps1") all
  if ($LASTEXITCODE -ne 0) { Write-Output "AI install had problems. Retry later from inside Bolo (Talk -> Install AI)." }
}
Write-Output ""
Write-Output "Done. Start Bolo from the Start Menu (search 'Bolo')."
