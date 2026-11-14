import sys, wave
import numpy as np
from faster_whisper import WhisperModel

def load(path):
    # plain PCM wav -> float32 mono 16 kHz (avoids the av/ffmpeg decoder)
    with wave.open(path, "rb") as w:
        rate, ch, width = w.getframerate(), w.getnchannels(), w.getsampwidth()
        raw = w.readframes(w.getnframes())
    dt = {1: np.uint8, 2: np.int16, 4: np.int32}[width]
    a = np.frombuffer(raw, dtype=dt).astype(np.float32)
    a = (a - 128) / 128 if width == 1 else a / float(np.iinfo(dt).max)
    if ch > 1:
        a = a.reshape(-1, ch).mean(axis=1)
    if rate != 16000 and len(a):
        n = int(len(a) * 16000 / rate)
        a = np.interp(np.linspace(0, len(a) - 1, n), np.arange(len(a)), a).astype(np.float32)
    return a

audio = load(sys.argv[1])
model = WhisperModel(sys.argv[2] if len(sys.argv) > 2 else "base.en", device="cpu", compute_type="int8", cpu_threads=4)
segs, _ = model.transcribe(audio, language="en", beam_size=1, vad_filter=True, condition_on_previous_text=False)
print(" ".join(s.text.strip() for s in segs).strip())
