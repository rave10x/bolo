//! The exercise engine: questions are generated from words, phrases, mistakes and lesson quizzes.

use crate::data::*;
use crate::{audio, Ctx};
use dioxus::prelude::*;
use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

thread_local! { static SEED: Cell<u64> = const { Cell::new(0) }; }

pub fn rnd(n: usize) -> usize {
    SEED.with(|s| {
        let mut x = s.get();
        if x == 0 {
            x = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(88172645463325252) | 1;
        }
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        (x % n.max(1) as u64) as usize
    })
}

pub fn shuffle<T>(v: &mut [T]) {
    for i in (1..v.len()).rev() {
        v.swap(i, rnd(i + 1));
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Choice,
    Type,
    Tiles,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Q {
    pub kind: Kind,
    pub drill: &'static str,
    /// short instruction in Hinglish
    pub title: String,
    pub prompt: String,
    pub sub: String,
    pub audio: Option<String>,
    pub choices: Vec<String>,
    pub answer: String,
    pub tiles: Vec<String>,
    pub explain: String,
    pub word: Option<String>,
    pub mkey: Option<String>,
}

fn base(kind: Kind, drill: &'static str, title: &str, prompt: String, answer: String) -> Q {
    Q { kind, drill, title: title.into(), prompt, sub: String::new(), audio: None, choices: vec![], answer, tiles: vec![], explain: String::new(), word: None, mkey: None }
}

pub fn norm(s: &str) -> String {
    let lowered = s.to_lowercase().replace(['’', '‘'], "'");
    let kept: String = lowered.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '\'').collect();
    kept.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Compare what was heard with the target sentence: (percent matched, target words that were missing).
pub fn score(target: &str, heard: &str) -> (u32, Vec<String>) {
    let t: Vec<String> = canon(target).split(' ').filter(|w| !w.is_empty()).map(String::from).collect();
    let mut h: Vec<String> = canon(heard).split(' ').filter(|w| !w.is_empty()).map(String::from).collect();
    let mut missing = Vec::new();
    let mut hit = 0u32;
    for w in &t {
        if let Some(p) = h.iter().position(|x| x == w) {
            h.remove(p);
            hit += 1;
        } else {
            missing.push(w.clone());
        }
    }
    (hit * 100 / t.len().max(1) as u32, missing)
}

/// Like `norm`, but contractions are expanded so "It's" and "It is" count as the same.
pub fn canon(s: &str) -> String {
    let mut t = format!(" {} ", norm(s));
    for (a, b) in [("i'm", "i am"), ("it's", "it is"), ("that's", "that is"), ("there's", "there is"), ("he's", "he is"), ("she's", "she is"), ("we're", "we are"), ("they're", "they are"), ("you're", "you are"),
        ("i've", "i have"), ("we've", "we have"), ("i'll", "i will"), ("we'll", "we will"), ("i'd", "i would"), ("don't", "do not"), ("doesn't", "does not"), ("didn't", "did not"), ("can't", "cannot"),
        ("won't", "will not"), ("isn't", "is not"), ("aren't", "are not"), ("wasn't", "was not"), ("haven't", "have not"), ("let's", "let us"), ("login", "log in"), ("log-in", "log in")] {
        t = t.replace(&format!(" {a} "), &format!(" {b} "));
    }
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn choices_from(correct: &str, pool: Vec<String>) -> Vec<String> {
    let mut opts: Vec<String> = Vec::new();
    let mut pool = pool;
    shuffle(&mut pool);
    for p in pool {
        if p != correct && !opts.contains(&p) && opts.len() < 3 {
            opts.push(p);
        }
    }
    opts.push(correct.to_string());
    shuffle(&mut opts);
    opts
}

/// Distractors from the same topic first, then from anywhere.
fn distractors(fw: &FlatWord, f: impl Fn(&FlatWord) -> String) -> Vec<String> {
    let mut same: Vec<String> = FLAT.iter().filter(|o| o.topic == fw.topic && o.id != fw.id).map(&f).collect();
    shuffle(&mut same);
    same.truncate(6);
    let mut any: Vec<String> = (0..8).map(|_| f(&FLAT[rnd(FLAT.len())])).collect();
    same.append(&mut any);
    same
}

pub const DRILLS: [(&str, &str, &str, &str); 8] = [
    ("meaning", "🔤", "Matlab batao · Meaning", "English word dekho, Hindi matlab chuno"),
    ("reverse", "🔁", "English batao · Reverse", "Hindi matlab dekho, English word chuno"),
    ("listen", "👂", "Suno aur chuno · Listen", "Word suno, sahi word chuno"),
    ("spell", "⌨️", "Suno aur likho · Spell", "Word suno aur spelling type karo"),
    ("fill", "🧩", "Khali jagah · Fill the blank", "Sentence mein missing word likho"),
    ("build", "🧱", "Sentence banao · Build", "Hindi sentence ko English mein lagao"),
    ("mistakes", "🛠️", "Galti pakdo · Fix it", "Sahi sentence chuno (common galtiyan)"),
    ("myfix", "🗣️", "Meri galtiyan · My mistakes", "Baat-cheet mein jo sentence maine galat bole"),
];

pub fn make_word_q(drill: &'static str, fw: &FlatWord) -> Q {
    let w = fw.w;
    let id = Some(fw.id.clone());
    match drill {
        "meaning" => {
            let mut q = base(Kind::Choice, "meaning", "Iska Hindi matlab kya hai?", w.en.clone(), w.hi.clone());
            q.sub = w.pr.clone();
            q.audio = Some(w.en.clone());
            q.choices = choices_from(&w.hi, distractors(fw, |o| o.w.hi.clone()));
            q.explain = format!("{} = {}   ({})", w.en, w.hi, w.pr);
            q.word = id;
            q
        }
        "reverse" => {
            let mut q = base(Kind::Choice, "reverse", "Is matlab ka English word chuno", w.hi.clone(), w.en.clone());
            q.choices = choices_from(&w.en, distractors(fw, |o| o.w.en.clone()));
            q.explain = format!("{} = {}   ({})", w.en, w.hi, w.pr);
            q.word = id;
            q
        }
        "listen" => {
            let mut q = base(Kind::Choice, "listen", "Suno, kaun sa word bola gaya?", "volume-2".into(), w.en.clone());
            q.audio = Some(w.en.clone());
            q.choices = choices_from(&w.en, distractors(fw, |o| o.w.en.clone()));
            q.explain = format!("{} = {}   ({})", w.en, w.hi, w.pr);
            q.word = id;
            q
        }
        "spell" => {
            let mut q = base(Kind::Type, "spell", "Suno aur spelling likho", "volume-2".into(), w.en.clone());
            q.audio = Some(w.en.clone());
            q.sub = crate::i18n::trf("Hint: {}", &[w.hi.clone()]);
            q.explain = format!("{} = {}   ({})", w.en, w.hi, w.pr);
            q.word = id;
            q
        }
        "fill" => {
            let lower = w.ex.to_lowercase();
            let key = w.en.to_lowercase();
            let whole = |p: usize| {
                let before = lower[..p].chars().next_back().is_none_or(|c| !c.is_alphabetic());
                let after = lower[p + key.len()..].chars().next().is_none_or(|c| !c.is_alphabetic());
                before && after
            };
            let blanked = match lower.match_indices(&key).map(|(p, _)| p).find(|p| whole(*p)) {
                Some(p) => format!("{}_____{}", &w.ex[..p], &w.ex[p + key.len()..]),
                None => format!("{}  (____ = {})", w.ex, w.en),
            };
            let mut q = base(Kind::Type, "fill", "Khali jagah mein sahi word likho", blanked, w.en.clone());
            q.sub = format!("{}  ·  Hint: {}", w.exhi, w.hi);
            q.audio = Some(w.ex.clone());
            q.explain = format!("{}   —   {}", w.ex, w.exhi);
            q.word = id;
            q
        }
        _ => unreachable!(),
    }
}

pub fn make_build_q(en: &str, hi: &str) -> Q {
    let mut q = base(Kind::Tiles, "build", "Is Hindi sentence ko English mein banao", hi.to_string(), en.to_string());
    let mut tiles: Vec<String> = en.split_whitespace().map(|s| s.to_string()).collect();
    shuffle(&mut tiles);
    q.tiles = tiles;
    q.audio = Some(en.to_string());
    q.explain = format!("{en}   —   {hi}");
    q
}

pub fn make_mistake_q(m: &Mistake) -> Q {
    let mut q = base(Kind::Choice, "mistakes", "Kaun sa sentence sahi hai?", String::new(), m.right.clone());
    let mut opts = vec![m.right.clone(), m.wrong.clone()];
    shuffle(&mut opts);
    q.choices = opts;
    q.explain = m.why.clone();
    q.mkey = Some(m.wrong.clone());
    q
}

pub fn make_mcq_q(m: &Mcq) -> Q {
    let mut order: Vec<usize> = (0..m.opts.len()).collect();
    shuffle(&mut order);
    let mut q = base(Kind::Choice, "lesson", "Sahi jawab chuno", m.q.clone(), m.opts[m.ans].clone());
    q.choices = order.iter().map(|i| m.opts[*i].clone()).collect();
    q.explain = m.why.clone();
    q
}

/// Words to practise: everything the learner has met; a starter set if nothing yet.
pub fn pool(state: &State) -> Vec<usize> {
    let mut seen: Vec<usize> = FLAT.iter().enumerate().filter(|(_, f)| state.words.contains_key(&f.id)).map(|(i, _)| i).collect();
    if seen.len() < 8 {
        for i in 0..FLAT.len() {
            if !seen.contains(&i) {
                seen.push(i);
            }
            if seen.len() >= 24 {
                break;
            }
        }
    }
    seen
}

pub fn build_session(drill: &'static str, state: &State, n: usize) -> Vec<Q> {
    match drill {
        "build" => {
            let mut all: Vec<(String, String)> = PACKS.packs.iter().flat_map(|p| p.items.iter().map(|i| (i.en.clone(), i.hi.clone()))).filter(|(e, _)| e.split_whitespace().count() <= 9).collect();
            shuffle(&mut all);
            all.into_iter().take(n).map(|(e, h)| make_build_q(&e, &h)).collect()
        }
        "mistakes" => {
            let mut idx: Vec<usize> = (0..MISTAKES.items.len()).collect();
            shuffle(&mut idx);
            // sentences you got wrong before come first
            idx.sort_by_key(|i| std::cmp::Reverse(state.mistakes_wrong.get(&MISTAKES.items[*i].wrong).copied().unwrap_or(0)));
            let mut head: Vec<usize> = idx.iter().copied().take(n / 2).collect();
            let mut rest: Vec<usize> = idx.iter().copied().skip(n / 2).collect();
            shuffle(&mut rest);
            head.extend(rest);
            head.into_iter().take(n).map(|i| make_mistake_q(&MISTAKES.items[i])).collect()
        }
        "myfix" => {
            let mut v: Vec<&Fix> = state.my_fixes.iter().collect();
            shuffle(&mut v);
            v.into_iter().take(n).map(|f| {
                let m = Mistake { wrong: f.said.clone(), right: f.better.clone(), why: if f.tip.is_empty() { f.better.clone() } else { f.tip.clone() } };
                make_mistake_q(&m)
            }).collect()
        }
        "mixed" => {
            let kinds: [&'static str; 6] = ["meaning", "reverse", "listen", "spell", "fill", "build"];
            let mut out = Vec::new();
            let mut p = pool(state);
            shuffle(&mut p);
            for i in 0..n {
                let k = kinds[i % kinds.len()];
                if k == "build" {
                    out.extend(build_session("build", state, 1));
                } else {
                    let fw = &FLAT[p[i % p.len()]];
                    out.push(make_word_q(k, fw));
                }
            }
            out
        }
        _ => {
            let key: &'static str = DRILLS.iter().find(|d| d.0 == drill).map(|d| d.0).unwrap_or("meaning");
            let mut p = pool(state);
            // due words first
            p.sort_by_key(|i| if state.is_due(&FLAT[*i].id) { 0 } else { 1 });
            let due = p.iter().take_while(|i| state.is_due(&FLAT[**i].id)).count();
            let (mut a, mut b) = (p[..due].to_vec(), p[due..].to_vec());
            shuffle(&mut a);
            shuffle(&mut b);
            a.extend(b);
            a.into_iter().take(n).map(|i| make_word_q(key, &FLAT[i])).collect()
        }
    }
}

// ---------------------------------------------------------------- UI

#[derive(Clone, PartialEq)]
enum Verdict {
    Open,
    Right,
    Wrong(String),
}

/// Runs a list of questions. `lesson` (if set) records the score as the lesson's quiz result.
#[component]
pub fn Drill(qs: Vec<Q>, back_label: String, on_exit: EventHandler<()>, lesson: Option<String>) -> Element {
    let mut ctx = use_context::<Ctx>();
    let mut at = use_signal(|| 0usize);
    let mut right = use_signal(|| 0u32);
    let mut verdict = use_signal(|| Verdict::Open);
    let mut typed = use_signal(String::new);
    let mut chosen = use_signal(Vec::<usize>::new); // tile indexes in order
    let mut retry = use_signal(Vec::<Q>::new); // words answered wrong come once more at the end
    let mut finished = use_signal(|| false);
    let total_base = qs.len();

    let extra = retry.read().clone();
    let all: Vec<Q> = qs.iter().cloned().chain(extra.into_iter()).collect();
    let n = all.len();
    let wpm = ctx.state.read().normal_wpm;

    if *finished.read() || *at.read() >= n {
        let r = *right.read();
        let pct = if total_base == 0 { 0 } else { r * 100 / total_base as u32 };
        let msg = crate::i18n::tr(if pct >= 90 { "Shaandaar! Bahut badhiya" } else if pct >= 60 { "Accha kiya! Thoda aur practice" } else { "Koi baat nahi — galti se hi seekhte hain" });
        let _ = document::eval("setTimeout(function(){var s=document.getElementById('scroller'); if(s){s.scrollTop=0;}},30);");
        return rsx! {
            div { class: "page narrow",
                div { class: "card result",
                    div { class: "big", "{r} / {total_base}" }
                    div { class: "sub", "{msg}" }
                    div { class: "row-actions",
                        button { class: "btn primary", onclick: move |_| on_exit.call(()), "{crate::i18n::tr(&back_label)}" }
                    }
                }
            }
        };
    }

    let q = all[*at.read()].clone();
    let v = verdict.read().clone();
    let answered = v != Verdict::Open;
    let is_last = *at.read() + 1 >= n;
    let q_for_check = q.clone();
    let lesson_id = lesson.clone();

    // grade this question once
    let mut grade = move |q: &Q, ok: bool, given: String| {
        let drill = q.drill;
        let word = q.word.clone();
        let mkey = q.mkey.clone();
        ctx.state.with_mut(|st| {
            st.record_drill(drill, ok);
            if let Some(w) = &word {
                st.grade_word(w, ok);
            }
            if let Some(k) = &mkey {
                if !ok {
                    *st.mistakes_wrong.entry(k.clone()).or_default() += 1;
                }
            }
            st.day_mut().drills += 1;
        });
        if ok {
            if *at.peek() < total_base {
                right += 1;
            }
            verdict.set(Verdict::Right);
        } else {
            verdict.set(Verdict::Wrong(given));
            if q.word.is_some() && retry.read().len() < 6 {
                let mut again = q.clone();
                again.word = None; // only count the first try for the schedule
                retry.with_mut(|r| r.push(again));
            }
        }
    };

    let tile_text: Vec<String> = chosen.read().iter().map(|i| q.tiles[*i].clone()).collect();
    let progress = (*at.read() as f32 / n as f32) * 100.0;

    rsx! {
        div { class: "page narrow",
            div { class: "drill-top",
                button { class: "btn back", onclick: move |_| { audio::stop_all(); on_exit.call(()) }, "← {crate::i18n::tr(&back_label)}" }
                span { class: "muted", "{*at.read() + 1} / {n}   ·   ✓ {right}" }
            }
            div { class: "bar", i { style: "width:{progress}%" } }
            div { class: "card q-card",
                div { class: "q-title", "{crate::i18n::tr(&q.title)}" }
                if !q.prompt.is_empty() {
                    div { class: if q.kind == Kind::Tiles || q.prompt.chars().count() > 22 { "q-prompt long" } else { "q-prompt" }, if q.prompt == "volume-2" { crate::icon::Ico { name: "volume-2".to_string(), size: 44 } } else { "{crate::i18n::tr(&q.prompt)}" } }
                }
                if !q.sub.is_empty() { div { class: "q-sub", "{crate::i18n::tr(&q.sub)}" } }
                if let Some(a) = q.audio.clone() {
                    {
                        let a2 = a.clone();
                        rsx! {
                            div { class: "audio-row",
                                button { class: "btn", onclick: move |_| { let _ = audio::speak(&a, wpm); }, crate::icon::Ico { name: "volume-2".to_string(), size: 18 }, " Suno" }
                                button { class: "btn", onclick: move |_| { let _ = audio::speak(&a2, 90); }, crate::icon::Ico { name: "turtle".to_string(), size: 18 }, " Dheere" }
                            }
                        }
                    }
                }
            }

            if q.kind == Kind::Choice {
                div { class: "opts",
                    for (i, c) in q.choices.iter().enumerate() {
                        {
                            let cls = if !answered { "opt" } else if *c == q.answer { "opt right" } else if matches!(&v, Verdict::Wrong(g) if g == c) { "opt wrong" } else { "opt dim" };
                            let c2 = c.clone();
                            let qq = q_for_check.clone();
                            rsx! {
                                button { key: "{i}", class: "{cls}", disabled: answered,
                                    onclick: move |_| { let ok = c2 == qq.answer; grade(&qq, ok, c2.clone()); },
                                    "{c}"
                                }
                            }
                        }
                    }
                }
            }
            if q.kind == Kind::Type {
                {
                    let qq = q_for_check.clone();
                    let qq2 = q_for_check.clone();
                    rsx! {
                        div { class: "type-row",
                            input { class: "input big-in", value: "{typed}", disabled: answered, autofocus: true, placeholder: crate::i18n::tr("yahan likho · type here"),
                                oninput: move |e| typed.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter && !answered && !typed.read().trim().is_empty() {
                                        let g = typed.read().clone();
                                        let ok = norm(&g) == norm(&qq.answer);
                                        grade(&qq, ok, g);
                                    }
                                }
                            }
                            button { class: "btn primary", disabled: answered || typed.read().trim().is_empty(),
                                onclick: move |_| { let g = typed.read().clone(); let ok = norm(&g) == norm(&qq2.answer); grade(&qq2, ok, g); },
                                "Check ✓" }
                        }
                    }
                }
            }
            if q.kind == Kind::Tiles {
                {
                    let qq = q_for_check.clone();
                    rsx! {
                        div { class: "tile-answer",
                            if tile_text.is_empty() { span { class: "muted", {crate::i18n::tr("Neeche ke words par tap karke sentence banao")} } }
                            for (k, ti) in chosen.read().clone().into_iter().enumerate() {
                                button { key: "{k}", class: "tile on", disabled: answered, onclick: move |_| { chosen.with_mut(|c| c.retain(|x| *x != ti)); }, "{q.tiles[ti]}" }
                            }
                        }
                        div { class: "tile-bank",
                            for (ti, t) in q.tiles.iter().enumerate() {
                                button { key: "{ti}", class: if chosen.read().contains(&ti) { "tile used" } else { "tile" }, disabled: answered || chosen.read().contains(&ti),
                                    onclick: move |_| { chosen.with_mut(|c| c.push(ti)); }, "{t}" }
                            }
                        }
                        button { class: "btn primary", disabled: answered || chosen.read().is_empty(),
                            onclick: move |_| {
                                let g = chosen.read().iter().map(|i| qq.tiles[*i].clone()).collect::<Vec<_>>().join(" ");
                                let ok = norm(&g) == norm(&qq.answer);
                                grade(&qq, ok, g);
                            }, "Check ✓" }
                    }
                }
            }

            if answered {
                div { class: if v == Verdict::Right { "card feedback good" } else { "card feedback bad" },
                    div { class: "fb-title", if v == Verdict::Right { {crate::i18n::tr("✓ Sahi! Correct")} } else { {crate::i18n::trf("✗ Galat — sahi jawab:  {}", &[(q.answer).to_string()])} } }
                    if !q.explain.is_empty() { p { "{crate::i18n::tr(&q.explain)}" } }
                    div { class: "row-actions",
                        button { class: "btn primary", autofocus: true, onclick: move |_| {
                            audio::stop_all();
                            if is_last {
                                // lesson quiz result
                                if let Some(id) = lesson_id.clone() {
                                    let pct = *right.read() * 100 / total_base.max(1) as u32;
                                    ctx.state.with_mut(|st| {
                                        let best = st.lessons.get(&id).copied().unwrap_or(0).max(pct);
                                        let was_done = st.lessons.get(&id).is_some_and(|p| *p >= 60);
                                        st.lessons.insert(id, best);
                                        if best >= 60 && !was_done { st.day_mut().lessons += 1; }
                                    });
                                }
                                finished.set(true);
                            } else {
                                at += 1;
                                verdict.set(Verdict::Open);
                                typed.set(String::new());
                                chosen.set(vec![]);
                            }
                        }, if is_last { {crate::i18n::tr("Result dekho →")} } else { {crate::i18n::tr("Aage →")} } }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_drill_builds_answerable_questions() {
        let st = State::default();
        for kind in ["meaning", "reverse", "listen", "spell", "fill", "build", "mistakes", "mixed"] {
            let qs = build_session(kind, &st, 10);
            assert_eq!(qs.len(), 10, "{kind}");
            for q in qs {
                match q.kind {
                    Kind::Choice => {
                        assert!(q.choices.contains(&q.answer), "{kind}: answer missing from choices");
                        let mut c = q.choices.clone();
                        c.sort();
                        c.dedup();
                        assert_eq!(c.len(), q.choices.len(), "{kind}: duplicate choices {:?}", q.choices);
                    }
                    Kind::Tiles => assert_eq!(norm(&q.tiles.join(" ")).split(' ').count(), norm(&q.answer).split(' ').count()),
                    Kind::Type => assert!(!q.answer.is_empty()),
                }
            }
        }
    }

    #[test]
    fn all_words_make_every_question() {
        for fw in FLAT.iter() {
            for d in ["meaning", "reverse", "listen", "spell", "fill"] {
                let q = make_word_q(d, fw);
                if d == "fill" {
                    assert!(q.prompt.contains("_____"), "no blank for {}: {}", fw.id, q.prompt);
                }
            }
        }
    }

    #[test]
    fn lesson_quizzes_are_valid() {
        for l in &LESSONS.lessons {
            for m in &l.quiz {
                let q = make_mcq_q(m);
                assert!(q.choices.contains(&q.answer), "{}", l.id);
            }
        }
    }

    #[test]
    fn normalising_answers() {
        assert_eq!(norm("  Hello,  World! "), "hello world");
        assert_eq!(norm("It’s fine."), "it's fine");
    }
}

#[cfg(test)]
mod canon_tests {
    use super::*;
    #[test]
    fn contractions_and_login_match() {
        assert_eq!(canon("It's an expense tracker."), canon("It is an expense tracker"));
        assert_eq!(canon("the log-in page"), canon("the login page"));
        let (pct, missing) = score("I don't know the answer.", "I do not know the answer");
        assert_eq!((pct, missing.len()), (100, 0));
    }
}
