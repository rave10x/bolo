#!/usr/bin/env bash
# One-step installer for Bolo: builds the app, installs it, and installs the AI parts (voice, speech recognition, chat model).
# Usage: ./install.sh            full install
#        ./install.sh --no-ai    app only (skip the ~2 GB AI download)
set -euo pipefail
cd "$(dirname "$0")"
AI=1; [ "${1:-}" = "--no-ai" ] && AI=0

MISSING=0
need() { command -v "$1" >/dev/null || { echo "Missing: $1 — $2"; MISSING=1; }; }
need cargo   "install Rust: https://rustup.rs"
need python3 "install Python 3 (sudo dnf install python3 / sudo apt install python3 python3-venv)"
need curl    "install curl"
need pw-record "install PipeWire tools (sudo dnf install pipewire-utils / sudo apt install pipewire-bin)"
if [ "$MISSING" = 1 ]; then echo "Install the missing tools above, then run ./install.sh again."; exit 1; fi

echo "== Building Bolo"
python3 -I tools/build.py
cargo build --release
BIN="$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys;print(json.load(sys.stdin)["target_directory"])')/release/bolo"

echo "== Installing app"
mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications" "$HOME/.local/share/icons/hicolor/scalable/apps"
install -m 755 "$BIN" "$HOME/.local/bin/bolo"
cp public/icon.svg "$HOME/.local/share/icons/hicolor/scalable/apps/bolo.svg"
cp packaging/bolo.desktop "$HOME/.local/share/applications/bolo.desktop"
if command -v update-desktop-database >/dev/null; then update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true; fi

if [ "$AI" = 1 ]; then
  echo "== Installing AI parts (about 2 GB, once)"
  bash scripts/install-ai.sh all || echo "AI install had problems. Retry later from inside Bolo (Talk -> Install AI) or run: bash scripts/install-ai.sh"
fi
echo
echo "Done. Start Bolo from your app menu or run: bolo   (make sure ~/.local/bin is on your PATH)"
