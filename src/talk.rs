//! Baat-cheet: talk to a local English partner. You speak, it listens, answers and corrects you.

use crate::data::*;
use crate::drill::canon;
use crate::study::Voice;
use crate::{audio, llm, navigate, Ctx, Route};
use dioxus::prelude::*;

/// (icon, name, Hinglish, scenario for the model, opening line)
pub const SCENARIOS: &[(&str, &str, &str, &str, &str)] = &[
    ("🎤", "Job interview (HR round)", "HR round ke sawaal", "You are an HR interviewer in the first round for a junior Java developer role. Ask about background, motivation, strengths, weaknesses, notice period and salary expectation, one question at a time.", "Good morning, and thank you for coming in. Could you start by telling me a little about yourself?"),
    ("🧠", "Technical interview", "Java / DSA / project sawaal", "You are a technical interviewer for a junior Java backend role. Ask one question at a time about Java basics, collections, OOP, SQL, REST and the learner's projects. Ask a follow-up when the answer is short.", "Hello! Let us begin with something simple. Can you explain the difference between an ArrayList and a LinkedList?"),
    ("🧩", "Explain your project", "Apna project samjhao", "You are an interviewer who wants to understand the learner's own project: purpose, design, challenges, testing and what they would improve. Ask one question at a time.", "I saw a project on your resume. Could you walk me through what it does and how you built it?"),
    ("🤝", "Behavioural interview", "STAR style sawaal", "You are an interviewer asking behavioural questions: conflict, failure, deadline pressure, teamwork, learning something new. Ask one at a time and follow up for details like what they did and the result.", "Let me ask about teamwork. Can you tell me about a time you had a disagreement with someone and how you handled it?"),
    ("📞", "Phone screening call", "Recruiter ki phone call", "You are a recruiter calling a candidate for a first phone screening for a junior Java developer role. This is a PHONE CALL: keep replies short (one or two sentences). Ask if it is a good time, then ask about background, current situation, notice period, salary expectation and motivation, one question at a time. React naturally if the learner asks you to repeat or speak slower, and end by explaining the next steps.", "Hello, may I speak to the candidate, please? Hi, this is Neha from a software company. Is this a good time to talk for about fifteen minutes?"),
    ("🎥", "Video interview", "Video call par interview", "You are an interviewer on a VIDEO CALL for a junior Java backend role. Start with call courtesies (can you hear and see me), then ask a mix of introduction, project and basic technical questions, one at a time. Keep replies short, as on a call. If the learner says audio is bad or asks you to repeat, help patiently.", "Hi, can you see and hear me clearly? Thanks for joining. Shall we begin with a short introduction about yourself?"),
    ("💰", "Salary negotiation", "Offer par salary baat", "You are an HR manager discussing a job offer. The learner may negotiate salary, joining date, variable pay and notice period. Be polite but realistic; you have some flexibility but not unlimited.", "Congratulations, we would like to offer you the role. Do you have any questions about the offer?"),
    ("🧑‍💻", "Daily stand-up", "Team ko update do", "You are a team lead in a daily stand-up meeting. Ask about yesterday's work, today's plan and blockers, then follow up on details.", "Good morning! Let us start the stand-up. What did you work on yesterday?"),
    ("👔", "One-to-one with manager", "Manager se baat", "You are the learner's manager in a one-to-one meeting. Discuss workload, feedback, challenges, goals and learning. Be supportive but ask direct questions.", "Thanks for making time. How have the last few weeks been for you at work?"),
    ("🧾", "Client requirement call", "Client ki zaroorat", "You are a client explaining requirements for a booking website. The learner should ask clarifying questions, repeat requirements to confirm, and discuss timelines.", "Hello, thanks for joining. We would like customers to be able to book appointments from our website. Where would you like to start?"),
    ("🙇", "Apologise for a problem", "Galti par maafi", "You are an unhappy client or manager. A deployment broke something. The learner should apologise, explain what happened, and say how they will fix and prevent it.", "Rahul, our users could not log in this morning. Can you tell me what happened?"),
    ("📊", "Present your work", "Demo / presentation", "You are a manager watching a demo of the learner's work. Ask questions about features, limitations and next steps, and react naturally.", "Go ahead, please show us what you have built this week."),
    ("✉️", "Resign politely", "Naukri chhodna", "You are the learner's manager. The learner wants to resign. React with surprise and ask about reasons, notice period and handover. Stay professional.", "Come in, please. You said you wanted to talk about something important?"),
    ("🙌", "First day at work", "Naye office mein", "You are a friendly colleague (buddy) welcoming the learner on their first day at a software company. Explain the office, team and tools and ask what they need.", "Welcome to the team! I will be your buddy for the first few weeks. How are you feeling about your first day?"),
    ("💬", "Free chat", "Kuch bhi baat karo", "A casual, friendly chat about daily life, hobbies, news, food, family and plans, at natural speed and with natural vocabulary.", "Hi! I am your English partner. How was your day today?"),
    ("🩺", "At the doctor", "Doctor ke paas", "You are a doctor. The learner is a patient describing a health problem. Ask follow-up questions.", "Hello, please sit down. What seems to be the problem?"),
    ("🧭", "Travel & directions", "Safar mein madad", "You are a helpful local person. The learner is travelling and needs directions or help with transport or a hotel.", "Hi! You look a little lost. Can I help you find something?"),
    ("📞", "Customer care call", "Phone par complaint", "You are a customer care agent on a phone call. The learner has a problem with an order or an internet connection.", "Hello, thank you for calling. How may I help you today?"),
];

const MOCK_Q: usize = 8;
const CALL_RULES: &str = "THIS IS A PHONE OR VIDEO CALL. Keep every reply short, as on a call (one or two sentences). Start with the usual call courtesies, and react naturally if the learner asks you to repeat or speak slower.";
const MOCK_RULES: &str = "THIS IS A MOCK INTERVIEW. Be a formal, neutral interviewer. Each turn: react in at most 3 words (for example \"Thank you.\" or \"I see.\"), then ask exactly ONE new question you have not asked before. Never give hints, never praise at length, never explain.";

#[derive(Clone, PartialEq)]
struct Msg {
    user: bool,
    text: String,
    better: String,
    tip: String,
    checked: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    Recording,
    Hearing,
    Thinking,
}

/// The corrected sentence word by word; `true` marks words that were not in what the learner said.
fn diff_words(better: &str, said: &str) -> Vec<(String, bool)> {
    let mut pool: Vec<String> = canon(said).split(' ').map(String::from).collect();
    better
        .split_whitespace()
        .map(|w| {
            let c = canon(w);
            match pool.iter().position(|x| *x == c) {
                Some(p) => { pool.remove(p); (w.to_string(), false) }
                None => (w.to_string(), !c.is_empty()),
            }
        })
        .collect()
}

fn scroll_down() {
    let _ = document::eval("setTimeout(function(){var s=document.getElementById('scroller'); if(s){s.scrollTop=s.scrollHeight;}},60);");
}

#[component]
pub fn Talk() -> Element {
    let ctx = use_context::<Ctx>();
    let mut sel = use_signal(|| std::env::var("BOLO_SCENARIO").ok().and_then(|v| v.parse::<usize>().ok()).map(|i| (i, false)));
    let mut recheck = use_signal(|| 0u32);
    let st = ctx.state.read().clone();
    let today_talks = st.days.get(&today()).map(|d| d.talks).unwrap_or(0);
    let fixes = st.my_fixes.len();
    let today_log = st.days.get(&today()).cloned().unwrap_or_default();
    let words_today = today_log.spoken_words;
    let avg_words = if today_talks > 0 { today_log.spoken_words / today_talks } else { 0 };
    let _ = *recheck.read();
    let (stt, chat) = (llm::stt_ready(), llm::chat_ready());
    use_hook(move || {
        if chat {
            spawn(async move { let _ = tokio::task::spawn_blocking(llm::warm_up).await; });
        }
    });

    if let Some((i, mock)) = *sel.read() {
        return rsx! { Chat { key: "{i}-{mock}", idx: i, mock, on_exit: move |_| sel.set(None) } };
    }

    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Baat-cheet · Talk")} }
            p { class: "sub", {crate::i18n::tr("Koi baat karne wala nahi? Koi baat nahi — ye app tumhara English partner hai. Tum bolo, wo suna aur jawab dega, aur galti hui to sahi tareeka bata dega. Sab kuch is laptop par hi chalta hai.")} }
            if !stt || !chat {
                div { class: "card feedback bad",
                    div { class: "fb-title", {crate::i18n::tr("Setup baaki hai")} }
                    p { if !stt { {crate::i18n::tr("• Awaaz pehchanne wala tool (Whisper) install nahi hai.  ")} } if !chat { {crate::i18n::tr("• Baat-cheet wala model (qwen2.5:3b) abhi tayyar nahi hai (ollama chalu hona chahiye).  ")} } }
                    p { class: "sm", {crate::i18n::tr("Neeche ka button dabao — app khud sab install kar legi (internet chahiye, ek hi baar).")} }
                    crate::study::InstallAi {}
                    div { class: "row-actions", button { class: "btn", onclick: move |_| recheck += 1, {crate::i18n::tr("Dobara check karo")} } }
                }
            }
            div { class: "stats",
                div { class: "stat", div { class: "n", "{today_talks}" } div { class: "l", {crate::i18n::tr("aaj ke jawab (target 5)")} } }
                div { class: "stat", div { class: "n", "{fixes}" } div { class: "l", {crate::i18n::tr("meri sudhari hui galtiyan")} } }
                div { class: "stat", div { class: "n", "{words_today}" } div { class: "l", {crate::i18n::tr("aaj bole gaye words")} } }
                div { class: "stat", div { class: "n", "{avg_words}" } div { class: "l", {crate::i18n::tr("ek jawab mein average words")} } }
            }
            div { class: "section-title", crate::icon::Ico { name: "target".to_string(), size: 20 }, " ", {crate::i18n::tr("Mock interview · Asli practice")} }
            p { class: "muted", {crate::i18n::trf("Asli interview jaisa round: {} sawaal, koi hint nahi, galtiyan sirf end mein, phir summary ki aapne kaisa kiya.", &[MOCK_Q.to_string()])} }
            div { class: "grid",
                for (i, label) in [(0usize, "HR round"), (1, "Technical round"), (3, "Behavioural round"), (4, "Call interview round")] {
                    button { key: "m{i}", class: "card topic", disabled: !stt || !chat, onclick: move |_| sel.set(Some((i, true))),
                        div { class: "ticon", crate::icon::Ico { name: "briefcase".to_string(), size: 26 } } div { class: "tn", "{crate::i18n::tr(label)}" } div { class: "muted sm", {crate::i18n::trf("{} sawaal", &[MOCK_Q.to_string()])} } }
                }
            }
            if fixes > 0 { div { class: "row-actions", button { class: "btn", onclick: move |_| navigate(ctx, Route::Drill("myfix")), crate::icon::Ico { name: "wrench".to_string(), size: 18 }, " Apni galtiyon ka test" } } }
            div { class: "section-title", {crate::i18n::tr("Topic chuno")} }
            div { class: "grid",
                for (i, (ico, name, hi, _, _)) in SCENARIOS.iter().enumerate() {
                    button { key: "{i}", class: "card topic", disabled: !stt || !chat, onclick: move |_| sel.set(Some((i, false))),
                        div { class: "ticon", crate::icon::Ico { name: ico.to_string(), size: 26 } } div { class: "tn", "{crate::i18n::tr(name)}" } div { class: "muted sm", "{crate::i18n::tr(hi)}" } }
                }
            }
        }
    }
}

#[component]
fn Chat(idx: usize, mock: bool, on_exit: EventHandler<()>) -> Element {
    let mut ctx = use_context::<Ctx>();
    let (ico, name, _, scenario0, opener) = SCENARIOS[idx];
    let scenario: &'static str = use_hook(move || if mock { Box::leak(format!("{scenario0}\n{MOCK_RULES}").into_boxed_str()) as &'static str } else { scenario0 });
    let mut done = use_signal(|| false);
    let mut msgs = use_signal(move || vec![Msg { user: false, text: opener.to_string(), better: String::new(), tip: String::new(), checked: false }]);
    let mut phase = use_signal(|| Phase::Idle);
    let mut err = use_signal(String::new);
    let mut typed = use_signal(String::new);
    let mut hints = use_signal(Vec::<String>::new);

    // speak the opening line once
    use_hook(move || {
        let wpm = ctx.state.peek().normal_wpm;
        let _ = audio::speak(opener, wpm);
    });

    // send one learner message to the partner
    let mut send = move |text: String| {
        let text = text.trim().to_string();
        if text.is_empty() {
            phase.set(Phase::Idle);
            return;
        }
        hints.set(vec![]);
        ctx.state.with_mut(|s| { let d = s.day_mut(); d.spoken_words += text.split_whitespace().count() as u32; d.fillers += llm::count_fillers(&text); });
        msgs.with_mut(|m| m.push(Msg { user: true, text: text.clone(), better: String::new(), tip: String::new(), checked: false }));
        phase.set(Phase::Thinking);
        scroll_down();
        let history: Vec<(bool, String)> = msgs.peek().iter().map(|m| (m.user, m.text.clone())).collect();
        let answered = history.iter().filter(|h| h.0).count();
        if mock && answered >= MOCK_Q {
            // last answer: no more questions; correct it, then show the summary
            ctx.state.with_mut(|s| s.day_mut().talks += 1);
            spawn(async move {
                let s2 = text.clone();
                let fixed = tokio::task::spawn_blocking(move || llm::correct(&s2)).await;
                if let Ok(Ok(better)) = fixed {
                    msgs.with_mut(|m| { if let Some(last) = m.iter_mut().rev().find(|x| x.user && x.text == text) { last.checked = true; if !better.is_empty() && canon(&better) != canon(&text) { last.better = better; } } });
                }
                phase.set(Phase::Idle);
                done.set(true);
                scroll_down();
            });
            return;
        }
        spawn(async move {
            let said = text.clone();
            let r = tokio::task::spawn_blocking(move || llm::partner_reply(scenario, &history)).await;
            match r {
                Ok(Ok(reply)) => {
                    msgs.with_mut(|m| m.push(Msg { user: false, text: reply.clone(), better: String::new(), tip: String::new(), checked: false }));
                    ctx.state.with_mut(|s| s.day_mut().talks += 1);
                    let wpm = ctx.state.peek().normal_wpm;
                    let _ = audio::speak(&reply, wpm);
                    err.set(String::new());
                    phase.set(Phase::Idle);
                    scroll_down();
                    // the correction arrives a few seconds later, so the conversation is not delayed
                    let s2 = said.clone();
                    let fixed = tokio::task::spawn_blocking(move || llm::correct(&s2)).await;
                    if let Ok(Ok(better)) = fixed {
                        msgs.with_mut(|m| { if let Some(last) = m.iter_mut().rev().find(|x| x.user && x.text == said) { last.checked = true; } });
                        if !better.is_empty() && canon(&better) != canon(&said) {
                            msgs.with_mut(|m| {
                                if let Some(last) = m.iter_mut().rev().find(|x| x.user && x.text == said) {
                                    last.better = better.clone();
                                }
                            });
                            ctx.state.with_mut(|s| {
                                s.my_fixes.retain(|f| f.said != said);
                                s.my_fixes.push(Fix { said: said.clone(), better, tip: String::new() });
                                if s.my_fixes.len() > 200 {
                                    s.my_fixes.remove(0);
                                }
                            });
                        }
                    }
                    scroll_down();
                    return;
                }
                Ok(Err(e)) => err.set(e),
                Err(e) => err.set(e.to_string()),
            }
            phase.set(Phase::Idle);
            scroll_down();
        });
    };

    // developer hook for tests: BOLO_SAY="..." sends that sentence after a few seconds
    use_hook(move || {
        if let Ok(t) = std::env::var("BOLO_SAY") {
            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(4)).await;
                send(t);
            });
        }
    });

    let ph = *phase.read();
    let list = msgs.read().clone();
    let status = match ph {
        Phase::Idle => crate::i18n::tr("Mic dabao aur English mein bolo"),
        Phase::Recording => crate::i18n::tr("Sun raha hoon… bol chuke to Stop dabao"),
        Phase::Hearing => crate::i18n::tr("Tumhari awaaz samajh raha hoon…"),
        Phase::Thinking => crate::i18n::tr("Jawab soch raha hoon… (thoda time lagta hai)"),
    };
    let last_ai = list.iter().rev().find(|m| !m.user).map(|m| m.text.clone()).unwrap_or_default();

    rsx! {
        div { class: "page narrow chat",
            div { class: "drill-top",
                button { class: "btn back", onclick: move |_| { audio::stop_all(); on_exit.call(()) }, "← ", crate::icon::Ico { name: ico.to_string(), size: 18 }, " {crate::i18n::tr(name)}" }
                span { class: "muted sm", if mock { {crate::i18n::trf("Sawaal {} / {}", &[((list.iter().filter(|m| m.user).count() + 1).min(MOCK_Q)).to_string(), MOCK_Q.to_string()])} } else { {crate::i18n::trf("{} jawab", &[(list.iter().filter(|m| m.user).count()).to_string()])} } }
            }
            for (i, m) in list.iter().enumerate() {
                div { key: "{i}", class: if m.user { "msgrow me" } else { "msgrow" },
                    div { class: if m.user { "bubble me" } else { "bubble ai" },
                        "{m.text}"
                        if !m.user { Voice { ctx, text: m.text.clone() } }
                    }
                    if m.user && !m.better.is_empty() && (!mock || *done.read()) {
                        div { class: "fixcard",
                            div { class: "fx good", span { "✓" }
                                span { for (k, (w, chg)) in diff_words(&m.better, &m.text).into_iter().enumerate() { span { key: "{k}", class: if chg { "chg" } else { "" }, "{w} " } } }
                                Voice { ctx, text: m.better.clone() } }
                            div { class: "muted sm", {crate::i18n::tr("Better version (badle hue words highlighted). AI kabhi-kabhi galat bhi ho sakta hai.")} }
                        }
                    }
                    if m.user && m.better.is_empty() && m.checked && (!mock || *done.read()) { div { class: "muted sm okline", {crate::i18n::tr("✓ Sahi English!")} } }
                }
            }
            if !hints.read().is_empty() {
                div { class: "hintbox",
                    div { class: "muted sm", {crate::i18n::tr("Aise bol sakte ho:")} }
                    for (k, h) in hints.read().iter().enumerate() {
                        div { key: "{k}", class: "exrow", Voice { ctx, text: h.clone() } div { class: "en", "{h}" } }
                    }
                }
            }
            if !err.read().is_empty() { div { class: "card feedback bad", "{err}" } }

            if *done.read() { MockSummary { msgs: list.clone(), ctx, on_exit } } else {
            div { class: "talkbar",
                div { class: "muted sm tstat", "{status}" }
                div { class: "talkrow",
                    button { class: match ph { Phase::Recording => "micbtn rec", _ => "micbtn" }, disabled: matches!(ph, Phase::Hearing | Phase::Thinking),
                        onclick: move |_| {
                            if ph == Phase::Idle {
                                audio::stop_all();
                                err.set(String::new());
                                match audio::start_recording("talk") { Ok(_) => phase.set(Phase::Recording), Err(e) => err.set(e) }
                            } else if ph == Phase::Recording {
                                audio::stop_recording();
                                phase.set(Phase::Hearing);
                                spawn(async move {
                                    let r = tokio::task::spawn_blocking(|| llm::transcribe(&audio::rec_path("talk"))).await;
                                    match r {
                                        Ok(Ok(t)) if !t.trim().is_empty() => send(t),
                                        Ok(Ok(_)) => { err.set(crate::i18n::tr("Kuch sunai nahi diya — mic ke paas jaakar dobara bolo.")); phase.set(Phase::Idle); }
                                        Ok(Err(e)) => { err.set(e); phase.set(Phase::Idle); }
                                        Err(e) => { err.set(e.to_string()); phase.set(Phase::Idle); }
                                    }
                                });
                            }
                        },
                        if ph == Phase::Recording { crate::icon::Ico { name: "square".to_string(), size: 22 } } else { crate::icon::Ico { name: "mic".to_string(), size: 22 } } }
                    input { class: "input", placeholder: crate::i18n::tr("ya yahan likho aur Enter dabao"), value: "{typed}", disabled: ph != Phase::Idle,
                        oninput: move |e| typed.set(e.value()),
                        onkeydown: move |e| { if e.key() == Key::Enter && ph == Phase::Idle { let t = typed.read().clone(); typed.set(String::new()); send(t); } } }
                    if !mock { button { class: "btn", disabled: ph != Phase::Idle, onclick: move |_| {
                        let q = last_ai.clone();
                        spawn(async move {
                            let r = tokio::task::spawn_blocking(move || llm::suggestions(scenario, &q)).await;
                            if let Ok(Ok(h)) = r { hints.set(h); }
                            scroll_down();
                        });
                    }, crate::icon::Ico { name: "lightbulb".to_string(), size: 18 }, " ", {crate::i18n::tr("Kya bolun?")} } }
                }
            }
            }
        }
    }
}


#[component]
fn MockSummary(msgs: Vec<Msg>, ctx: Ctx, on_exit: EventHandler<()>) -> Element {
    let answers: Vec<&Msg> = msgs.iter().filter(|m| m.user).collect();
    let n = answers.len().max(1);
    let counts: Vec<usize> = answers.iter().map(|m| m.text.split_whitespace().count()).collect();
    let total: usize = counts.iter().sum();
    let avg = total / n;
    let short = counts.iter().filter(|c| **c < 8).count();
    let fillers: u32 = answers.iter().map(|m| llm::count_fillers(&m.text)).sum();
    let wrong: Vec<&Msg> = answers.iter().copied().filter(|m| !m.better.is_empty()).collect();
    let clean = n - wrong.len().min(n);
    let mut notes: Vec<String> = vec![];
    if avg < 12 {
        notes.push(crate::i18n::tr("Aapke jawab chhote hain. Interview mein 25–40 words (15–20 second) ka jawab do: jawab, ek wajah, ek example."));
    } else if avg < 25 {
        notes.push(crate::i18n::tr("Length theek hai. Jo jawab aam rahe unmein ek asli example jodo."));
    } else {
        notes.push(crate::i18n::tr("Jawab ki length acchi hai. Har jawab ek minute se kam rakho."));
    }
    if short > 0 {
        notes.push(crate::i18n::trf("{} jawab 8 words se chhote the. Pehle unhe badhao.", &[short.to_string()]));
    }
    if fillers > 3 {
        notes.push(crate::i18n::trf("Aapne {} filler phrases use kiye (basically, actually, you know...). Unki jagah ek second chup raho.", &[fillers.to_string()]));
    }
    if wrong.is_empty() {
        notes.push(crate::i18n::tr("Koi grammar correction nahi chahiye thi. Badhiya!"));
    } else {
        notes.push(crate::i18n::trf("{} / {} jawab mein sudhaar tha. Ye Meri galtiyan mein save hain — inhe practice karo.", &[wrong.len().to_string(), n.to_string()]));
    }
    rsx! {
        div { class: "card result",
            div { class: "stage-t", crate::icon::Ico { name: "chart-column".to_string(), size: 22 }, " ", {crate::i18n::tr("Interview ka summary")} }
            div { class: "stats",
                div { class: "stat", div { class: "n", "{n}" } div { class: "l", {crate::i18n::tr("jawab")} } }
                div { class: "stat", div { class: "n", "{avg}" } div { class: "l", {crate::i18n::tr("ek jawab mein average words")} } }
                div { class: "stat", div { class: "n", "{clean}/{n}" } div { class: "l", {crate::i18n::tr("bina galti ke jawab")} } }
                div { class: "stat", div { class: "n", "{fillers}" } div { class: "l", {crate::i18n::tr("filler phrases")} } }
            }
            div { class: "notes",
                for (k, t) in notes.iter().enumerate() { p { key: "{k}", class: "lp", "{t}" } }
            }
            if !wrong.is_empty() {
                div { class: "section-title", {crate::i18n::tr("Agli baar aise bolo")} }
                for (k, m) in wrong.iter().enumerate() {
                    div { key: "{k}", class: "fixcard",
                        div { class: "fx bad", span { "✗" } span { class: "strike", "{m.text}" } }
                        div { class: "fx good", span { "✓" }
                            span { for (q, (w, chg)) in diff_words(&m.better, &m.text).into_iter().enumerate() { span { key: "{q}", class: if chg { "chg" } else { "" }, "{w} " } } }
                            Voice { ctx, text: m.better.clone() } }
                    }
                }
            }
            div { class: "row-actions",
                button { class: "btn", onclick: move |_| on_exit.call(()), {crate::i18n::tr("Wapas")} }
                if !wrong.is_empty() { button { class: "btn primary", onclick: move |_| navigate(ctx, Route::Drill("myfix")), {crate::i18n::tr("Apni galtiyon ka test")} } }
            }
        }
    }
}
