# Bolo — decisions and why

Written 2026-11-14 while the owner slept; nothing here was reviewed by them. Items marked **CHECK** are the ones most worth a second look.

## What the user needs (as stated)
1. First message: English is weak, government school, no English until high school → I built a beginner app.
2. Correction: **"I can understand English, my problem is speaking, and there is no one to talk with."**
3. Correction: **"It is absolutely basic. Add conversation like office, interview, HR."**

So the product is a **speaking trainer for a working/job-seeking developer**, not a beginner course. Everything below follows from that.

## Product decisions
| Decision | Why |
|---|---|
| Lead with **Partner · Talk** (AI conversation) and **Conversations** (scripted dialogues), not lessons | The stated problem is speaking with nobody to practise with. A partner removes that blocker. |
| Keep the 30 beginner lessons, but put **Professional speaking** (6 lessons) **above** them and say "skip what you know" | Deleting content is irreversible and the beginner pages cost nothing; ordering fixes the "too basic" complaint. |
| 32 dialogues in 5 categories (Interview 9, Office 11, HR & Career 6, Client & Calls 4, Team & Social 3) | Matches the three areas named (office, interview, HR) plus client calls, which a developer meets next. Each has 8 lines and 3 key phrases. |
| Dialogue **role-play**: partner line is spoken, you speak yours, app scores it | Speaking aloud with feedback is the core exercise; scripted lines give a safe model answer to say. |
| 16 Talk scenarios, professional ones first (HR round, technical interview, project walk-through, behavioural/STAR, salary negotiation, stand-up, 1:1, client call, apology, demo, resignation, first day), casual ones last | Same reason; casual chat kept because small talk matters too. |
| Partner prompt asks for **upper-intermediate, natural workplace English** | The user said they understand English; baby-English replies would not stretch them. |
| Five new speaking packs (meetings, polite disagreement, interview answers, HR talk, presenting): 60 longer sentences | The old packs were short beginner sentences. |
| UI labels bilingual ("Aaj · Today"); explanations stay Hinglish; **tips from the AI are English** | The user reads English; Hinglish tips from a 3B model came out as garbled Devanagari (see below). |
| Progress is one JSON file, `#[serde(default)]` on every struct | Adding fields must never wipe a learner's history. Covered by a test. |

## Technical decisions
- **Rust + Dioxus desktop, content embedded with rust-embed.** Same stack as PrepDeck: toolchain already installed, deps cached (the build shares PrepDeck's `target/`, so a release build takes ~35 s), user already uses that app.
- **Content as plain text** (`tools/*.txt`, `lessons_*.py`) → `python3 -I tools/build.py` → JSON. Validator rejects an example sentence that does not contain its word as a whole word, duplicate ids, quizzes with a bad answer index, dialogues without both speakers. Easy to extend without touching Rust.
- **Everything runs offline on the laptop.** Reasons: the user has no one to talk to and probably wants to practise anytime; privacy (their voice never leaves the machine); no API key, cost or quota to manage.
  - **Speech recognition: faster-whisper `base.en`**, int8 on CPU. Measured: **2.3 s** for a short sentence, transcript correct. `small.en` would be more accurate for accented speech but ~3x slower; **CHECK** if base.en mishears the owner a lot — change `STT_MODEL` in `src/llm.rs`.
  - **Conversation model: `qwen2.5:3b-instruct` via the ollama that was already installed.** The two installed models were coding models (7B), slow and wrong for chat. 3B gives a reply in ~3–4 s on this i3 (first reply ~13 s because of model load → I warm it up when the Talk screen opens). A 7B instruct model would be smarter but ~2x slower; **CHECK** if replies feel shallow.
  - **Voice: Piper** (`lessac` for the app/you, `ryan` for the other person in dialogues), falling back to espeak-ng. espeak is robotic, which is bad for a learner copying pronunciation.
- **Split the AI into two calls (reply, then correction).** One prompt that both replied and corrected: the 3B model skipped real errors ("am having two years experience… since six month" came back "fine"). A dedicated corrector with 4 worked examples fixed that. The reply appears first and the correction card follows a few seconds later, so the conversation is never blocked.
- **No AI-written explanations.** The corrector's tips were wrong too often ("use 'fix' instead of 'fixed'", "capital I when part of a name"). A wrong explanation is worse than none for a learner. Instead the correction shows the better sentence with the **changed words highlighted** (a diff against what was said), which is always accurate. The UI says "AI kabhi-kabhi galat bhi ho sakta hai".
- **Contractions normalised before comparing** ("It's" = "It is", "log-in" = "login"), so formatting differences are not reported as mistakes or lower the pronunciation score.
- **Whisper reads WAV files directly (numpy), not through ffmpeg/`av`.** `av` 19 crashes on this Python (3.14) with `metadata_errors`. Recordings are plain 16 kHz WAV, so the bypass loses nothing.
- **Pronunciation "check" = what Whisper heard vs the target sentence (% of words matched, missing words listed).** It is a proxy: it finds dropped/replaced words, not subtle accent.
- **Chat history sent to the model is the last 10 turns** to keep latency flat on CPU.
- **Corrections you receive become your own drill** ("Meri galtiyan"): sentences you said wrong turn into pick-the-right-one questions. Capped at 200.

## Verified vs not verified
Verified (by running it): all content builds and validates; 10 Rust tests pass (word → every question type incl. a blank in every example, drill generation, old-save compatibility, streak, SRS box moves, dialogues well-formed, contraction/score matching, speech text cleaning); app builds in release; Today, Lessons, Conversations list, dialogue page, role-play first step, Talk with a typed sentence (reply + highlighted correction) all render; Whisper transcribes a Piper-generated sentence correctly in 2.3 s; ollama replies and corrects; Piper voices (both) synthesise; microphone records a non-empty file.

**Not verified — CHECK these first:**
1. **A real spoken round trip.** I cannot speak into the mic. Pieces were tested separately (mic records, Whisper transcribes audio, text reaches the model), not as one live sentence. Accented speech may need `small.en`.
2. **Whisper tends to "fix" grammar when transcribing**, so some spoken mistakes will be silently corrected before the corrector ever sees them. Typing a sentence is the reliable way to test correction.
3. **Small-model quality**: replies can be generic/slightly off; corrections miss some errors (e.g. "I have fixed … yesterday" was not flagged in my test) and occasionally over-correct. This is the main limitation of running 3B on a laptop CPU.
4. Hindi translations, Devanagari pronunciations and the 32 dialogues were written by me and not reviewed by a fluent speaker. Dialogue salary figures are placeholders ("between X and Y").
5. The dialogue's "other person" voice and the role-play scoring beyond the first step were checked visually, not by running a full role-play.
6. A fully fresh machine: the install scripts (`scripts/install-voice.sh`, `scripts/install-talk.sh`) were written from what I ran by hand, not run from scratch.

## Not done / next
- Android port (PrepDeck has one; Bolo does not).
- Streaming replies (text appears word by word) and faster models.
- A weekly "speaking report" (words/min, filler words, most repeated mistakes).
- Sentence-level pronunciation feedback (phoneme alignment) — needs a heavier model.
- Offer `small.en` / `qwen2.5:7b-instruct` as a Settings choice.

## Where things are
- Code `~/Workspace/Projects/bolo` (installed `~/.local/bin/bolo`); progress `~/.local/share/bolo/state.json`.
- Models: `~/.local/share/bolo/{voice,piper-venv,stt-venv}`, ollama `qwen2.5:3b-instruct`, Whisper cache in `~/.cache/huggingface`.
- Dev env vars: `BOLO_ROUTE` (talk|dialogues|dialogue:<id>|lessons|words|speak|practice|…), `BOLO_SCENARIO`, `BOLO_SAY`, `BOLO_ROLEPLAY`, `BOLO_THEME`.
