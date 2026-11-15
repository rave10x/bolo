//! Linux audio backend: PipeWire tools (pw-record / pw-play), piper, espeak-ng.

use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct Handle(Child);

impl Handle {
    /// `graceful` lets a recorder finish writing its WAV header.
    pub fn stop(mut self, graceful: bool) {
        if graceful {
            // SIGINT lets pw-record finish the WAV header.
            let _ = Command::new("kill").args(["-INT", &self.0.id().to_string()]).status();
        } else {
            let _ = self.0.kill();
        }
        let _ = self.0.wait();
    }
    pub fn finished(&mut self) -> bool {
        matches!(self.0.try_wait(), Ok(Some(_)))
    }
}

pub fn can_record() -> bool {
    crate::platform::which("pw-record").is_some() && crate::platform::which("pw-play").is_some()
}

pub fn record(path: &Path) -> Result<Handle, String> {
    let child = Command::new("pw-record")
        .args(["--rate", "16000", "--channels", "1"])
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("pw-record: {e}"))?;
    Ok(Handle(child))
}

pub fn play(path: &Path) -> Result<Handle, String> {
    let child = Command::new("pw-play").arg(path).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(|e| format!("pw-play: {e}"))?;
    Ok(Handle(child))
}

/// piper writes a wav file (length-scale > 1 is slower), then pw-play plays it.
pub fn speak_piper(bin: &Path, model: &Path, scale: f32, text: &str, wav: &Path) -> Result<Handle, String> {
    let script = "printf '%s' \"$4\" | \"$1\" -m \"$2\" --length-scale \"$3\" -f \"$5\" >/dev/null 2>&1 && exec pw-play \"$5\"";
    let child = Command::new("sh")
        .args(["-c", script, "sh"])
        .arg(bin)
        .arg(model)
        .arg(format!("{scale:.2}"))
        .arg(text)
        .arg(wav)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("piper: {e}"))?;
    Ok(Handle(child))
}

/// Fallback voice when piper is not installed.
pub fn speak_system(text: &str, wpm: u32) -> Result<Handle, String> {
    let child = Command::new("espeak-ng")
        .args(["-v", "en-us", "-s", &wpm.to_string()])
        .arg(text)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("espeak-ng: {e}"))?;
    Ok(Handle(child))
}

pub fn system_voice_name() -> Option<&'static str> {
    crate::platform::which("espeak-ng").map(|_| "espeak-ng (robot voice)")
}
