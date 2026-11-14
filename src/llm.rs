//! Local conversation partner: speech-to-text (faster-whisper) and a small chat model through ollama.
//! Everything runs on this laptop; nothing is sent anywhere.

use crate::data::State;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub const MODEL: &str = "qwen2.5:3b-instruct";
pub const STT_MODEL: &str = "base.en";

fn stt_python() -> std::path::PathBuf {
    State::dir().join("stt-venv/bin/python")
}

pub fn stt_ready() -> bool {
    stt_python().exists()
}

/// Is ollama reachable and does it have the chat model?
pub fn chat_ready() -> bool {
    let out = Command::new("curl").args(["-s", "-m", "3", "http://localhost:11434/api/tags"]).output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).contains("qwen2.5:3b-instruct"),
        Err(_) => false,
    }
}

/// Speech file -> English text. Blocking: call from a worker thread.
pub fn transcribe(wav: &Path) -> Result<String, String> {
    let script = State::dir().join("transcribe.py");
    std::fs::write(&script, include_str!("../assets/transcribe.py")).map_err(|e| e.to_string())?;
    let out = Command::new(stt_python()).arg(&script).arg(wav).arg(STT_MODEL).output().map_err(|e| format!("speech tool: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("speech recognition failed: {}", err.lines().last().unwrap_or("")));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn ollama(body: serde_json::Value) -> Result<String, String> {
    let mut child = Command::new("curl")
        .args(["-s", "-m", "120", "http://localhost:11434/api/chat", "-d", "@-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("curl: {e}"))?;
    child.stdin.take().ok_or("no stdin")?.write_all(body.to_string().as_bytes()).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|_| "ollama did not answer (is it running?)".to_string())?;
    v["message"]["content"].as_str().map(|s| s.to_string()).ok_or_else(|| "empty answer".to_string())
}

const RULES: &str = "You are a friendly English speaking partner for a learner from India who is preparing for a software job. The learner understands English well and reads it fine, but finds speaking hard and has nobody to practise with. \
Speak natural, professional workplace English at an upper-intermediate level, like a real colleague or interviewer would; do not talk down to them. \
Keep your reply to ONE or TWO sentences and end with one question so the conversation continues. Be warm and encouraging. \
The learner's message comes from speech recognition: ignore capital letters and punctuation. Never mention mistakes in your reply. \
Reply ONLY with JSON: {\"reply\": \"your reply\"}";

const FIXER: &str = "You are an English teacher for an Indian learner. The learner's sentence comes from speech recognition, so ignore capitalisation and punctuation. \
Rewrite it as correct, natural spoken English with the SAME meaning, keeping the learner's own words where they are fine. \
Typical mistakes to fix: 'am having/am knowing' for state verbs, 'since' with a duration (should be 'for'), missing articles (a/an/the), singular/plural errors, wrong tense, 'discuss about', 'explain me', 'revert back', 'I am agree'. \
Do not change things that are already acceptable, and do not change style or contractions. If the sentence is acceptable, return it unchanged. \
Examples:\nInput: i am having a doubt about this topic\n{\"better\": \"I have a doubt about this topic.\"}\n\
Input: we discussed about the plan yesterday\n{\"better\": \"We discussed the plan yesterday.\"}\n\
Input: I live in Delhi for five years\n{\"better\": \"I have lived in Delhi for five years.\"}\n\
Input: Yesterday I went to the office and met my manager\n{\"better\": \"Yesterday I went to the office and met my manager.\"}\n\
Reply ONLY with JSON {\"better\": \"...\"}";

/// `history` = (is_user, text) pairs, oldest first. The newest entry must be the learner's message.
pub fn partner_reply(scenario: &str, history: &[(bool, String)]) -> Result<String, String> {
    let mut msgs = vec![serde_json::json!({"role": "system", "content": format!("{RULES}\nScenario: {scenario}")})];
    let start = history.len().saturating_sub(10);
    for (user, text) in &history[start..] {
        msgs.push(serde_json::json!({"role": if *user { "user" } else { "assistant" }, "content": text}));
    }
    let raw = ollama(serde_json::json!({
        "model": MODEL, "messages": msgs, "stream": false, "format": "json",
        "options": {"temperature": 0.6, "num_predict": 160}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| format!("could not read the answer: {raw}"))?;
    let reply = v["reply"].as_str().unwrap_or("").trim().to_string();
    if reply.is_empty() { Err("empty reply".into()) } else { Ok(reply) }
}

/// Correct one learner sentence. The result equals the input (apart from capitals) when it was fine.
pub fn correct(said: &str) -> Result<String, String> {
    let raw = ollama(serde_json::json!({
        "model": MODEL, "stream": false, "format": "json",
        "messages": [{"role": "system", "content": FIXER}, {"role": "user", "content": said}],
        "options": {"temperature": 0, "num_predict": 100}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad correction".to_string())?;
    Ok(v["better"].as_str().unwrap_or("").trim().to_string())
}

/// Two short example answers the learner could say next.
pub fn suggestions(scenario: &str, last_question: &str) -> Result<Vec<String>, String> {
    let raw = ollama(serde_json::json!({
        "model": MODEL, "stream": false, "format": "json",
        "messages": [
            {"role": "system", "content": "You help an English learner who is stuck. Give two different, short (6-12 words), simple, natural example answers in first person. Reply ONLY with JSON {\"a\": \"...\", \"b\": \"...\"}."},
            {"role": "user", "content": format!("Scenario: {scenario}\nThe partner just said: {last_question}")}
        ],
        "options": {"temperature": 0.8, "num_predict": 80}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad answer".to_string())?;
    Ok(["a", "b"].iter().filter_map(|k| v[k].as_str().map(|s| s.to_string())).filter(|s| !s.is_empty()).collect())
}

/// Load the chat model into memory in the background so the first reply is not slow.
pub fn warm_up() {
    let _ = ollama(serde_json::json!({
        "model": MODEL, "stream": false,
        "messages": [{"role": "user", "content": "hi"}],
        "options": {"num_predict": 1}, "keep_alive": "30m"
    }));
}
