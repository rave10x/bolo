#!/usr/bin/env bash
# Installs what the Baat-cheet (Talk) screen needs: offline speech recognition + a small chat model. No sudo. ~2.5 GB once.
set -euo pipefail
D="${XDG_DATA_HOME:-$HOME/.local/share}/bolo"
mkdir -p "$D"
[ -x "$D/stt-venv/bin/python" ] || python3 -m venv "$D/stt-venv"
"$D/stt-venv/bin/pip" install -q faster-whisper
# download the speech model once now (so the first conversation is not slow)
"$D/stt-venv/bin/python" -c "from faster_whisper import WhisperModel; WhisperModel('base.en', device='cpu', compute_type='int8')"
command -v ollama >/dev/null || { echo "ollama is not installed: https://ollama.com/download"; exit 1; }
ollama pull qwen2.5:3b-instruct
echo "Ready. Open Bolo -> Baat-cheet."
