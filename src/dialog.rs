//! Conversations: listen to realistic workplace dialogues, learn the key phrases, then role-play your side.

use crate::data::*;
use crate::drill::score;
use crate::study::Voice;
use crate::{audio, llm, navigate, Ctx, Route};
use dioxus::prelude::*;

const CATS: [&str; 6] = ["Interview", "Call Interview", "Office", "HR & Career", "Client & Calls", "Team & Social"];

#[component]
pub fn DialogueList() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Conversations · Baat-cheet scripts")} }
            p { class: "sub", {crate::i18n::tr("Asli office, interview aur HR ki baat-cheet. Pehle suno, key phrases seekho, phir apna hissa khud bolo (role-play) — app tumhari awaaz sunkar bataega kitna sahi bola.")} }
            for cat in CATS.iter() {
                {
                    let items: Vec<Dialogue> = DIALOGUES.items.iter().filter(|d| d.cat == *cat).cloned().collect();
                    rsx! {
                        div { key: "{cat}",
                            div { class: "section-title", "{cat}" }
                            div { class: "grid",
                                for d in items.iter() {
                                    {
                                        let id = d.id.clone();
                                        let best = st.dialogs.get(&d.id).copied();
                                        rsx! {
                                            button { key: "{d.id}", class: "card topic", onclick: move |_| navigate(ctx, Route::Dialogue(id.clone())),
                                                div { class: "ticon", crate::icon::Ico { name: d.icon.clone(), size: 26 } }
                                                div { class: "tn", "{d.title}" }
                                                div { class: "muted sm", "{crate::i18n::tr(&d.hi)}" }
                                                div { class: "muted sm", "{d.lines.len()} lines" if let Some(b) = best { {crate::i18n::trf("  ·  role-play best {}%", &[(b).to_string()])} } }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Idle,
    Recording,
    Checking,
    Scored,
}

#[component]
pub fn DialogueView(id: String) -> Element {
    let ctx = use_context::<Ctx>();
    let mut roleplay = use_signal(|| std::env::var("BOLO_ROLEPLAY").is_ok());
    let Some(d) = DIALOGUES.items.iter().find(|d| d.id == id).cloned() else { return rsx! { div { class: "page", "?" } } };

    if *roleplay.read() {
        return rsx! { RolePlay { d: d.clone(), on_exit: move |_| roleplay.set(false) } };
    }

    let mut playing = use_signal(|| None::<usize>);
    let mut generation = use_signal(|| 0u32);
    let lines = d.lines.clone();
    let wpm = ctx.state.read().normal_wpm;
    let is_playing = playing.read().is_some();
    let cur = *playing.read();

    rsx! {
        div { class: "page narrow chat",
            button { class: "btn back", onclick: move |_| { generation += 1; audio::stop_all(); navigate(ctx, Route::Dialogues) }, {crate::i18n::tr("← Saari conversations")} }
            div { class: "kicker", "{d.cat}" }
            h1 { class: "title", crate::icon::Ico { name: d.icon.clone(), size: 28 }, " {d.title}" }
            p { class: "sub", {crate::i18n::trf("{}  ·  tum = Y, saamne wala = {}", &[crate::i18n::tr(&d.hi), (d.role).to_string()])} }
            div { class: "row-actions", style: "margin-top:0",
                if is_playing {
                    button { class: "btn", onclick: move |_| { generation += 1; audio::stop_all(); playing.set(None); }, crate::icon::Ico { name: "square".to_string(), size: 16 }, " Rokho" }
                } else {
                    button { class: "btn primary", onclick: move |_| {
                        generation += 1;
                        let my_gen = *generation.peek();
                        let ls = lines.clone();
                        spawn(async move {
                            for (i, l) in ls.iter().enumerate() {
                                if *generation.peek() != my_gen { return; }
                                playing.set(Some(i));
                                let (t, alt) = (l.text.clone(), l.who == "q");
                                let ok = tokio::task::spawn_blocking(move || audio::speak_wait(&t, wpm, alt)).await.unwrap_or(false);
                                if !ok || *generation.peek() != my_gen { break; }
                            }
                            if *generation.peek() == my_gen { playing.set(None); }
                        });
                    }, crate::icon::Ico { name: "play".to_string(), size: 16 }, " Poori baat-cheet suno" }
                }
                button { class: "btn", onclick: move |_| { generation += 1; audio::stop_all(); roleplay.set(true); }, crate::icon::Ico { name: "drama".to_string(), size: 18 }, " Role-play: apna hissa bolo" }
            }
            div { class: "dlg",
                for (i, l) in d.lines.iter().enumerate() {
                    div { key: "{i}", class: if l.who == "y" { "msgrow me" } else { "msgrow" },
                        div { class: "who", if l.who == "y" { {crate::i18n::tr("You")} } else { "{d.role}" } }
                        div { class: if cur == Some(i) { if l.who == "y" { "bubble me now" } else { "bubble ai now" } } else if l.who == "y" { "bubble me" } else { "bubble ai" },
                            "{l.text}"
                            Voice { ctx, text: l.text.clone() }
                        }
                    }
                }
            }
            div { class: "section-title", crate::icon::Ico { name: "key-round".to_string(), size: 18 }, " Key phrases" }
            div { class: "exlist",
                for (k, p) in d.keys.iter().enumerate() {
                    div { key: "{k}", class: "exrow", Voice { ctx, text: p.en.replace(" / ", ". ").replace('|', "") } div { div { class: "en", "{p.en}" } div { class: "hi", "{crate::i18n::tr(&p.hi)}" } } }
                }
            }
        }
    }
}

#[component]
fn RolePlay(d: Dialogue, on_exit: EventHandler<()>) -> Element {
    let mut ctx = use_context::<Ctx>();
    let mut at = use_signal(|| 0usize);
    let mut step = use_signal(|| Step::Idle);
    let mut shown = use_signal(|| true);
    let mut msg = use_signal(String::new);
    let mut last_pct = use_signal(|| 0u32);
    let mut scores = use_signal(Vec::<u32>::new);
    let stt = llm::stt_ready();
    let wpm = ctx.state.peek().normal_wpm;
    let lines = d.lines.clone();
    let n = lines.len();

    // the other person's lines are spoken automatically when we arrive at them
    let ls = lines.clone();
    use_effect(move || {
        let i = *at.read();
        if let Some(l) = ls.get(i) {
            if l.who == "q" {
                let _ = audio::speak_as(&l.text, wpm, true);
            }
        }
    });

    // save the best score once the role-play is finished (a hook, so it must run on every render)
    let (did, nn) = (d.id.clone(), n);
    use_effect(move || {
        if *at.read() >= nn {
            let sc = scores.peek().clone();
            let avg = if sc.is_empty() { 0 } else { sc.iter().sum::<u32>() / sc.len() as u32 };
            if avg > 0 {
                let did = did.clone();
                ctx.state.with_mut(|s| { let e = s.dialogs.entry(did).or_default(); *e = (*e).max(avg); });
            }
        }
    });

    if *at.read() >= n {
        let sc = scores.read().clone();
        let avg = if sc.is_empty() { 0 } else { sc.iter().sum::<u32>() / sc.len() as u32 };
        return rsx! {
            div { class: "page narrow",
                div { class: "card result",
                    div { class: "big", if stt { "{avg}%" } else { "✓" } }
                    div { class: "sub", if stt { {crate::i18n::tr("Tumhare bole hue sentences average itne match hue. 80% se upar bahut accha hai!")} } else { {crate::i18n::tr("Role-play poora hua. Speech tool install karoge to score bhi milega.")} } }
                    div { class: "row-actions",
                        button { class: "btn primary", onclick: move |_| { at.set(0); scores.set(vec![]); step.set(Step::Idle); msg.set(String::new()); }, {crate::i18n::tr("Phir se karo")} }
                        button { class: "btn", onclick: move |_| on_exit.call(()), {crate::i18n::tr("Wapas")} }
                    }
                }
            }
        };
    }

    let i = *at.read();
    let line = lines[i].clone();
    let st = *step.read();
    let progress = i as f32 * 100.0 / n as f32;
    // chat so far
    let past: Vec<DLine> = lines[..i].to_vec();

    rsx! {
        div { class: "page narrow chat",
            div { class: "drill-top",
                button { class: "btn back", onclick: move |_| { audio::stop_all(); on_exit.call(()) }, "← {d.title}" }
                span { class: "muted sm", "{i + 1} / {n}" }
            }
            div { class: "bar", i { style: "width:{progress}%" } }
            for (k, l) in past.iter().enumerate() {
                div { key: "{k}", class: if l.who == "y" { "msgrow me" } else { "msgrow" },
                    div { class: if l.who == "y" { "bubble me" } else { "bubble ai" }, "{l.text}" }
                }
            }
            if line.who == "q" {
                div { class: "msgrow",
                    div { class: "who", "{d.role}" }
                    div { class: "bubble ai now", "{line.text}" Voice { ctx, text: line.text.clone() } }
                }
                div { class: "row-actions", button { class: "btn primary", onclick: move |_| { audio::stop_all(); at += 1; step.set(Step::Idle); msg.set(String::new()); }, {crate::i18n::tr("Meri baari →")} } }
            } else {
                div { class: "yourturn",
                    div { class: "muted sm", {crate::i18n::tr("Ab tumhari baari — ye bolo:")} }
                    if *shown.read() {
                        div { class: "yline", "{line.text}" }
                    } else {
                        div { class: "yline blur", {crate::i18n::tr("Bina dekhe bolo (yaad se)")} }
                    }
                    div { class: "audio-row",
                        Voice { ctx, text: line.text.clone() }
                        button { class: "btn", onclick: move |_| { let v = *shown.peek(); shown.set(!v); }, if *shown.read() { crate::icon::Ico { name: "eye-off".to_string(), size: 18 }, " Chhupao" } else { crate::icon::Ico { name: "eye".to_string(), size: 18 }, " Dikhao" } }
                    }
                }
                if !msg.read().is_empty() { div { class: if *last_pct.read() >= 75 { "card feedback good" } else { "card feedback bad" }, "{msg}" } }
                div { class: "talkrow center",
                    if stt {
                        button { class: if st == Step::Recording { "micbtn rec" } else { "micbtn" }, disabled: matches!(st, Step::Checking),
                            onclick: {
                                let target = line.text.clone();
                                move |_| {
                                    if st == Step::Recording {
                                        audio::stop_recording();
                                        step.set(Step::Checking);
                                        let target = target.clone();
                                        spawn(async move {
                                            let r = tokio::task::spawn_blocking(|| llm::transcribe(&audio::rec_path("dlg"))).await;
                                            match r {
                                                Ok(Ok(heard)) => {
                                                    let (pct, missing) = score(&target, &heard);
                                                    last_pct.set(pct);
                                                    let tail = if missing.is_empty() { crate::i18n::tr("bilkul sahi!") } else { crate::i18n::trf("alag/chhoote: {}", &[missing.join(", ")]) };
                                                    msg.set(crate::i18n::trf("Maine suna: \"{}\" — {}% match · {}", &[heard.trim().to_string(), pct.to_string(), tail]));
                                                    step.set(Step::Scored);
                                                }
                                                Ok(Err(e)) => { msg.set(e); step.set(Step::Idle); }
                                                Err(e) => { msg.set(e.to_string()); step.set(Step::Idle); }
                                            }
                                        });
                                    } else if st != Step::Checking {
                                        audio::stop_all();
                                        msg.set(String::new());
                                        if audio::start_recording("dlg").is_ok() { step.set(Step::Recording); }
                                    }
                                }
                            },
                            if st == Step::Recording { crate::icon::Ico { name: "square".to_string(), size: 20 } } else { crate::icon::Ico { name: "mic".to_string(), size: 20 } } }
                    }
                    if st == Step::Scored || !stt {
                        button { class: "btn primary", onclick: move |_| {
                            let p = if stt { *last_pct.peek() } else { 100 };
                            scores.with_mut(|s| s.push(p));
                            ctx.state.with_mut(|s| s.day_mut().phrases += 1);
                            at += 1; step.set(Step::Idle); msg.set(String::new());
                        }, {crate::i18n::tr("Aage →")} }
                    }
                }
                div { class: "muted sm center", if st == Step::Recording { {crate::i18n::tr("Sun raha hoon… bol chuke to Stop dabao")} } else if st == Step::Checking { {crate::i18n::tr("Check kar raha hoon…")} } else if stt { {crate::i18n::tr("Mic dabao, upar wala sentence bolo")} } else { {crate::i18n::tr("Upar wala sentence bolo, phir Aage dabao")} } }
            }
        }
    }
}
