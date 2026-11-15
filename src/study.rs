//! Screens: Today, Lessons, Words, Speak, Practice, Mistakes, Settings.

use crate::data::*;
use crate::drill::{self, Drill, DRILLS};
use crate::{audio, llm, navigate, Ctx, Route};
use dioxus::prelude::*;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// **bold** and `code`-less inline markup for lesson text.
fn rich(s: &str) -> String {
    let e = esc(s);
    let mut out = String::new();
    for (i, part) in e.split("**").enumerate() {
        if i % 2 == 1 {
            out.push_str("<b>");
            out.push_str(part);
            out.push_str("</b>");
        } else {
            out.push_str(part);
        }
    }
    out.replace('\n', "<br>")
}

pub fn say(ctx: Ctx, text: &str, slow: bool) {
    let st = ctx.state.peek();
    let wpm = if slow { st.slow_wpm } else { st.normal_wpm };
    let _ = audio::speak(text, wpm);
}

/// Button + live log for installing the AI parts (voice, speech recognition, chat model).
#[component]
pub fn InstallAi() -> Element {
    let mut running = use_signal(llm::installing);
    let mut tail = use_signal(llm::install_tail);
    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let r = llm::installing();
            if *running.peek() != r { running.set(r); }
            let t = llm::install_tail();
            if *tail.peek() != t { tail.set(t); }
        }
    });
    let busy = *running.read();
    rsx! {
        div { class: "row-actions",
            button { class: "btn primary", disabled: busy, onclick: move |_| { llm::start_install(); running.set(true); },
                crate::icon::Ico { name: "download".to_string(), size: 18 }, " ",
                if busy { {crate::i18n::tr("Install ho raha hai… (kuch minute lagenge)")} } else { {crate::i18n::tr("AI install karo")} } }
        }
        if !tail.read().is_empty() { pre { class: "installlog", "{tail}" } }
    }
}

#[component]
pub fn Voice(ctx: Ctx, text: String) -> Element {
    let t2 = text.clone();
    rsx! {
        span { class: "voice",
            button { class: "ib", title: crate::i18n::tr("Suno"), onclick: move |_| say(ctx, &text, false), crate::icon::Ico { name: "volume-2".to_string(), size: 18 } }
            button { class: "ib", title: crate::i18n::tr("Dheere suno"), onclick: move |_| say(ctx, &t2, true), crate::icon::Ico { name: "turtle".to_string(), size: 18 } }
        }
    }
}

// ---------------------------------------------------------------- Today

#[component]
fn PlanRow(done: bool, icon: String, title: String, detail: String, cta: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        div { class: if done { "plan done" } else { "plan" },
            span { class: "plan-ico", if done { crate::icon::Ico { name: "check".to_string(), size: 24 } } else { crate::icon::Ico { name: icon.clone(), size: 24 } } }
            div { class: "plan-body", div { class: "plan-t", "{crate::i18n::tr(&title)}" } div { class: "muted", "{detail}" } }
            button { class: if done { "btn" } else { "btn primary" }, onclick: move |_| onclick.call(()), "{crate::i18n::tr(&cta)}" }
        }
    }
}

#[component]
pub fn Today() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let t = today();
    let log = st.days.get(&t).cloned().unwrap_or_default();
    let due = st.due_ids();
    let next_new: Vec<String> = FLAT.iter().filter(|f| !st.words.contains_key(&f.id)).take(5).map(|f| f.id.clone()).collect();
    let next_lesson = LESSONS.lessons.iter().find(|l| st.lessons.get(&l.id).is_none_or(|p| *p < 60)).cloned();
    let known = st.words.values().filter(|p| p.boxn >= 3).count();
    let seen = st.words.len();
    let lessons_done = st.lessons_done();
    let phrases_done = st.phrases.len();
    let phrase_total: usize = PACKS.packs.iter().map(|p| p.items.len()).sum();
    let streak = st.streak();
    let hour: u32 = chrono::Local::now().format("%H").to_string().parse().unwrap_or(9);
    let greet = if hour < 12 { "Suprabhat" } else if hour < 17 { "Namaste" } else { "Shubh sandhya" };
    let greet = crate::i18n::tr(greet);
    let days: Vec<(String, bool, bool)> = (0..7)
        .rev()
        .map(|i| {
            let d = add_days(&t, -i);
            let label = chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").map(|x| x.format("%a").to_string()).unwrap_or_default();
            (label, st.days.get(&d).is_some_and(|l| l.any()), i == 0)
        })
        .collect();

    let new_ids = next_new.clone();
    let due_ids = due.iter().take(15).cloned().collect::<Vec<_>>();
    let nl = next_lesson.clone();
    let pack_for_today = PACKS.packs.iter().find(|p| p.items.iter().enumerate().any(|(i, _)| !st.phrases.contains_key(&format!("{}:{}", p.id, i)))).map(|p| p.id.clone());

    rsx! {
        div { class: "page",
            h1 { class: "title", "{greet}!" }
            p { class: "sub", {crate::i18n::tr("Aaj ka chhota sa plan. Sabse zyada dhyan bolne par — roz 20–30 minute.")} }

            div { class: "stats",
                div { class: "stat", div { class: "n", crate::icon::Ico { name: "flame".to_string(), size: 26 }, " {streak}" } div { class: "l", {crate::i18n::tr("din ki streak")} } }
                div { class: "stat", div { class: "n", "{known}" } div { class: "l", {crate::i18n::trf("shabd pakke ({} dekhe / {})", &[seen.to_string(), FLAT.len().to_string()])} } }
                div { class: "stat", div { class: "n", "{lessons_done}/{LESSONS.lessons.len()}" } div { class: "l", {crate::i18n::tr("lessons poore")} } }
                div { class: "stat", div { class: "n", "{phrases_done}/{phrase_total}" } div { class: "l", {crate::i18n::tr("sentences bole")} } }
            }

            div { class: "week",
                for (i, (label, done, is_today)) in days.iter().enumerate() {
                    div { key: "{i}", class: if *done { "wd on" } else if *is_today { "wd today" } else { "wd" }, div { class: "dot", if *done { crate::icon::Ico { name: "check".to_string(), size: 18 } } }, div { class: "dl", "{label}" } }
                }
            }

            div { class: "section-title", {crate::i18n::tr("Aaj ka plan")} }
            div { class: "plans",
                PlanRow { done: log.talks >= 5, icon: "💬", title: crate::i18n::tr("Partner se 5 jawab bolo"),
                    detail: crate::i18n::trf("Aaj {} jawab bole — bolna hi sabse zaroori hai", &[log.talks.to_string()]), cta: "Baat karo",
                    onclick: move |_| navigate(ctx, Route::Talk) }
                PlanRow { done: st.coach_runs.iter().any(|r| r.date == t), icon: "💼", title: crate::i18n::tr("Ek mock interview karo"),
                    detail: crate::i18n::trf("Job description se apna interview banao — ab tak {} kiye", &[st.coach_runs.len().to_string()]), cta: "Coach",
                    onclick: move |_| navigate(ctx, Route::Coach) }
                {
                    let next_d = DIALOGUES.items.iter().find(|d| !st.dialogs.contains_key(&d.id)).cloned();
                    let any_done = !st.dialogs.is_empty();
                    match next_d {
                        Some(d) => { let id = d.id.clone(); rsx! { PlanRow { done: false, icon: "🎭", title: crate::i18n::tr("Ek conversation role-play karo"), detail: crate::i18n::trf("Agli: {} ({}){}", &[d.title.clone(), crate::i18n::tr(&d.cat), if any_done { String::new() } else { crate::i18n::tr(" — office/interview ki asli baat-cheet") }]), cta: "Suno aur bolo", onclick: move |_| navigate(ctx, Route::Dialogue(id.clone())) } } }
                        None => rsx! { PlanRow { done: true, icon: "🎭", title: "Saari conversations ho gayi ", detail: crate::i18n::tr("Ab Partner se naye topics par baat karo"), cta: "Dekho", onclick: move |_| navigate(ctx, Route::Dialogues) } },
                    }
                }
                PlanRow { done: log.new_words >= 5 || next_new.is_empty(), icon: "🔤", title: crate::i18n::tr("5 naye shabd seekho"),
                    detail: crate::i18n::trf("Aaj {} naye shabd seekhe", &[log.new_words.to_string()]), cta: if log.new_words >= 5 { {crate::i18n::tr("Aur seekho")} } else { {crate::i18n::tr("Shuru karo")} },
                    onclick: move |_| navigate(ctx, Route::Cards { ids: new_ids.clone(), review: false, title: crate::i18n::tr("Naye shabd").into() }) }
                PlanRow { done: due.is_empty(), icon: "🔁", title: crate::i18n::tr("Purane shabd review karo"),
                    detail: if due.is_empty() { crate::i18n::tr("Aaj kuch due nahi — bahut accha!") } else { crate::i18n::trf("{} shabd review ke liye ready", &[due.len().to_string()]) }, cta: "Review",
                    onclick: move |_| if !due_ids.is_empty() { navigate(ctx, Route::Cards { ids: due_ids.clone(), review: true, title: crate::i18n::tr("Review").into() }) } }
                if let Some(l) = nl {
                    {
                        let id = l.id.clone();
                        rsx! { PlanRow { done: log.lessons > 0, icon: "📘", title: crate::i18n::tr("Ek lesson padho"), detail: crate::i18n::trf("Agla: {} — {}", &[l.title.clone(), crate::i18n::tr(&l.hi)]), cta: "Padho", onclick: move |_| navigate(ctx, Route::Lesson(id.clone())) } }
                    }
                } else {
                    PlanRow { done: true, icon: "📘", title: "Saare lessons ho gaye ", detail: crate::i18n::tr("Ab practice aur speaking par dhyan do"), cta: "Dekho", onclick: move |_| navigate(ctx, Route::Lessons) }
                }
                PlanRow { done: log.phrases >= 3, icon: "🎤", title: crate::i18n::tr("3 sentences bolo"),
                    detail: crate::i18n::trf("Aaj {} sentences bole — suno, bolo, record karo", &[log.phrases.to_string()]), cta: "Bolo",
                    onclick: move |_| match pack_for_today.clone() { Some(p) => navigate(ctx, Route::Pack(p)), None => navigate(ctx, Route::Speak) } }
                PlanRow { done: log.drills >= 10, icon: "🎯", title: crate::i18n::tr("10 sawaal practice"),
                    detail: crate::i18n::trf("Aaj {} sawaal kiye", &[log.drills.to_string()]), cta: "Test do",
                    onclick: move |_| navigate(ctx, Route::Drill("mixed")) }
            }
        }
    }
}

// ---------------------------------------------------------------- Lessons

#[component]
pub fn LessonList() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Seekho · Lessons")} }
            p { class: "sub", {crate::i18n::tr("Upar Professional speaking (office, interview), neeche basics — jo aata hai use skip karo. Example sunne ke liye speaker button dabao, end mein chhota test.")} }
            for stage in LESSONS.stages.iter() {
                {
                    let ls: Vec<Lesson> = LESSONS.lessons.iter().filter(|l| l.stage == stage.id).cloned().collect();
                    let done = ls.iter().filter(|l| st.lessons.get(&l.id).is_some_and(|p| *p >= 60)).count();
                    rsx! {
                        div { key: "{stage.id}", class: "card stage",
                            div { class: "stage-head",
                                div { div { class: "stage-t", "{stage.name}" } div { class: "muted", "{crate::i18n::tr(&stage.hi)} — {crate::i18n::tr(&stage.desc)}" } }
                                span { class: "badge", "{done}/{ls.len()}" }
                            }
                            div { class: "lrows",
                                for l in ls.iter() {
                                    {
                                        let id = l.id.clone();
                                        let best = st.lessons.get(&l.id).copied();
                                        rsx! {
                                            button { key: "{l.id}", class: "lrow", onclick: move |_| navigate(ctx, Route::Lesson(id.clone())),
                                                span { class: match best { Some(p) if p >= 60 => "tick on", _ => "tick" }, if best.is_some_and(|p| p >= 60) { "✓" } }
                                                span { class: "lt", "{l.title}" }
                                                span { class: "muted", "{crate::i18n::tr(&l.hi)}" }
                                                if let Some(p) = best { span { class: "muted sm", "{p}%" } }
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

#[component]
pub fn LessonView(id: String) -> Element {
    let ctx = use_context::<Ctx>();
    let mut quiz = use_signal(|| None::<Vec<drill::Q>>);
    // developer hook: BOLO_AUTOQUIZ=1 opens the mini test straight away
    use_hook(|| if std::env::var("BOLO_AUTOQUIZ").is_ok() { if let Some(l) = LESSONS.lessons.iter().find(|l| l.id == id) { quiz.set(Some(l.quiz.iter().map(drill::make_mcq_q).collect())); } });
    let Some(idx) = LESSONS.lessons.iter().position(|l| l.id == id) else {
        return rsx! { div { class: "page", {crate::i18n::tr("Lesson nahi mila")} } };
    };
    let l = LESSONS.lessons[idx].clone();
    let best = ctx.state.read().lessons.get(&l.id).copied();
    let next = LESSONS.lessons.get(idx + 1).map(|n| n.id.clone());

    if let Some(qs) = quiz.read().clone() {
        let lid = l.id.clone();
        return rsx! {
            Drill { qs, back_label: format!("{}", l.title), lesson: Some(lid), on_exit: move |_| { quiz.set(None); let _ = document::eval("setTimeout(function(){var s=document.getElementById('scroller'); if(s){s.scrollTop=0;}},30);"); } }
        };
    }
    let l2 = l.clone();
    rsx! {
        div { class: "page narrow lesson",
            button { class: "btn back", onclick: move |_| navigate(ctx, Route::Lessons), {crate::i18n::tr("← Saare lessons")} }
            div { class: "kicker", "{LESSONS.stages.iter().find(|s| s.id == l.stage).map(|s| s.name.clone()).unwrap_or_default()}" }
            h1 { class: "title", "{l.title}" }
            p { class: "sub", "{crate::i18n::tr(&l.hi)}" }
            for (i, b) in l.blocks.iter().enumerate() {
                {
                    match b {
                        Block::P { x } => rsx! { p { key: "{i}", class: "lp", dangerous_inner_html: "{rich(&crate::i18n::tr(x))}" } },
                        Block::H { x } => rsx! { h2 { key: "{i}", class: "lh", "{crate::i18n::tr(x)}" } },
                        Block::Tip { x } => rsx! { div { key: "{i}", class: "tip", span { crate::icon::Ico { name: "lightbulb".to_string(), size: 20 } } div { dangerous_inner_html: "{rich(&crate::i18n::tr(x))}" } } },
                        Block::Ex { items } => rsx! {
                            div { key: "{i}", class: "exlist",
                                for (k, p) in items.iter().enumerate() {
                                    div { key: "{k}", class: "exrow",
                                        Voice { ctx, text: p.en.clone() }
                                        div { div { class: "en", "{p.en}" } div { class: "hi", "{p.hi}" } }
                                    }
                                }
                            }
                        },
                        Block::Tbl { head, rows } => rsx! {
                            div { key: "{i}", class: "tblwrap",
                                table { class: "tbl",
                                    thead { tr { for (k, h) in head.iter().enumerate() { th { key: "{k}", "{crate::i18n::tr(h)}" } } } }
                                    tbody { for (r, row) in rows.iter().enumerate() { tr { key: "{r}", for (k, c) in row.iter().enumerate() { td { key: "{k}", "{crate::i18n::tr(c)}" } } } } }
                                }
                            }
                        },
                        Block::Fix { wrong, right, why } => rsx! {
                            div { key: "{i}", class: "fixbox",
                                div { class: "fx bad", span { "✗" } span { class: "strike", "{wrong}" } }
                                div { class: "fx good", span { "✓" } span { "{right}" } Voice { ctx, text: right.clone() } }
                                div { class: "muted", "{crate::i18n::tr(why)}" }
                            }
                        },
                    }
                }
            }
            div { class: "card lesson-end",
                div { class: "stage-t", {crate::i18n::trf("Mini test ({} sawaal)", &[(l.quiz.len()).to_string()])} }
                div { class: "muted", if let Some(p) = best { {crate::i18n::trf("Best score: {}%  — 60% se upar matlab lesson poora ✓", &[(p).to_string()])} } else { {crate::i18n::tr("60% se upar aaya to lesson poora maana jayega")} } }
                div { class: "row-actions",
                    button { class: "btn primary", onclick: move |_| { let qs: Vec<drill::Q> = l2.quiz.iter().map(drill::make_mcq_q).collect(); quiz.set(Some(qs)); let _ = document::eval("setTimeout(function(){var s=document.getElementById('scroller'); if(s){s.scrollTop=0;}},30);"); }, {crate::i18n::tr("Test shuru karo")} }
                    if let Some(n) = next { button { class: "btn", onclick: move |_| navigate(ctx, Route::Lesson(n.clone())), {crate::i18n::tr("Agla lesson →")} } }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- Words

#[component]
pub fn WordTopics() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let due = st.due_ids();
    let due2 = due.iter().take(15).cloned().collect::<Vec<_>>();
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Shabd · Words")} }
            p { class: "sub", {crate::i18n::trf("{} zaroori shabd, topic ke hisaab se. Har shabd ke saath Hindi matlab, uchcharan aur example sentence.", &[(FLAT.len()).to_string()])} }
            div { class: "card review-box",
                div { div { class: "stage-t", crate::icon::Ico { name: "repeat".to_string(), size: 18 }, " Review" } div { class: "muted", if due.is_empty() { {crate::i18n::tr("Abhi koi shabd due nahi")} } else { {crate::i18n::trf("{} shabd dobara dekhne ka time hai", &[(due.len()).to_string()])} } } }
                button { class: "btn primary", disabled: due.is_empty(), onclick: move |_| navigate(ctx, Route::Cards { ids: due2.clone(), review: true, title: crate::i18n::tr("Review").into() }), {crate::i18n::tr("Review shuru karo")} }
            }
            div { class: "grid",
                for (i, t) in WORDS.topics.iter().enumerate() {
                    {
                        let seen = t.words.iter().filter(|w| st.words.contains_key(&format!("{}:{}", t.id, w.en.to_lowercase()))).count();
                        let pct = seen * 100 / t.words.len().max(1);
                        rsx! {
                            button { key: "{t.id}", class: "card topic", onclick: move |_| navigate(ctx, Route::Topic(i)),
                                div { class: "ticon", crate::icon::Ico { name: t.icon.clone(), size: 26 } }
                                div { class: "tn", "{t.name}" }
                                div { class: "muted sm", "{crate::i18n::tr(&t.hi)}" }
                                div { class: "bar", i { style: "width:{pct}%" } }
                                div { class: "muted sm", {crate::i18n::trf("{}/{} dekhe", &[(seen).to_string(), (t.words.len()).to_string()])} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TopicView(idx: usize) -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let Some(t) = WORDS.topics.get(idx) else { return rsx! { div { class: "page", "?" } } };
    let ids: Vec<String> = t.words.iter().map(|w| format!("{}:{}", t.id, w.en.to_lowercase())).collect();
    let new_ids: Vec<String> = ids.iter().filter(|i| !st.words.contains_key(*i)).take(8).cloned().collect();
    let due_ids: Vec<String> = ids.iter().filter(|i| st.is_due(i)).cloned().collect();
    let (n_new, n_due) = (new_ids.len(), due_ids.len());
    let title = t.name.clone();
    let title2 = t.name.clone();
    rsx! {
        div { class: "page",
            button { class: "btn back", onclick: move |_| navigate(ctx, Route::Words), {crate::i18n::tr("← Saare topics")} }
            h1 { class: "title", crate::icon::Ico { name: t.icon.clone(), size: 28 }, " {t.name}" }
            p { class: "sub", "{crate::i18n::tr(&t.hi)}" }
            div { class: "row-actions",
                button { class: "btn primary", disabled: n_new == 0, onclick: move |_| navigate(ctx, Route::Cards { ids: new_ids.clone(), review: false, title: title.clone() }),
                    if n_new == 0 { {crate::i18n::tr("Sab shabd dekh liye ✓")} } else { {crate::i18n::trf("Naye {} shabd seekho", &[(n_new).to_string()])} } }
                button { class: "btn", disabled: n_due == 0, onclick: move |_| navigate(ctx, Route::Cards { ids: due_ids.clone(), review: true, title: title2.clone() }), "Review ({n_due})" }
            }
            div { class: "wlist",
                for (k, w) in t.words.iter().enumerate() {
                    {
                        let wid = &ids[k];
                        let p = st.words.get(wid);
                        let status = match p { None => "new", Some(p) if p.boxn >= 3 => "known", Some(_) => "learning" };
                        rsx! {
                            div { key: "{k}", class: "wrow",
                                span { class: "wst {status}", title: "{status}" }
                                Voice { ctx, text: w.en.clone() }
                                div { class: "wmain", div { class: "wen", "{w.en} " span { class: "pr", "{w.pr}" } } div { class: "muted sm", "{w.ex}" } }
                                div { class: "whi", "{w.hi}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Cards(ids: Vec<String>, review: bool, title: String) -> Element {
    let mut ctx = use_context::<Ctx>();
    let mut at = use_signal(|| 0usize);
    let mut shown = use_signal(|| !review);
    let mut again = use_signal(Vec::<String>::new);
    let mut good = use_signal(|| 0usize);

    let ids_e = ids.clone();
    use_effect(move || {
        let i = *at.read();
        let q: Vec<String> = ids_e.iter().cloned().chain(again.peek().iter().cloned()).collect();
        if let Some(w) = q.get(i).and_then(|id| word_by_id(id)) {
            let wpm = ctx.state.peek().normal_wpm;
            let _ = audio::speak(&w.w.en, wpm);
        }
    });

    let queue: Vec<String> = ids.iter().cloned().chain(again.read().iter().cloned()).collect();
    let i = *at.read();
    let Some(fw) = queue.get(i).and_then(|id| word_by_id(id)) else {
        let g = *good.read();
        let n = ids.len();
        return rsx! {
            div { class: "page narrow",
                div { class: "card result",
                    div { class: "big", "{g}/{n}" }
                    div { class: "sub", {crate::i18n::trf("Pehli baar mein yaad: {} shabd. Baaki wapas aayenge — tension nahi!", &[(g).to_string()])} }
                    div { class: "row-actions",
                        button { class: "btn primary", onclick: move |_| navigate(ctx, Route::Drill("mixed")), {crate::i18n::tr("Ab chhota test do")} }
                        button { class: "btn", onclick: move |_| navigate(ctx, Route::Words), {crate::i18n::tr("Words par wapas")} }
                    }
                }
            }
        };
    };
    let w = fw.w;
    let wid = fw.id.clone();
    let wid2 = fw.id.clone();
    let topic = &WORDS.topics[fw.topic];
    let total = queue.len();
    let progress = i as f32 * 100.0 / total.max(1) as f32;
    let en = w.en.clone();
    let ex = w.ex.clone();
    let revealed = *shown.read();
    let first_pass = i < ids.len();

    rsx! {
        div { class: "page narrow",
            div { class: "drill-top",
                button { class: "btn back", onclick: move |_| navigate(ctx, Route::Words), "← {title}" }
                span { class: "muted", "{i + 1} / {total}" }
            }
            div { class: "bar", i { style: "width:{progress}%" } }
            div { class: "card flash",
                div { class: "muted sm", crate::icon::Ico { name: topic.icon.clone(), size: 16 }, " {topic.name}" }
                div { class: "fw", "{w.en}" }
                if revealed { div { class: "fpr", "{w.pr}" } }
                Voice { ctx, text: en }
                if revealed {
                    div { class: "fhi", "{w.hi}" }
                    div { class: "fex",
                        div { class: "exrow", Voice { ctx, text: ex } div { div { class: "en", "{w.ex}" } div { class: "hi", "{w.exhi}" } } }
                    }
                } else {
                    button { class: "btn primary", onclick: move |_| shown.set(true), {crate::i18n::tr("Matlab dikhao")} }
                }
            }
            if revealed {
                div { class: "row-actions center",
                    button { class: "btn bad", onclick: move |_| {
                        let id = wid.clone();
                        ctx.state.with_mut(|s| s.grade_word(&id, false));
                        if first_pass { again.with_mut(|a| a.push(id)); }
                        at += 1; shown.set(!review);
                    }, if review { {crate::i18n::tr("✗ Bhool gaya")} } else { {crate::i18n::tr("↻ Phir dikhana")} } }
                    button { class: "btn good", onclick: move |_| {
                        let id = wid2.clone();
                        ctx.state.with_mut(|s| s.grade_word(&id, true));
                        if first_pass { good += 1; }
                        at += 1; shown.set(!review);
                    }, if review { {crate::i18n::tr("✓ Yaad tha")} } else { {crate::i18n::tr("✓ Samajh gaya")} } }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- Speak

#[component]
pub fn SpeakPacks() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let ok_tools = audio::can_record();
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Bolo · Speak")} }
            p { class: "sub", {crate::i18n::tr("Bolna sabse zaroori hai. Tareeka: pehle suno → phir saath mein bolo → apni awaaz record karke suno.")} }
            if !ok_tools { div { class: "card feedback bad", {crate::i18n::tr("Recording ke liye pw-record / pw-play chahiye (PipeWire). Sunna phir bhi kaam karega.")} } }
            div { class: "grid",
                for p in PACKS.packs.iter() {
                    {
                        let id = p.id.clone();
                        let done = (0..p.items.len()).filter(|i| st.phrases.contains_key(&format!("{}:{}", p.id, i))).count();
                        let pct = done * 100 / p.items.len().max(1);
                        rsx! {
                            button { key: "{p.id}", class: "card topic", onclick: move |_| navigate(ctx, Route::Pack(id.clone())),
                                div { class: "ticon", crate::icon::Ico { name: p.icon.clone(), size: 26 } }
                                div { class: "tn", "{p.name}" }
                                div { class: "muted sm", "{crate::i18n::tr(&p.hi)}" }
                                div { class: "bar", i { style: "width:{pct}%" } }
                                div { class: "muted sm", {crate::i18n::trf("{}/{} bole", &[(done).to_string(), (p.items.len()).to_string()])} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PackView(id: String) -> Element {
    let mut ctx = use_context::<Ctx>();
    let mut rec = use_signal(|| None::<String>);
    let mut heard = use_signal(std::collections::HashMap::<String, String>::new);
    let mut busy = use_signal(|| None::<String>);
    let Some(p) = PACKS.packs.iter().find(|p| p.id == id).cloned() else { return rsx! { div { class: "page", "?" } } };
    let st = ctx.state.read().clone();
    let recording = rec.read().clone();
    let tip_html = rich(&crate::i18n::tr("Har sentence: **1)** suno  **2)** dheere suno aur saath bolo  **3)** record karo  **4)** apni awaaz suno aur compare karo  **5)** ✓ dabao."));
    rsx! {
        div { class: "page",
            button { class: "btn back", onclick: move |_| navigate(ctx, Route::Speak), {crate::i18n::tr("← Saare packs")} }
            h1 { class: "title", crate::icon::Ico { name: p.icon.clone(), size: 28 }, " {p.name}" }
            p { class: "sub", "{crate::i18n::tr(&p.hi)}" }
            div { class: "tip", span { crate::icon::Ico { name: "headphones".to_string(), size: 20 } } div { dangerous_inner_html: "{tip_html}" } }
            div { class: "speaklist",
                for (i, it) in p.items.iter().enumerate() {
                    {
                        let key = format!("{}:{}", p.id, i);
                        let count = st.phrases.get(&key).copied().unwrap_or(0);
                        let is_rec = recording.as_deref() == Some(key.as_str());
                        let has = audio::has_recording(&key);
                        let (k1, k2, k3, k4, k5) = (key.clone(), key.clone(), key.clone(), key.clone(), key.clone());
                        let target_en = it.en.clone();
                        rsx! {
                            div { key: "{key}", class: if count > 0 { "srow done" } else { "srow" },
                                div { class: "stext", div { class: "en", "{it.en}" } div { class: "hi", "{it.hi}" }
                                    if busy.read().as_deref() == Some(key.as_str()) { div { class: "heardline", {crate::i18n::tr("Sun raha hoon…")} } }
                                    else if let Some(m) = heard.read().get(&key) { div { class: "heardline", "{m}" } } }
                                div { class: "sctl",
                                    Voice { ctx, text: it.en.clone() }
                                    button { class: if is_rec { "ib rec" } else { "ib" }, title: crate::i18n::tr("Record"), onclick: move |_| {
                                        if is_rec { audio::stop_recording(); rec.set(None); }
                                        else if audio::start_recording(&k1).is_ok() { rec.set(Some(k1.clone())); }
                                    }, if is_rec { crate::icon::Ico { name: "square".to_string(), size: 16 } } else { crate::icon::Ico { name: "mic".to_string(), size: 16 } } }
                                    button { class: "ib", disabled: !has || is_rec, title: crate::i18n::tr("Meri awaaz"), onclick: move |_| { let _ = audio::play_recording(&k2); }, crate::icon::Ico { name: "play".to_string(), size: 16 } }
                                    if has && !is_rec && llm::stt_ready() {
                                        button { class: "ib", title: crate::i18n::tr("Check: kya sahi bola?"), disabled: busy.read().is_some(), onclick: move |_| {
                                            let key = k5.clone(); let target = target_en.clone();
                                            busy.set(Some(key.clone()));
                                            spawn(async move {
                                                let kk = key.clone();
                                                let r = tokio::task::spawn_blocking(move || llm::transcribe(&audio::rec_path(&kk))).await;
                                                let msg = match r { Ok(Ok(t)) => compare(&target, &t), Ok(Err(e)) => e, Err(e) => e.to_string() };
                                                heard.with_mut(|h| { h.insert(key, msg); });
                                                busy.set(None);
                                            });
                                        }, crate::icon::Ico { name: "search".to_string(), size: 16 } }
                                    }
                                    button { class: if count > 0 { "ib ok" } else { "ib" }, title: crate::i18n::tr("Maine bol liya"), onclick: move |_| {
                                        let k = k3.clone();
                                        if rec.read().is_some() { audio::stop_recording(); rec.set(None); }
                                        ctx.state.with_mut(|s| { *s.phrases.entry(k).or_default() += 1; s.day_mut().phrases += 1; });
                                    }, "✓" }
                                    if count > 0 { span { class: "muted sm", "×{count}" } }
                                    span { style: "display:none", "{k4}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- Practice / mistakes / settings

#[component]
pub fn PracticeHub() -> Element {
    let ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let total: u32 = st.drill_stats.values().map(|v| v.1).sum();
    let right: u32 = st.drill_stats.values().map(|v| v.0).sum();
    rsx! {
        div { class: "page",
            h1 { class: "title", {crate::i18n::tr("Abhyas · Practice")} }
            p { class: "sub", {crate::i18n::tr("Sawaal hal karne se hi English pakki hoti hai. Galat jawab par ghabrao mat — wahi se seekhte hain.")} }
            div { class: "card review-box",
                div { div { class: "stage-t", crate::icon::Ico { name: "target".to_string(), size: 18 }, " Aaj ka test (10 sawaal, sab mix)" } div { class: "muted", if total > 0 { {crate::i18n::trf("Ab tak {} sawaal, {}% sahi", &[(total).to_string(), (right * 100 / total.max(1)).to_string()])} } else { {crate::i18n::tr("Pehla test abhi baaki hai")} } } }
                button { class: "btn primary", onclick: move |_| navigate(ctx, Route::Drill("mixed")), {crate::i18n::tr("Shuru karo")} }
            }
            div { class: "grid",
                for (key, ico, name, desc) in DRILLS.iter() {
                    {
                        let k: &'static str = key;
                        let (r, t) = st.drill_stats.get(*key).copied().unwrap_or((0, 0));
                        rsx! {
                            button { key: "{key}", class: "card topic", onclick: move |_| navigate(ctx, Route::Drill(k)),
                                div { class: "ticon", crate::icon::Ico { name: ico.to_string(), size: 26 } }
                                div { class: "tn", "{crate::i18n::tr(name)}" }
                                div { class: "muted sm", "{crate::i18n::tr(desc)}" }
                                div { class: "muted sm", if t > 0 { {crate::i18n::trf("{}% sahi ({} sawaal)", &[(r * 100 / t).to_string(), (t).to_string()])} } else { {crate::i18n::tr("abhi tak nahi kiya")} } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DrillHost(kind: &'static str) -> Element {
    let ctx = use_context::<Ctx>();
    let qs = use_hook(|| drill::build_session(kind, &ctx.state.peek(), 10));
    if qs.is_empty() {
        return rsx! { div { class: "page narrow", button { class: "btn back", onclick: move |_| navigate(ctx, Route::Practice), {crate::i18n::tr("← Abhyas")} }
            div { class: "card", {crate::i18n::tr("Abhi yahan kuch nahi hai. Pehle Baat-cheet mein kuch jawab do — jo sentence sudharenge, wo yahan test banenge.")} }
            div { class: "row-actions", button { class: "btn primary", onclick: move |_| navigate(ctx, Route::Talk), {crate::i18n::tr("Baat-cheet par jao")} } } } };
    }
    rsx! { Drill { qs, back_label: "Abhyas".to_string(), lesson: None, on_exit: move |_| navigate(ctx, Route::Practice) } }
}

#[component]
pub fn MistakeList() -> Element {
    let ctx = use_context::<Ctx>();
    rsx! {
        div { class: "page narrow",
            h1 { class: "title", {crate::i18n::tr("Galtiyan · Common Mistakes")} }
            p { class: "sub", {crate::i18n::trf("Hindi bolne walon ki sabse common {} English galtiyan — sahi tareeka yaad kar lo.", &[(MISTAKES.items.len()).to_string()])} }
            div { class: "row-actions", button { class: "btn primary", onclick: move |_| navigate(ctx, Route::Drill("mistakes")), crate::icon::Ico { name: "wrench".to_string(), size: 18 }, " Inka test do" } }
            for (i, m) in MISTAKES.items.iter().enumerate() {
                div { key: "{i}", class: "fixbox",
                    div { class: "fx bad", span { "✗" } span { class: "strike", "{m.wrong}" } }
                    div { class: "fx good", span { "✓" } span { "{m.right}" } Voice { ctx, text: m.right.clone() } }
                    div { class: "muted", "{crate::i18n::tr(&m.why)}" }
                }
            }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    let mut ctx = use_context::<Ctx>();
    let st = ctx.state.read().clone();
    let mut msg = use_signal(String::new);
    let models = use_hook(llm::installed_models);
    let current_model = llm::model();
    let voice = audio::voice_name();
    let stt_txt = if llm::stt_ready() { "ready ✓" } else { "not installed (press Install AI)" };
    let chat_txt = if llm::chat_ready() { "ready ✓" } else { "not ready" };
    rsx! {
        div { class: "page narrow",
            h1 { class: "title", {crate::i18n::tr("Settings")} }
            div { class: "card set",
                div { class: "stage-t", crate::icon::Ico { name: "languages".to_string(), size: 20 }, " ", {crate::i18n::tr("Language · Bhasha")} }
                div { class: "chips",
                    for (k, l) in [("hinglish", "Hinglish"), ("english", "English")] {
                        button { key: "{k}", class: if st.lang == k { "chip on" } else { "chip" }, onclick: move |_| ctx.state.with_mut(|s| s.lang = k.to_string()), "{l}" }
                    }
                }
                div { class: "muted sm", {crate::i18n::tr("Hinglish = Hindi explanations in English letters. English = everything in plain English.")} }
            }
            div { class: "card set",
                div { class: "stage-t", crate::icon::Ico { name: "brain".to_string(), size: 20 }, " ", {crate::i18n::tr("Baat-cheet partner ka model")} }
                if models.is_empty() {
                    div { class: "muted sm", {crate::i18n::tr("ollama chalu nahi hai, isliye koi model nahi mila.")} }
                } else {
                    div { class: "chips",
                        for m in models.iter() {
                            { let m2 = m.clone(); let on = *m == current_model;
                              rsx! { button { key: "{m}", class: if on { "chip on" } else { "chip" }, onclick: move |_| ctx.state.with_mut(|s| s.chat_model = m2.clone()), "{m}" } } }
                        }
                    }
                    div { class: "muted sm", {crate::i18n::trf("Bade model (7B+) zyada smart jawab dete hain par slow hote hain. Default: {}. Instruct/chat model chuno, coder model nahi.", &[llm::DEFAULT_MODEL.to_string()])} }
                }
            }
            div { class: "card set",
                div { class: "stage-t", {crate::i18n::tr("Theme")} }
                div { class: "chips",
                    for (k, l) in [("auto", "Auto"), ("light", "Light"), ("dark", "Dark")] {
                        button { key: "{k}", class: if st.theme == k { "chip on" } else { "chip" }, onclick: move |_| ctx.state.with_mut(|s| s.theme = k.to_string()), "{crate::i18n::tr(l)}" }
                    }
                }
            }
            div { class: "card set",
                div { class: "stage-t", {crate::i18n::tr("Awaaz ki speed · Voice speed")} }
                div { class: "muted", {crate::i18n::tr("Normal")} }
                div { class: "chips",
                    for (v, l) in [(120u32, "Dheeri"), (150, "Normal"), (175, "Tez")] {
                        button { key: "{v}", class: if st.normal_wpm == v { "chip on" } else { "chip" }, onclick: move |_| ctx.state.with_mut(|s| s.normal_wpm = v), "{crate::i18n::tr(l)}" }
                    }
                }
                div { class: "muted", {crate::i18n::tr("Dheere")} }
                div { class: "chips",
                    for (v, l) in [(70u32, "Bahut dheere"), (95, "Dheere"), (110, "Thoda dheere")] {
                        button { key: "{v}", class: if st.slow_wpm == v { "chip on" } else { "chip" }, onclick: move |_| ctx.state.with_mut(|s| s.slow_wpm = v), "{crate::i18n::tr(l)}" }
                    }
                }
                div { class: "row-actions", button { class: "btn", onclick: move |_| { let s = ctx.state.peek(); let _ = audio::speak("Hello! This is how I sound. Let us learn English together.", s.normal_wpm); }, crate::icon::Ico { name: "volume-2".to_string(), size: 18 }, " Awaaz test karo" } }
                div { class: "muted sm", "Voice engine: {voice}" }
                div { class: "muted sm", "Speech recognition (Baat-cheet): {stt_txt}   ·   Chat model: {chat_txt}" }
            }
            div { class: "card set",
                div { class: "stage-t", crate::icon::Ico { name: "download".to_string(), size: 20 }, " ", {crate::i18n::tr("AI install / repair")} }
                div { class: "muted sm", {crate::i18n::tr("Voice, speech recognition and the chat model (about 2 GB, once). Safe to run again.")} }
                InstallAi {}
            }
            div { class: "card set",
                div { class: "stage-t", {crate::i18n::tr("Backup")} }
                div { class: "muted sm", {crate::i18n::trf("Progress yahan save hota hai: {}", &[(State::path().display()).to_string()])} }
                div { class: "row-actions",
                    button { class: "btn", onclick: move |_| {
                        let dir = crate::platform::documents_dir();
                        let _ = std::fs::create_dir_all(&dir);
                        let file = dir.join(format!("Bolo-backup-{}.json", today()));
                        match ctx.state.peek().save().and_then(|_| std::fs::copy(State::path(), &file).map(|_| ())) {
                            Ok(_) => msg.set(crate::i18n::trf("Backup ban gaya: {}", &[file.display().to_string()])),
                            Err(e) => msg.set(crate::i18n::trf("Backup fail: {}", &[e.to_string()])),
                        }
                    }, {crate::i18n::tr("Backup banao")} }
                }
                if !msg.read().is_empty() { div { class: "muted sm", "{msg}" } }
            }
        }
    }
}

/// How close was what the speech recogniser heard to the target sentence?
fn compare(target: &str, heard: &str) -> String {
    if heard.trim().is_empty() {
        return crate::i18n::tr("Kuch sunai nahi diya — dobara record karo.");
    }
    let (pct, missing) = drill::score(target, heard);
    let tail = if missing.is_empty() { format!(" {}", crate::i18n::tr("bilkul sahi!")) } else { format!(" · {}", crate::i18n::trf("alag/chhoote: {}", &[missing.join(", ")])) };
    crate::i18n::trf("Maine suna: \"{}\" — {}% match{}", &[heard.trim().to_string(), pct.to_string(), tail])
}
