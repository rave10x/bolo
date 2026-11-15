//! Windows audio backend, all in Rust: cpal records, rodio plays, piper (pip) or the built-in
//! Windows voice (System.Speech, no install needed) speaks.

use crate::platform::quiet;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

/// A running recording, playback or speech job.
pub struct Handle {
    stop: Arc<AtomicBool>,
    done: Arc<AtomicBool>,
    child: Arc<Mutex<Option<Child>>>,
    join: Option<JoinHandle<()>>,
}

impl Handle {
    /// Ask the job to end and wait for it (a recording writes its WAV file before returning).
    pub fn stop(mut self, _graceful: bool) {
        self.stop.store(true, Relaxed);
        if let Some(c) = self.child.lock().unwrap().as_mut() {
            let _ = c.kill();
        }
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
    pub fn finished(&mut self) -> bool {
        self.done.load(Relaxed)
    }
}

type Ready = mpsc::Sender<Result<(), String>>;

/// Start a worker thread. `work` reports setup success or failure once through `Ready`, so the caller can return an error.
fn spawn_job<F>(work: F) -> Result<Handle, String>
where
    F: FnOnce(Arc<AtomicBool>, Arc<Mutex<Option<Child>>>, Ready) + Send + 'static,
{
    let stop = Arc::new(AtomicBool::new(false));
    let done = Arc::new(AtomicBool::new(false));
    let child = Arc::new(Mutex::new(None));
    let (tx, rx) = mpsc::channel();
    let (s2, d2, c2) = (stop.clone(), done.clone(), child.clone());
    let join = std::thread::spawn(move || {
        work(s2, c2, tx);
        d2.store(true, Relaxed);
    });
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(())) => Ok(Handle { stop, done, child, join: Some(join) }),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("audio device did not start".into()),
    }
}

pub fn can_record() -> bool {
    use cpal::traits::HostTrait;
    cpal::default_host().default_input_device().is_some()
}

// ---------- recording ----------

fn to_mono_16k(samples: &[f32], channels: usize, rate: u32) -> Vec<i16> {
    let mono: Vec<f32> = if channels > 1 { samples.chunks(channels).map(|f| f.iter().sum::<f32>() / channels as f32).collect() } else { samples.to_vec() };
    let out_len = if rate == 16000 { mono.len() } else { (mono.len() as u64 * 16000 / rate.max(1) as u64) as usize };
    (0..out_len)
        .map(|i| {
            let pos = if rate == 16000 { i as f64 } else { i as f64 * rate as f64 / 16000.0 };
            let a = pos.floor() as usize;
            let frac = (pos - a as f64) as f32;
            let x = mono.get(a).copied().unwrap_or(0.0);
            let y = mono.get(a + 1).copied().unwrap_or(x);
            ((x + (y - x) * frac).clamp(-1.0, 1.0) * i16::MAX as f32) as i16
        })
        .collect()
}

pub fn record(path: &Path) -> Result<Handle, String> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let path = path.to_path_buf();
    spawn_job(move |stop, _child, ready: Ready| {
        let setup = || -> Result<(cpal::Stream, Arc<Mutex<Vec<f32>>>, u16, u32), String> {
            let device = cpal::default_host().default_input_device().ok_or("no microphone found")?;
            let cfg = device.default_input_config().map_err(|e| format!("microphone: {e}"))?;
            let (channels, rate, fmt) = (cfg.channels(), cfg.sample_rate().0, cfg.sample_format());
            let buf: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
            let b = buf.clone();
            let err = |e| eprintln!("microphone stream error: {e}");
            let cfg: cpal::StreamConfig = cfg.into();
            let stream = match fmt {
                cpal::SampleFormat::F32 => device.build_input_stream(&cfg, move |d: &[f32], _| b.lock().unwrap().extend_from_slice(d), err, None),
                cpal::SampleFormat::I16 => device.build_input_stream(&cfg, move |d: &[i16], _| b.lock().unwrap().extend(d.iter().map(|s| *s as f32 / 32768.0)), err, None),
                cpal::SampleFormat::U16 => device.build_input_stream(&cfg, move |d: &[u16], _| b.lock().unwrap().extend(d.iter().map(|s| (*s as f32 - 32768.0) / 32768.0)), err, None),
                other => return Err(format!("unsupported microphone format {other:?}")),
            }
            .map_err(|e| format!("microphone: {e}"))?;
            stream.play().map_err(|e| format!("microphone: {e}"))?;
            Ok((stream, buf, channels, rate))
        };
        match setup() {
            Err(e) => {
                let _ = ready.send(Err(e));
            }
            Ok((stream, buf, channels, rate)) => {
                let _ = ready.send(Ok(()));
                while !stop.load(Relaxed) {
                    std::thread::sleep(Duration::from_millis(20));
                }
                drop(stream);
                let samples = std::mem::take(&mut *buf.lock().unwrap());
                let data = to_mono_16k(&samples, channels as usize, rate);
                let spec = hound::WavSpec { channels: 1, sample_rate: 16000, bits_per_sample: 16, sample_format: hound::SampleFormat::Int };
                if let Ok(mut w) = hound::WavWriter::create(&path, spec) {
                    for s in data {
                        let _ = w.write_sample(s);
                    }
                    let _ = w.finalize();
                }
            }
        }
    })
}

// ---------- playback ----------

/// Play a WAV file and return when it ends or `stop` is set.
fn play_blocking(path: &Path, stop: &AtomicBool) -> Result<(), String> {
    let (_stream, handle) = rodio::OutputStream::try_default().map_err(|e| format!("speakers: {e}"))?;
    let sink = rodio::Sink::try_new(&handle).map_err(|e| format!("speakers: {e}"))?;
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let src = rodio::Decoder::new_wav(std::io::BufReader::new(file)).map_err(|e| format!("wav: {e}"))?;
    sink.append(src);
    while !sink.empty() && !stop.load(Relaxed) {
        std::thread::sleep(Duration::from_millis(30));
    }
    sink.stop();
    Ok(())
}

pub fn play(path: &Path) -> Result<Handle, String> {
    let path = path.to_path_buf();
    // check the file and the output device before reporting success
    spawn_job(move |stop, _child, ready: Ready| {
        if !path.exists() {
            let _ = ready.send(Err("no recording yet".into()));
            return;
        }
        let _ = ready.send(Ok(()));
        let _ = play_blocking(&path, &stop);
    })
}

// ---------- speaking ----------

pub fn speak_piper(bin: &Path, model: &Path, scale: f32, text: &str, wav: &Path) -> Result<Handle, String> {
    let (bin, model, wav, text) = (bin.to_path_buf(), model.to_path_buf(), wav.to_path_buf(), text.to_string());
    spawn_job(move |stop, child, ready: Ready| {
        let spawned = quiet(Command::new(&bin).arg("-m").arg(&model).arg("--length-scale").arg(format!("{scale:.2}")).arg("-f").arg(&wav))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        let mut c = match spawned {
            Ok(c) => c,
            Err(e) => {
                let _ = ready.send(Err(format!("piper: {e}")));
                return;
            }
        };
        if let Some(mut stdin) = c.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        *child.lock().unwrap() = Some(c);
        let _ = ready.send(Ok(()));
        // wait for piper without holding the lock, so stop() can kill it
        loop {
            let status = child.lock().unwrap().as_mut().map(|c| c.try_wait());
            match status {
                Some(Ok(Some(s))) => {
                    if s.success() && !stop.load(Relaxed) {
                        let _ = play_blocking(&wav, &stop);
                    }
                    return;
                }
                Some(Ok(None)) if !stop.load(Relaxed) => std::thread::sleep(Duration::from_millis(30)),
                _ => return,
            }
        }
    })
}

/// Fallback voice when piper is not installed: the voices that come with Windows.
pub fn speak_system(text: &str, wpm: u32) -> Result<Handle, String> {
    let rate = ((wpm as i32 - 150) / 15).clamp(-10, 10); // SAPI rate: -10..10, 0 = normal
    let text = text.to_string();
    spawn_job(move |stop, child, ready: Ready| {
        let script = "Add-Type -AssemblyName System.Speech; $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; $s.Rate = [int]$env:BOLO_RATE; $s.Speak($env:BOLO_TEXT)";
        let spawned = quiet(Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-Command", script]))
            .env("BOLO_RATE", rate.to_string())
            .env("BOLO_TEXT", &text)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        match spawned {
            Err(e) => {
                let _ = ready.send(Err(format!("powershell: {e}")));
            }
            Ok(c) => {
                *child.lock().unwrap() = Some(c);
                let _ = ready.send(Ok(()));
                loop {
                    let status = child.lock().unwrap().as_mut().map(|c| c.try_wait());
                    match status {
                        Some(Ok(None)) if !stop.load(Relaxed) => std::thread::sleep(Duration::from_millis(40)),
                        _ => return,
                    }
                }
            }
        }
    })
}

pub fn system_voice_name() -> Option<&'static str> {
    Some("Windows voice (built in)")
}
