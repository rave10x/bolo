//! Local conversation partner: speech-to-text (faster-whisper) and a small chat model through ollama.
//! Everything runs on this laptop; nothing is sent anywhere.

use crate::data::State;
use crate::platform::quiet;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

const OLLAMA: &str = "http://localhost:11434";

fn http_get(path: &str, secs: u64) -> Option<String> {
    ureq::get(&format!("{OLLAMA}{path}")).timeout(Duration::from_secs(secs)).call().ok()?.into_string().ok()
}

pub const DEFAULT_MODEL: &str = "qwen2.5:3b-instruct";
static MODEL_NAME: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

pub fn set_model(m: &str) {
    if let Ok(mut g) = MODEL_NAME.lock() {
        *g = m.to_string();
    }
}

pub fn model() -> String {
    match MODEL_NAME.lock() {
        Ok(g) if !g.is_empty() => g.clone(),
        _ => DEFAULT_MODEL.to_string(),
    }
}

/// Models installed in ollama (names), empty if ollama is not running.
pub fn installed_models() -> Vec<String> {
    let Some(body) = http_get("/api/tags", 3) else { return vec![] };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) else { return vec![] };
    v["models"].as_array().map(|a| a.iter().filter_map(|m| m["name"].as_str().map(String::from)).collect()).unwrap_or_default()
}

static INSTALLING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn installing() -> bool {
    INSTALLING.load(std::sync::atomic::Ordering::Relaxed)
}

fn install_log() -> std::path::PathBuf {
    State::dir().join("install.log")
}

/// Last few lines of the installer's output.
pub fn install_tail() -> String {
    let t = std::fs::read_to_string(install_log()).unwrap_or_default();
    let lines: Vec<&str> = t.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(5)..].join("\n")
}

/// Run the bundled installer (voice + speech recognition + chat model) in the background. Output goes to install.log.
pub fn start_install() {
    use std::sync::atomic::Ordering::Relaxed;
    if INSTALLING.swap(true, Relaxed) {
        return;
    }
    std::thread::spawn(|| {
        let dir = State::dir();
        let _ = std::fs::create_dir_all(&dir);
        #[cfg(windows)]
        let (script, text) = (dir.join("install-ai.ps1"), include_str!("../scripts/install-ai.ps1"));
        #[cfg(not(windows))]
        let (script, text) = (dir.join("install-ai.sh"), include_str!("../scripts/install-ai.sh"));
        let _ = std::fs::write(&script, text);
        let log = std::fs::File::create(install_log());
        if let Ok(log) = log {
            let err = log.try_clone();
            if let Ok(err) = err {
                #[cfg(windows)]
                let mut cmd = {
                    let mut c = Command::new("powershell");
                    c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"]).arg(&script).arg("all");
                    c
                };
                #[cfg(not(windows))]
                let mut cmd = {
                    let mut c = Command::new("bash");
                    c.arg(&script).arg("all");
                    c
                };
                let _ = quiet(&mut cmd).env("BOLO_NONINTERACTIVE", "1").env("BOLO_MODEL", model())
                    .stdin(Stdio::null()).stdout(log).stderr(err).status();
            }
        }
        INSTALLING.store(false, Relaxed);
    });
}

const FILLERS: [&str; 6] = ["basically", "actually", "you know", "i mean", "kind of", "sort of"];

/// Count filler words/phrases in a spoken answer (Whisper often drops "um", so this is a lower bound).
pub fn count_fillers(text: &str) -> u32 {
    let t = format!(" {} ", crate::drill::norm(text));
    let mut n = 0;
    for f in FILLERS {
        n += t.matches(&format!(" {f} ")).count() as u32;
    }
    for f in ["um", "umm", "uh", "uhh", "erm", "hmm"] {
        n += t.matches(&format!(" {f} ")).count() as u32;
    }
    n
}
pub const STT_MODEL: &str = "base.en";

fn stt_python() -> std::path::PathBuf {
    crate::platform::venv_bin(&State::dir().join("stt-venv"), "python")
}

pub fn stt_ready() -> bool {
    stt_python().exists()
}

/// Is ollama reachable and does it have the chat model?
pub fn chat_ready() -> bool {
    http_get("/api/tags", 3).is_some_and(|b| b.contains(&model()))
}

/// Speech file -> English text. Blocking: call from a worker thread.
pub fn transcribe(wav: &Path) -> Result<String, String> {
    let script = State::dir().join("transcribe.py");
    std::fs::write(&script, include_str!("../assets/transcribe.py")).map_err(|e| e.to_string())?;
    let out = quiet(Command::new(stt_python()).arg(&script).arg(wav).arg(STT_MODEL)).output().map_err(|e| format!("speech tool: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("speech recognition failed: {}", err.lines().last().unwrap_or("")));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn ollama(body: serde_json::Value) -> Result<String, String> {
    let resp = ureq::post(&format!("{OLLAMA}/api/chat"))
        .timeout(Duration::from_secs(120))
        .send_string(&body.to_string())
        .map_err(|_| "ollama did not answer (is it running?)".to_string())?;
    let text = resp.into_string().map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|_| "ollama did not answer (is it running?)".to_string())?;
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
        "model": model(), "messages": msgs, "stream": false, "format": "json",
        "options": {"temperature": 0.6, "num_predict": 160}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| format!("could not read the answer: {raw}"))?;
    let reply = v["reply"].as_str().unwrap_or("").trim().to_string();
    if reply.is_empty() { Err("empty reply".into()) } else { Ok(reply) }
}

/// Correct one learner sentence. The result equals the input (apart from capitals) when it was fine.
pub fn correct(said: &str) -> Result<String, String> {
    let raw = ollama(serde_json::json!({
        "model": model(), "stream": false, "format": "json",
        "messages": [{"role": "system", "content": FIXER}, {"role": "user", "content": said}],
        "options": {"temperature": 0, "num_predict": 100}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad correction".to_string())?;
    Ok(v["better"].as_str().unwrap_or("").trim().to_string())
}

/// Two short example answers the learner could say next.
pub fn suggestions(scenario: &str, last_question: &str) -> Result<Vec<String>, String> {
    let raw = ollama(serde_json::json!({
        "model": model(), "stream": false, "format": "json",
        "messages": [
            {"role": "system", "content": "You help an English learner who is stuck. Give two different, short (6-12 words), simple, natural example answers in first person. Reply ONLY with JSON {\"a\": \"...\", \"b\": \"...\"}."},
            {"role": "user", "content": format!("Scenario: {scenario}\nThe partner just said: {last_question}")}
        ],
        "options": {"temperature": 0.8, "num_predict": 80}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad answer".to_string())?;
    Ok(["a", "b"].iter().filter_map(|k| v[k].as_str().map(|s| s.to_string())).filter(|s| !s.is_empty()).collect())
}

/// An interview built from one job description.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct CoachPlan {
    pub role: String,
    pub skills: Vec<String>,
    pub questions: Vec<String>,
}

fn clip(s: &str, max: usize) -> String {
    s.trim().chars().take(max).collect()
}

/// Interview questions tailored to a pasted job description and what the learner says about themselves.
pub fn coach_plan(jd: &str, profile: &str) -> Result<CoachPlan, String> {
    let sys = "You are an experienced interviewer. From the job description and the candidate's profile, write the MIDDLE part of one realistic interview: exactly 6 questions, in this order: \
1) why this role or company, 2) a technical question about a skill the job description really asks for, 3) a second technical question about a different skill from the job description, \
4) a question about one of the candidate's own projects (or a practical task if no project is given), 5) a behavioural question (teamwork, deadline, conflict or learning), 6) a situational question about something that could happen in this job. \
Do NOT ask 'tell me about yourself' and do not ask about salary. Every item must be a complete spoken question of 10 to 25 words that a real interviewer would say aloud (never an instruction like 'Develop a...' and never a bare topic). One question each, no numbering. \
Style examples: 'What attracted you to this role, and what do you already know about the work our team does?' / 'Can you explain how you would design a REST API for a booking system and handle errors in it?' / 'Tell me about a time when you disagreed with a teammate. What did you do and how did it end?' \
Also give the job title (max 6 words) and the 4 to 7 most important skills or tools named in the job description. \
Reply ONLY with JSON {\"role\": \"...\", \"skills\": [\"...\"], \"questions\": [\"...\"]}";
    let user = format!("JOB DESCRIPTION:\n{}\n\nCANDIDATE PROFILE:\n{}", clip(jd, 2800), if profile.trim().is_empty() { "(not given: a junior developer)".to_string() } else { clip(profile, 800) });
    let raw = ollama(serde_json::json!({
        "model": model(), "stream": false, "format": "json",
        "messages": [{"role": "system", "content": sys}, {"role": "user", "content": user}],
        "options": {"temperature": 0.4, "num_predict": 700, "num_ctx": 4096}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "could not read the questions".to_string())?;
    let list = |k: &str| -> Vec<String> { v[k].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(|t| t.trim().to_string())).filter(|t| !t.is_empty()).collect()).unwrap_or_default() };
    let mut mid = list("questions");
    mid.truncate(6);
    if mid.len() < 4 {
        return Err("the model gave too few questions - try again, or paste a longer job description".into());
    }
    // the first and last questions are always the same classic ones
    let mut questions = vec!["Good morning, and thank you for coming. Could you please tell me a little about yourself?".to_string()];
    questions.extend(mid);
    questions.push("We are nearly done. When could you join us if selected, and do you have any questions for us?".to_string());
    let mut skills = list("skills");
    skills.truncate(7);
    Ok(CoachPlan { role: v["role"].as_str().unwrap_or("").trim().to_string(), skills, questions })
}

/// A stronger sample answer built only from facts the learner gave.
pub fn model_answer(question: &str, jd: &str, profile: &str, said: &str) -> Result<String, String> {
    let sys = "You coach an Indian job candidate. Write ONE strong sample answer to the interview question: first person, simple natural spoken English, 45 to 70 words, with this shape: direct answer, one reason or example, a short result or what you learned. \
Use ONLY facts found in the candidate profile or in what the candidate already said. If a fact is missing, keep it general and honest: never invent companies, numbers, years or tools. \
Reply ONLY with JSON {\"answer\": \"...\"}";
    let user = format!("QUESTION: {question}\n\nJOB DESCRIPTION (short):\n{}\n\nCANDIDATE PROFILE:\n{}\n\nWHAT THE CANDIDATE SAID:\n{said}", clip(jd, 900), if profile.trim().is_empty() { "(not given)".to_string() } else { clip(profile, 600) });
    let raw = ollama(serde_json::json!({
        "model": model(), "stream": false, "format": "json",
        "messages": [{"role": "system", "content": sys}, {"role": "user", "content": user}],
        "options": {"temperature": 0.5, "num_predict": 200, "num_ctx": 4096}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad answer".to_string())?;
    let a = v["answer"].as_str().unwrap_or("").trim().to_string();
    if a.is_empty() { Err("empty answer".into()) } else { Ok(a) }
}

/// Writing helper. `kind`: polish | followup | linkedin | call.
pub fn write_help(kind: &str, text: &str) -> Result<String, String> {
    let task = match kind {
        "followup" => "Write a short, polite follow-up email or message (60 to 90 words) from a job candidate to a recruiter. Use the notes below (role, when they spoke). Do not invent names, dates or facts: use [Name] for unknown names. Include a greeting, the purpose, a polite question about the status, and a thank-you.",
        "linkedin" => "Write a LinkedIn connection note of at most 280 characters. Friendly, specific, no flattery, no demands. Use only the facts in the notes; use [Name] for the person's name.",
        "call" => "Write a short phone script (what to SAY) for a job candidate calling a recruiter to follow up. 6 to 9 short spoken lines, each on its own line: greeting and 'is this a good time', reason for calling, the question, a polite close. Use [Name] for unknown names. Plain spoken English.",
        _ => "Rewrite the message as polite, clear, professional English that a good Indian developer would send at work. Keep the meaning and roughly the same length, fix grammar, never use the phrase 'revert back' (say 'reply' or 'get back to me'), remove rudeness and repetition, and add nothing that is not there.",
    };
    let raw = ollama(serde_json::json!({
        "model": model(), "stream": false, "format": "json",
        "messages": [{"role": "system", "content": format!("{task}\nReply ONLY with JSON {{\"text\": \"...\"}}")}, {"role": "user", "content": clip(text, 1500)}],
        "options": {"temperature": 0.4, "num_predict": 320, "num_ctx": 4096}, "keep_alive": "30m"
    }))?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "bad answer".to_string())?;
    let t = v["text"].as_str().unwrap_or("").trim().to_string();
    if t.is_empty() { Err("empty answer".into()) } else { Ok(t) }
}

/// Load the chat model into memory in the background so the first reply is not slow.
pub fn warm_up() {
    let _ = ollama(serde_json::json!({
        "model": model(), "stream": false,
        "messages": [{"role": "user", "content": "hi"}],
        "options": {"num_predict": 1}, "keep_alive": "30m"
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // needs ollama running: cargo test --release coach_live -- --ignored --nocapture
    fn coach_live() {
        let jd = "Web Developer (Junior). Build responsive pages with HTML, CSS, JavaScript (React preferred), REST APIs with Node.js or Java Spring Boot, SQL databases, Git, code reviews, agile team. 0-2 years, good English.";
        let prof = "Java developer, Spring Boot, SQL, REST API. Built an expense tracker project.";
        let p = coach_plan(jd, prof).expect("plan");
        println!("{p:#?}");
        let a = model_answer(&p.questions[4], jd, prof, "i make expense tracker in java").expect("answer");
        println!("SAMPLE: {a}");
        println!("FOLLOWUP: {}", write_help("followup", "Web Developer role, spoke with recruiter yesterday evening, want status").expect("w"));
        println!("POLISH: {}", write_help("polish", "sir i am waiting for reply since 3 days please revert back fast").expect("w"));
    }

    #[test]
    fn counts_fillers() {
        assert_eq!(count_fillers("Basically, I worked on the API and, you know, the database."), 2);
        assert_eq!(count_fillers("Um I think uh it works"), 2);
        assert_eq!(count_fillers("I built a REST API."), 0);
    }
}
