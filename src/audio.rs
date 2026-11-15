//! Speech and recording (all offline). The OS-specific part lives in a backend:
//! Linux = pw-record / pw-play + espeak-ng, Windows = cpal / rodio + the built-in Windows voice.
//! Piper is used for a natural voice on both when it is installed.

use crate::audio_backend as backend;
use crate::data::State;
use std::path::PathBuf;
use std::sync::Mutex;

static REC: Mutex<Option<backend::Handle>> = Mutex::new(None);
static PLAY: Mutex<Option<backend::Handle>> = Mutex::new(None);
static VOICE: Mutex<Option<backend::Handle>> = Mutex::new(None);

/// Can this machine record and play sound?
pub fn can_record() -> bool {
    backend::can_record()
}

pub fn recordings_dir() -> PathBuf {
    State::dir().join("recordings")
}

pub fn rec_path(id: &str) -> PathBuf {
    let safe: String = id.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    recordings_dir().join(format!("{safe}.wav"))
}

pub fn has_recording(id: &str) -> bool {
    std::fs::metadata(rec_path(id)).is_ok_and(|m| m.len() > 1000)
}

fn stop_slot(slot: &Mutex<Option<backend::Handle>>, graceful: bool) {
    if let Some(h) = slot.lock().unwrap().take() {
        h.stop(graceful);
    }
}

pub fn stop_recording() {
    stop_slot(&REC, true);
}
pub fn stop_all() {
    stop_recording();
    stop_slot(&PLAY, false);
    stop_slot(&VOICE, false);
}

pub fn start_recording(id: &str) -> Result<(), String> {
    stop_all();
    let path = rec_path(id);
    std::fs::create_dir_all(recordings_dir()).map_err(|e| e.to_string())?;
    *REC.lock().unwrap() = Some(backend::record(&path)?);
    Ok(())
}

pub fn play_recording(id: &str) -> Result<(), String> {
    stop_all();
    *PLAY.lock().unwrap() = Some(backend::play(&rec_path(id))?);
    Ok(())
}

fn models() -> Vec<PathBuf> {
    let dir = State::dir().join("voice");
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir).map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "onnx")).collect()).unwrap_or_default();
    v.sort();
    v
}

fn piper_model() -> Option<PathBuf> {
    models().into_iter().find(|p| p.to_string_lossy().contains("lessac")).or_else(|| models().into_iter().next())
}

/// A second voice (male) for the other person in a dialogue, if installed.
fn piper_model_alt() -> Option<PathBuf> {
    models().into_iter().find(|p| p.to_string_lossy().contains("ryan"))
}

/// piper from the app's own venv (installed by the AI installer) or from PATH.
fn piper_bin() -> Option<PathBuf> {
    let own = crate::platform::venv_bin(&State::dir().join("piper-venv"), "piper");
    if own.exists() {
        return Some(own);
    }
    crate::platform::which("piper")
}

/// Which voice is in use, for the Settings page.
pub fn voice_name() -> String {
    if piper_model().is_some() && piper_bin().is_some() {
        "Piper (natural voice)".into()
    } else if let Some(n) = backend::system_voice_name() {
        n.into()
    } else {
        "none".into()
    }
}

/// Speak English text. `wpm` is the speed in words per minute.
pub fn speak(text: &str, wpm: u32) -> Result<(), String> {
    speak_as(text, wpm, false)
}

/// Same, but `alt` uses the second voice when it is installed (for the other person in a dialogue).
pub fn speak_as(text: &str, wpm: u32, alt: bool) -> Result<(), String> {
    stop_all();
    let cleaned = clean_for_speech(text);
    let text = cleaned.as_str();
    let model = if alt { piper_model_alt().or_else(piper_model) } else { piper_model() };
    let handle = if let (Some(model), Some(bin)) = (model, piper_bin()) {
        let scale = (150.0 / wpm.max(60) as f32).clamp(0.8, 1.8); // length-scale > 1 is slower
        let wav = State::dir().join("speak.wav");
        let _ = std::fs::create_dir_all(State::dir());
        backend::speak_piper(&bin, &model, scale, text, &wav)?
    } else {
        backend::speak_system(text, wpm)?
    };
    *VOICE.lock().unwrap() = Some(handle);
    Ok(())
}

/// Speak and wait until the sentence has finished. Returns false if it was stopped meanwhile.
pub fn speak_wait(text: &str, wpm: u32, alt: bool) -> bool {
    if speak_as(text, wpm, alt).is_err() {
        return false;
    }
    loop {
        std::thread::sleep(std::time::Duration::from_millis(80));
        let mut slot = VOICE.lock().unwrap();
        match slot.as_mut() {
            Some(h) => {
                if h.finished() {
                    return true;
                }
            }
            None => return false,
        }
    }
}

/// Remove things that should not be read aloud: "(pause)" notes, arrows, bars.
fn clean_for_speech(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0;
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            '↗' | '↘' | '|' => {}
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn speech_text_is_cleaned() {
        assert_eq!(clean_for_speech("Good morning. (pause) I am Rahul."), "Good morning. I am Rahul.");
        assert_eq!(clean_for_speech("Are you ready? ↗"), "Are you ready?");
    }
}
