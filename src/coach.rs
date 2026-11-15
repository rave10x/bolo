//! Interview Coach: paste a job description, get an interview built for it, answer by voice, and see
//! corrected answers plus a stronger sample answer for each question. Also a small writing helper.
//! Everything runs on this laptop (Whisper, ollama, Piper).

use crate::data::*;
use crate::drill::canon;
use crate::study::Voice;
use crate::talk::diff_words;
use crate::{audio, llm, Ctx};
use dioxus::prelude::*;

const SAMPLE_JD: &str = "Web Developer (Junior) - We are looking for a web developer to build and maintain customer-facing web applications. \
Responsibilities: develop responsive front-end pages with HTML, CSS and JavaScript (React preferred); build REST APIs with Node.js or Java Spring Boot; \
work with SQL databases; write clean, tested code; use Git and take part in code reviews; work with designers and product managers in an agile team. \
Requirements: 0-2 years of experience, strong basics in JavaScript, understanding of HTTP and REST, good communication in English, willingness to learn.";

#[derive(Clone, PartialEq, Default)]
struct Turn {
    q: String,
    a: String,
    better: String,
    sample: String,
    /// grammar check finished / sample answer finished
    fixed: bool,
    sampled: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    Recording,
    Hearing,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Interview,
    Writing,
}

fn scroll_top() {
    let _ = document::eval("setTimeout(function(){var s=document.getElementById('scroller'); if(s){s.scrollTop=0;}},30);");
}

#[component]
pub fn Coach() -> Element {
    let mut ctx = use_context::<Ctx>();
    let mut tab = use_signal(|| Tab::Interview);
    let mut jd = use_signal(|| ctx.state.peek().coach_jd.clone());
    let mut profile = use_signal(|| ctx.state.peek().coach_profile.clone());
    let mut plan = use_signal(|| None::<llm::CoachPlan>);
    let mut running = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let mut err = use_signal(String::new);
    let mut recheck = use_signal(|| 0u32);
    let _ = *recheck.read();
    let (stt, chat) = (llm::stt_ready(), llm::chat_ready());

    if *running.read() {
        if let Some(p) = plan.read().clone() {
            return rsx! { Run { plan: p, jd: jd.read().clone(), profile: profile.read().clone(), on_exit: move |_| running.set(false) } };
        }
    }

    let tabs = [(Tab::Interview, "Interview taiyari"), (Tab::Writing, "Likhne mein madad")];
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Interview Coach")} }
            p { class: "sub", {crate::i18n::tr("Job description paste karo — app usi job ke hisaab se sawaal banati hai, tum bolkar jawab dete ho, aur har jawab ka sahi version aur ek mazboot sample jawab milta hai. Sab kuch offline, is laptop par.")} }
            if !chat {
                div { class: "card feedback bad",
                    div { class: "fb-title", {crate::i18n::tr("Setup baaki hai")} }
                    p { {crate::i18n::tr("• Baat-cheet wala model (qwen2.5:3b) abhi tayyar nahi hai (ollama chalu hona chahiye).")} }
                    crate::study::InstallAi {}
                    div { class: "row-actions", button { class: "btn", onclick: move |_| recheck += 1, {crate::i18n::tr("Dobara check karo")} } }
                }
            }
            div { class: "chips",
                for (t, label) in tabs {
                    button { key: "{label}", class: if *tab.read() == t { "chip on" } else { "chip" }, onclick: move |_| tab.set(t), {crate::i18n::tr(label)} }
                }
            }
            if *tab.read() == Tab::Writing {
                Writing {}
            } else if let Some(p) = plan.read().clone() {
                // the generated interview
                div { class: "card",
                    div { class: "stage-t", crate::icon::Ico { name: "briefcase".to_string(), size: 22 }, " ", if p.role.is_empty() { {crate::i18n::tr("Tumhara interview")} } else { "{p.role}" } }
                    if !p.skills.is_empty() {
                        div { class: "muted sm", {crate::i18n::tr("Is job ke important skills (✓ = tumhare profile mein likha hai):")} }
                        div { class: "chips",
                            for (k, sk) in p.skills.iter().enumerate() {
                                {
                                    let have = profile.read().to_lowercase().contains(&sk.to_lowercase());
                                    rsx! { span { key: "{k}", class: if have { "chip on" } else { "chip" }, if have { "✓ " } "{sk}" } }
                                }
                            }
                        }
                    }
                    div { class: "muted sm", {crate::i18n::tr("Interview mein ye sawaal aayenge (suno aur pehle se socho):")} }
                    div { class: "exlist",
                        for (k, q) in p.questions.iter().enumerate() {
                            div { key: "{k}", class: "exrow", Voice { ctx, text: q.clone() } div { div { class: "en", "{k + 1}. {q}" } } }
                        }
                    }
                    div { class: "row-actions",
                        button { class: "btn primary", onclick: move |_| { running.set(true); scroll_top(); }, crate::icon::Ico { name: "mic".to_string(), size: 18 }, " ", {crate::i18n::tr("Interview shuru karo")} }
                        button { class: "btn", onclick: move |_| plan.set(None), {crate::i18n::tr("Job description badlo")} }
                    }
                }
            } else {
                div { class: "card",
                    div { class: "stage-t", {crate::i18n::tr("1. Job description")} }
                    textarea { class: "input area", rows: "9", placeholder: crate::i18n::tr("Naukri ka job description yahan paste karo (LinkedIn / Naukri / company site se)"), value: "{jd}", oninput: move |e| jd.set(e.value()) }
                    div { class: "stage-t", style: "margin-top:14px", {crate::i18n::tr("2. Apne baare mein (optional, par behtar jawab ke liye likho)")} }
                    textarea { class: "input area", rows: "4", placeholder: crate::i18n::tr("Jaise: Java developer, Spring Boot, SQL, REST API. Project: expense tracker. 1 saal ki practice."), value: "{profile}", oninput: move |e| profile.set(e.value()) }
                    if !err.read().is_empty() { div { class: "card feedback bad", "{err}" } }
                    div { class: "row-actions",
                        button { class: "btn primary", disabled: !chat || *busy.read() || jd.read().trim().len() < 40, onclick: move |_| {
                            let (j, p) = (jd.read().clone(), profile.read().clone());
                            ctx.state.with_mut(|s| { s.coach_jd = j.clone(); s.coach_profile = p.clone(); });
                            busy.set(true);
                            err.set(String::new());
                            spawn(async move {
                                let r = tokio::task::spawn_blocking(move || llm::coach_plan(&j, &p)).await;
                                match r {
                                    Ok(Ok(pl)) => plan.set(Some(pl)),
                                    Ok(Err(e)) => err.set(e),
                                    Err(e) => err.set(e.to_string()),
                                }
                                busy.set(false);
                            });
                        }, if *busy.read() { {crate::i18n::tr("Sawaal bana raha hoon… (20-40 second)")} } else { {crate::i18n::tr("Mere liye interview banao")} } }
                        button { class: "btn", onclick: move |_| jd.set(SAMPLE_JD.to_string()), {crate::i18n::tr("Example job description")} }
                    }
                    if jd.read().trim().len() < 40 { div { class: "muted sm", {crate::i18n::tr("Kam se kam 2-3 line ka job description chahiye.")} } }
                    if !stt { div { class: "muted sm", {crate::i18n::tr("Mic wala tool (Whisper) install nahi hai — tab tum jawab type kar sakte ho.")} } }
                }
                History {}
            }
        }
    }
}

#[component]
fn Run(plan: llm::CoachPlan, jd: String, profile: String, on_exit: EventHandler<()>) -> Element {
    let mut ctx = use_context::<Ctx>();
    let n = plan.questions.len();
    let data = use_signal(|| (plan.questions.clone(), jd.clone(), profile.clone()));
    let mut turns = use_signal(|| plan.questions.iter().map(|q| Turn { q: q.clone(), ..Default::default() }).collect::<Vec<_>>());
    let mut at = use_signal(|| 0usize);
    let mut phase = use_signal(|| Phase::Idle);
    let mut err = use_signal(String::new);
    let mut typed = use_signal(String::new);
    let stt = llm::stt_ready();

    // remember the finished interview (a hook, so it runs on every render, before any branch)
    let role = plan.role.clone();
    use_effect(move || {
        if *at.read() >= n {
            let t = turns.peek().clone();
            let words: usize = t.iter().map(|x| x.a.split_whitespace().count()).sum();
            let fillers: u32 = t.iter().map(|x| llm::count_fillers(&x.a)).sum();
            ctx.state.with_mut(|s| {
                s.coach_runs.push(CoachRun { date: today(), role: role.clone(), answers: n as u32, avg_words: (words / n.max(1)) as u32, fillers });
                if s.coach_runs.len() > 30 {
                    s.coach_runs.remove(0);
                }
            });
        }
    });

    // ask the first question out loud
    use_hook(move || {
        let wpm = ctx.state.peek().normal_wpm;
        let q = data.peek().0[0].clone();
        let _ = audio::speak(&q, wpm);
    });

    // take one answer: store it, ask the next question, and check the answer in the background
    let mut send = move |text: String| {
        let text = text.trim().to_string();
        if text.is_empty() {
            phase.set(Phase::Idle);
            return;
        }
        let i = *at.peek();
        if i >= n {
            return;
        }
        ctx.state.with_mut(|s| {
            let d = s.day_mut();
            d.spoken_words += text.split_whitespace().count() as u32;
            d.fillers += llm::count_fillers(&text);
            d.talks += 1;
        });
        turns.with_mut(|t| t[i].a = text.clone());
        at.set(i + 1);
        phase.set(Phase::Idle);
        let (qs, j, p) = data.peek().clone();
        if i + 1 < n {
            let wpm = ctx.state.peek().normal_wpm;
            let _ = audio::speak(&qs[i + 1], wpm);
        } else {
            scroll_top();
        }
        let question = qs[i].clone();
        spawn(async move {
            let said = text.clone();
            let fixed = tokio::task::spawn_blocking({ let s = said.clone(); move || llm::correct(&s) }).await;
            let mut better = String::new();
            if let Ok(Ok(b)) = fixed {
                if !b.is_empty() && canon(&b) != canon(&said) {
                    better = b;
                }
            }
            turns.with_mut(|t| { t[i].better = better.clone(); t[i].fixed = true; });
            if !better.is_empty() {
                ctx.state.with_mut(|s| {
                    s.my_fixes.retain(|f| f.said != said);
                    s.my_fixes.push(Fix { said: said.clone(), better, tip: String::new() });
                    if s.my_fixes.len() > 200 {
                        s.my_fixes.remove(0);
                    }
                });
            }
            let sample = tokio::task::spawn_blocking(move || llm::model_answer(&question, &j, &p, &said)).await;
            turns.with_mut(|t| {
                if let Ok(Ok(a)) = sample {
                    t[i].sample = a;
                }
                t[i].sampled = true;
            });
        });
    };

    let ph = *phase.read();
    let cur = *at.read();
    let list = turns.read().clone();
    let finished = cur >= n;

    rsx! {
        div { class: "page narrow chat",
            div { class: "drill-top",
                button { class: "btn back", onclick: move |_| { audio::stop_all(); on_exit.call(()) }, "← ", {crate::i18n::tr("Coach")} }
                span { class: "muted sm", if finished { {crate::i18n::tr("Interview poora ✓")} } else { {crate::i18n::trf("Sawaal {} / {}", &[(cur + 1).to_string(), n.to_string()])} } }
            }
            if !finished {
                div { class: "bar", i { style: "width:{cur as f32 * 100.0 / n as f32}%" } }
                for (k, t) in list.iter().enumerate().take(cur) {
                    div { key: "{k}", class: "msgrow",
                        div { class: "bubble ai", "{t.q}" }
                    }
                    div { key: "a{k}", class: "msgrow me", div { class: "bubble me", "{t.a}" } }
                }
                div { class: "msgrow",
                    div { class: "who", {crate::i18n::tr("Interviewer")} }
                    div { class: "bubble ai now", "{list[cur].q}" Voice { ctx, text: list[cur].q.clone() } }
                }
                if !err.read().is_empty() { div { class: "card feedback bad", "{err}" } }
                div { class: "talkbar",
                    div { class: "muted sm tstat", match ph {
                        Phase::Idle => crate::i18n::tr("Mic dabao aur jawab do (25-40 words)"),
                        Phase::Recording => crate::i18n::tr("Sun raha hoon… bol chuke to Stop dabao"),
                        Phase::Hearing => crate::i18n::tr("Tumhari awaaz samajh raha hoon…"),
                    } }
                    div { class: "talkrow",
                        if stt {
                            button { class: if ph == Phase::Recording { "micbtn rec" } else { "micbtn" }, disabled: ph == Phase::Hearing,
                                onclick: move |_| {
                                    if ph == Phase::Idle {
                                        audio::stop_all();
                                        err.set(String::new());
                                        match audio::start_recording("coach") { Ok(_) => phase.set(Phase::Recording), Err(e) => err.set(e) }
                                    } else if ph == Phase::Recording {
                                        audio::stop_recording();
                                        phase.set(Phase::Hearing);
                                        spawn(async move {
                                            let r = tokio::task::spawn_blocking(|| llm::transcribe(&audio::rec_path("coach"))).await;
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
                        }
                        input { class: "input", placeholder: crate::i18n::tr("ya yahan likho aur Enter dabao"), value: "{typed}", disabled: ph != Phase::Idle,
                            oninput: move |e| typed.set(e.value()),
                            onkeydown: move |e| { if e.key() == Key::Enter && ph == Phase::Idle { let t = typed.read().clone(); typed.set(String::new()); send(t); } } }
                    }
                }
            } else {
                Summary { turns: list.clone(), ctx, on_exit }
            }
        }
    }
}

#[component]
fn Summary(turns: Vec<Turn>, ctx: Ctx, on_exit: EventHandler<()>) -> Element {
    let n = turns.len().max(1);
    let counts: Vec<usize> = turns.iter().map(|t| t.a.split_whitespace().count()).collect();
    let avg = counts.iter().sum::<usize>() / n;
    let short = counts.iter().filter(|c| **c < 15).count();
    let fillers: u32 = turns.iter().map(|t| llm::count_fillers(&t.a)).sum();
    let pending = turns.iter().any(|t| !t.sampled);
    let wrong = turns.iter().filter(|t| !t.better.is_empty()).count();
    let mut notes: Vec<String> = vec![];
    if avg < 20 {
        notes.push(crate::i18n::tr("Jawab chhote hain. Interview mein 25–50 words do: seedha jawab, ek wajah ya example, aur result."));
    } else if avg < 60 {
        notes.push(crate::i18n::tr("Jawab ki length theek hai. Har jawab mein ek asli example zaroor jodo."));
    } else {
        notes.push(crate::i18n::tr("Jawab lambe hain. Ek minute se kam rakho: jawab, example, result — bas."));
    }
    if short > 0 { notes.push(crate::i18n::trf("{} jawab 15 words se chhote the — pehle unhe badhao.", &[short.to_string()])); }
    if fillers > 3 { notes.push(crate::i18n::trf("{} filler words (basically, actually, you know…). Unki jagah ek second chup raho.", &[fillers.to_string()])); }
    rsx! {
        div { class: "card result",
            div { class: "stage-t", crate::icon::Ico { name: "chart-column".to_string(), size: 22 }, " ", {crate::i18n::tr("Interview ka summary")} }
            div { class: "stats",
                div { class: "stat", div { class: "n", "{n}" } div { class: "l", {crate::i18n::tr("jawab")} } }
                div { class: "stat", div { class: "n", "{avg}" } div { class: "l", {crate::i18n::tr("ek jawab mein average words")} } }
                div { class: "stat", div { class: "n", "{n - wrong.min(n)}/{n}" } div { class: "l", {crate::i18n::tr("bina galti ke jawab")} } }
                div { class: "stat", div { class: "n", "{fillers}" } div { class: "l", {crate::i18n::tr("filler phrases")} } }
            }
            div { class: "notes", for (k, t) in notes.iter().enumerate() { p { key: "{k}", class: "lp", "{t}" } } }
            if pending { div { class: "muted sm", {crate::i18n::tr("Sample jawab abhi ban rahe hain… (har ek mein kuch second lagte hain)")} } }
        }
        for (k, t) in turns.iter().enumerate() {
            div { key: "{k}", class: "card",
                div { class: "kicker", {crate::i18n::trf("Sawaal {}", &[(k + 1).to_string()])} }
                div { class: "stage-t", "{t.q}" }
                div { class: "fx", span { class: "muted sm", {crate::i18n::tr("Tumne kaha:")} } " {t.a}" }
                if !t.better.is_empty() {
                    div { class: "fixcard",
                        div { class: "fx good", span { "✓" }
                            span { for (q, (w, chg)) in diff_words(&t.better, &t.a).into_iter().enumerate() { span { key: "{q}", class: if chg { "chg" } else { "" }, "{w} " } } }
                            Voice { ctx, text: t.better.clone() } }
                        div { class: "muted sm", {crate::i18n::tr("Sahi version (badle hue words highlighted). AI kabhi-kabhi galat bhi ho sakta hai.")} }
                    }
                } else if t.fixed {
                    div { class: "muted sm okline", {crate::i18n::tr("✓ Grammar theek hai")} }
                }
                if !t.sample.is_empty() {
                    div { class: "hintbox",
                        div { class: "muted sm", crate::icon::Ico { name: "lightbulb".to_string(), size: 16 }, " ", {crate::i18n::tr("Sample mazboot jawab — sirf wahi bolo jo tumhare saath sach mein hua ho. AI kabhi-kabhi galat bhi ho sakta hai:")} }
                        div { class: "exrow", Voice { ctx, text: t.sample.clone() } div { class: "en", "{t.sample}" } }
                    }
                } else if !t.sampled {
                    div { class: "muted sm", {crate::i18n::tr("Sample jawab ban raha hai…")} }
                }
            }
        }
        div { class: "row-actions",
            button { class: "btn primary", onclick: move |_| on_exit.call(()), {crate::i18n::tr("Wapas Coach par")} }
            if ctx.state.read().my_fixes.len() > 0 { button { class: "btn", onclick: move |_| crate::navigate(ctx, crate::Route::Drill("myfix")), {crate::i18n::tr("Apni galtiyon ka test")} } }
        }
    }
}

const WRITE_KINDS: [(&str, &str, &str); 4] = [
    ("polish", "Message sudharo", "Apna message likho (Hinglish ya tooti-footi English chalegi) — app use polite, professional bana degi."),
    ("followup", "Recruiter ko follow-up", "Notes likho, jaise: Web Developer role, recruiter se kal shaam baat hui, status jaanna hai."),
    ("linkedin", "LinkedIn connect note", "Kisse judna hai aur kyon, jaise: Java developer hoon, aap backend team mein ho, seekhna chahta hoon."),
    ("call", "Follow-up call ki script", "Kis role ke liye call karna hai, jaise: Web Developer, recruiter se pichle hafte baat hui."),
];

#[component]
fn Writing() -> Element {
    let ctx = use_context::<Ctx>();
    let mut kind = use_signal(|| 0usize);
    let mut input = use_signal(String::new);
    let mut out = use_signal(String::new);
    let mut err = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let chat = llm::chat_ready();
    let (kid, _, hint) = WRITE_KINDS[*kind.read()];
    rsx! {
        div { class: "card",
            div { class: "chips",
                for (i, (_, label, _)) in WRITE_KINDS.iter().enumerate() {
                    button { key: "{i}", class: if *kind.read() == i { "chip on" } else { "chip" }, onclick: move |_| { kind.set(i); out.set(String::new()); err.set(String::new()); }, {crate::i18n::tr(label)} }
                }
            }
            textarea { class: "input area", rows: "6", placeholder: crate::i18n::tr(hint), value: "{input}", oninput: move |e| input.set(e.value()) }
            if !err.read().is_empty() { div { class: "card feedback bad", "{err}" } }
            div { class: "row-actions",
                button { class: "btn primary", disabled: !chat || *busy.read() || input.read().trim().len() < 8, onclick: move |_| {
                    let t = input.read().clone();
                    busy.set(true);
                    err.set(String::new());
                    spawn(async move {
                        let r = tokio::task::spawn_blocking(move || llm::write_help(kid, &t)).await;
                        match r {
                            Ok(Ok(o)) => out.set(o),
                            Ok(Err(e)) => err.set(e),
                            Err(e) => err.set(e.to_string()),
                        }
                        busy.set(false);
                    });
                }, if *busy.read() { {crate::i18n::tr("Likh raha hoon…")} } else { {crate::i18n::tr("Likho")} } }
            }
            if !out.read().is_empty() {
                div { class: "hintbox",
                    div { class: "muted sm", {crate::i18n::tr("Tayyar text (apne hisaab se badal lo, [Name] jagah naam likho):")} }
                    div { class: "writeout", "{out}" }
                    div { class: "row-actions",
                        Voice { ctx, text: out.read().clone() }
                        button { class: "btn", onclick: move |_| {
                            let js = format!("navigator.clipboard.writeText({});", serde_json::to_string(&*out.read()).unwrap_or_default());
                            let _ = document::eval(&js);
                        }, crate::icon::Ico { name: "copy".to_string(), size: 16 }, " ", {crate::i18n::tr("Copy")} }
                    }
                }
            }
        }
    }
}

/// Past mock interviews: a small table and a one-line trend.
#[component]
fn History() -> Element {
    let ctx = use_context::<Ctx>();
    let runs = ctx.state.read().coach_runs.clone();
    if runs.is_empty() {
        return rsx! {};
    }
    let last: Vec<CoachRun> = runs.iter().rev().take(6).cloned().collect();
    let trend = if runs.len() >= 2 {
        let (a, b) = (&runs[runs.len() - 2], &runs[runs.len() - 1]);
        if b.avg_words > a.avg_words + 4 {
            crate::i18n::trf("Pichli baar se jawab {} words lambe — accha!", &[(b.avg_words - a.avg_words).to_string()])
        } else if b.fillers + 2 <= a.fillers {
            crate::i18n::tr("Filler words kam hue — accha!")
        } else {
            crate::i18n::tr("Roz ek interview karo — 5 interview ke baad farak dikhega.")
        }
    } else {
        crate::i18n::tr("Pehla interview ho gaya. Ab roz ek karo — farak dikhega.")
    };
    rsx! {
        div { class: "card",
            div { class: "stage-t", crate::icon::Ico { name: "chart-column".to_string(), size: 22 }, " ", {crate::i18n::trf("Tumhari progress ({} interview)", &[runs.len().to_string()])} }
            p { class: "lp", "{trend}" }
            div { class: "tblwrap",
                table { class: "tbl",
                    thead { tr { th { {crate::i18n::tr("Tareekh")} } th { {crate::i18n::tr("Job")} } th { {crate::i18n::tr("Avg words")} } th { {crate::i18n::tr("Fillers")} } } }
                    tbody { for (k, r) in last.iter().enumerate() { tr { key: "{k}", td { "{r.date}" } td { "{r.role}" } td { "{r.avg_words}" } td { "{r.fillers}" } } } }
                }
            }
        }
    }
}
