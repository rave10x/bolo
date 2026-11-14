#!/usr/bin/env bash
# Installs the AI parts of Bolo, all offline-capable afterwards:
#   voice : Piper natural voices (~70 MB)
#   talk  : Whisper speech recognition (faster-whisper base.en) + the chat model in Ollama (~2 GB)
# Usage: install-ai.sh [all|voice|talk]     (default: all)
# Set BOLO_NONINTERACTIVE=1 when run from the app: it never asks for a password.
set -uo pipefail
WHAT="${1:-all}"
D="${XDG_DATA_HOME:-$HOME/.local/share}/bolo"
MODEL="${BOLO_MODEL:-qwen2.5:3b-instruct}"
mkdir -p "$D/voice"
say() { printf '\n== %s\n' "$*"; }

install_voice() {
  say "Voice (Piper)"
  [ -x "$D/piper-venv/bin/piper" ] || { python3 -m venv "$D/piper-venv" && "$D/piper-venv/bin/pip" install -q piper-tts; } || { echo "Could not install piper."; return 1; }
  hf=https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US
  # lessac speaks for you / the app; ryan is the other person in dialogues
  for v in lessac ryan; do
    for f in en_US-$v-medium.onnx en_US-$v-medium.onnx.json; do
      [ -s "$D/voice/$f" ] || curl -fL -o "$D/voice/$f" "$hf/$v/medium/$f" || return 1
    done
  done
  echo "Voice installed."
}

install_talk() {
  say "Speech recognition (Whisper)"
  [ -x "$D/stt-venv/bin/python" ] || python3 -m venv "$D/stt-venv" || return 1
  "$D/stt-venv/bin/pip" install -q faster-whisper || return 1
  "$D/stt-venv/bin/python" -c "from faster_whisper import WhisperModel; WhisperModel('base.en', device='cpu', compute_type='int8')" || return 1
  echo "Speech recognition installed."

  say "Chat model (Ollama)"
  if ! command -v ollama >/dev/null; then
    if [ "${BOLO_NONINTERACTIVE:-0}" = "1" ]; then
      echo "NEED_OLLAMA: Ollama is not installed. Open a terminal and run:  curl -fsSL https://ollama.com/install.sh | sh"
      echo "Then press 'Install AI' again."
      return 2
    fi
    echo "Installing Ollama (asks for your password)..."
    curl -fsSL https://ollama.com/install.sh | sh || return 1
  fi
  if ! curl -s -m 2 http://localhost:11434/api/tags >/dev/null; then
    nohup ollama serve >/dev/null 2>&1 &
    for _ in 1 2 3 4 5 6 7 8; do curl -s -m 2 http://localhost:11434/api/tags >/dev/null && break; sleep 1; done
  fi
  ollama pull "$MODEL" 2>&1 | tr '\r' '\n' | sed -u 's/\x1b\[[0-9;?]*[A-Za-z]//g' | awk 'NF { print; fflush() }'
  [ "${PIPESTATUS[0]}" = "0" ] || return 1
  echo "Chat model ready: $MODEL"
}

rc=0
case "$WHAT" in
  voice) install_voice || rc=$? ;;
  talk)  install_talk  || rc=$? ;;
  all)   install_talk  || rc=$?; install_voice || { [ $rc -eq 0 ] && rc=1; } ;;
  *) echo "usage: $0 [all|voice|talk]"; exit 64 ;;
esac
if [ $rc -eq 0 ]; then echo "DONE: everything installed. Restart Bolo."; else echo "FINISHED WITH PROBLEMS (code $rc). See the messages above."; fi
exit $rc
