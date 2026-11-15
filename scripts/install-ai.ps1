# Installs the AI parts of Bolo on Windows, all offline-capable afterwards:
#   voice : Piper natural voices (~70 MB)
#   talk  : Whisper speech recognition (faster-whisper base.en) + the chat model in Ollama (~2 GB)
# Usage: install-ai.ps1 [all|voice|talk]     (default: all)
# BOLO_NONINTERACTIVE=1 (set by the app) means: never wait for input.
param([string]$What = "all")
$ErrorActionPreference = "Continue"
$ProgressPreference = "SilentlyContinue"
$D = Join-Path $env:LOCALAPPDATA "bolo"
$Model = if ($env:BOLO_MODEL) { $env:BOLO_MODEL } else { "qwen2.5:3b-instruct" }
New-Item -ItemType Directory -Force -Path (Join-Path $D "voice") | Out-Null

function Say($m) { Write-Output ""; Write-Output "== $m" }

# find a working Python 3 (the "py" launcher first: it avoids the Microsoft Store stub)
$Py = $null
foreach ($c in @(@("py", "-3"), @("python"), @("python3"))) {
  try {
    & $c[0] @($c[1..($c.Length - 1)] + @("-c", "import venv, ensurepip")) 2>$null
    if ($LASTEXITCODE -eq 0) { $Py = $c; break }
  } catch {}
}
if (-not $Py) {
  Write-Output "PROBLEM: Python 3 was not found (or its venv module is missing)."
  Write-Output "Install it from https://www.python.org/downloads/ (tick 'Add python.exe to PATH'), or run:  winget install -e --id Python.Python.3.12"
  Write-Output "Then press 'Install AI' again."
  exit 3
}
function RunPy { & $Py[0] @($Py[1..($Py.Length - 1)] + $args) }

function Install-Voice {
  Say "Voice (Piper)"
  $venv = Join-Path $D "piper-venv"
  $piper = Join-Path $venv "Scripts\piper.exe"
  if (-not (Test-Path $piper)) {
    RunPy -m venv $venv
    if ($LASTEXITCODE -ne 0) { Write-Output "Could not create the Python environment."; return 1 }
    & (Join-Path $venv "Scripts\pip.exe") install -q piper-tts
    if ($LASTEXITCODE -ne 0) { Write-Output "Could not install piper."; return 1 }
  }
  $hf = "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US"
  # lessac speaks for you / the app; ryan is the other person in dialogues
  foreach ($v in @("lessac", "ryan")) {
    foreach ($f in @("en_US-$v-medium.onnx", "en_US-$v-medium.onnx.json")) {
      $out = Join-Path $D "voice\$f"
      if (-not (Test-Path $out) -or (Get-Item $out).Length -eq 0) {
        try { Invoke-WebRequest -Uri "$hf/$v/medium/$f" -OutFile $out -UseBasicParsing } catch { Write-Output "Download failed: $f"; return 1 }
      }
    }
  }
  Write-Output "Voice installed."
  return 0
}

function Find-Ollama {
  $c = Get-Command ollama -ErrorAction SilentlyContinue
  if ($c) { return $c.Source }
  $p = Join-Path $env:LOCALAPPDATA "Programs\Ollama\ollama.exe"
  if (Test-Path $p) { return $p }
  return $null
}

function Install-Talk {
  Say "Speech recognition (Whisper)"
  $venv = Join-Path $D "stt-venv"
  $python = Join-Path $venv "Scripts\python.exe"
  if (-not (Test-Path $python)) {
    RunPy -m venv $venv
    if ($LASTEXITCODE -ne 0) { Write-Output "Could not create the Python environment."; return 1 }
  }
  & (Join-Path $venv "Scripts\pip.exe") install -q faster-whisper
  if ($LASTEXITCODE -ne 0) { return 1 }
  & $python -c "from faster_whisper import WhisperModel; WhisperModel('base.en', device='cpu', compute_type='int8')"
  if ($LASTEXITCODE -ne 0) { return 1 }
  Write-Output "Speech recognition installed."

  Say "Chat model (Ollama)"
  $ollama = Find-Ollama
  if (-not $ollama) {
    if (Get-Command winget -ErrorAction SilentlyContinue) {
      Write-Output "Installing Ollama with winget..."
      winget install -e --id Ollama.Ollama --silent --accept-package-agreements --accept-source-agreements
      $ollama = Find-Ollama
    }
  }
  if (-not $ollama) {
    Write-Output "NEED_OLLAMA: Ollama is not installed. Download and run the installer from https://ollama.com/download/windows"
    Write-Output "Then press 'Install AI' again."
    return 2
  }
  $up = $false
  try { Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing -TimeoutSec 2 | Out-Null; $up = $true } catch {}
  if (-not $up) {
    Start-Process -FilePath $ollama -ArgumentList "serve" -WindowStyle Hidden
    for ($i = 0; $i -lt 10 -and -not $up; $i++) {
      Start-Sleep -Seconds 1
      try { Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing -TimeoutSec 2 | Out-Null; $up = $true } catch {}
    }
  }
  & $ollama pull $Model
  if ($LASTEXITCODE -ne 0) { return 1 }
  Write-Output "Chat model ready: $Model"
  return 0
}

$rc = 0
switch ($What) {
  "voice" { $rc = Install-Voice | Select-Object -Last 1 }
  "talk"  { $rc = Install-Talk | Select-Object -Last 1 }
  "all"   {
    $rc = Install-Talk | Select-Object -Last 1
    $v = Install-Voice | Select-Object -Last 1
    if ($rc -eq 0 -and $v -ne 0) { $rc = 1 }
  }
  default { Write-Output "usage: install-ai.ps1 [all|voice|talk]"; exit 64 }
}
if ($rc -eq 0) { Write-Output "DONE: everything installed. Restart Bolo." } else { Write-Output "FINISHED WITH PROBLEMS (code $rc). See the messages above." }
exit $rc
