#!/usr/bin/env bash
# Optional: installs the natural English voice (Piper) for Bolo. Needs internet once (~70 MB). No sudo.
set -euo pipefail
D="${XDG_DATA_HOME:-$HOME/.local/share}/bolo"
mkdir -p "$D/voice"
[ -x "$D/piper-venv/bin/piper" ] || { python3 -m venv "$D/piper-venv" && "$D/piper-venv/bin/pip" install -q piper-tts; }
hf=https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US
# lessac (female) speaks for you / the app; ryan (male) is the other person in dialogues
for v in lessac ryan; do
  for f in en_US-$v-medium.onnx en_US-$v-medium.onnx.json; do
    [ -s "$D/voice/$f" ] || curl -fL -o "$D/voice/$f" "$hf/$v/medium/$f"
  done
done
echo "Voice installed. Restart Bolo."
