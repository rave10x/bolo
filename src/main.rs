mod audio;
mod data;
mod dialog;
mod i18n;
mod icon;
mod icon_data;
mod drill;
mod llm;
mod study;
mod talk;

use base64::Engine;
use data::*;
use dioxus::prelude::*;

const CSS: &str = include_str!("../assets/app.css");
const FONT: &[u8] = include_bytes!("../assets/onest.woff2");

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Today,
    Talk,
    Dialogues,
    Dialogue(String),
    Lessons,
    Lesson(String),
    Words,
    Topic(usize),
    Cards { ids: Vec<String>, review: bool, title: String },
    Speak,
    Pack(String),
    Practice,
    Drill(&'static str),
    Mistakes,
    Settings,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ctx {
    pub state: Signal<State>,
    pub route: Signal<Route>,
    pub sys_dark: Signal<bool>,
}

pub fn navigate(mut ctx: Ctx, r: Route) {
    audio::stop_all();
    ctx.route.set(r);
    let _ = document::eval("var s=document.getElementById('scroller'); if(s){s.scrollTop=0;}");
}

fn main() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("Bolo — English Practice")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1180.0, 820.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(820.0, 600.0));
    let cfg = dioxus::desktop::Config::new().with_window(window).with_menu(None).with_background_color((251, 247, 240, 255));
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

fn font_css() -> String {
    let b = base64::engine::general_purpose::STANDARD.encode(FONT);
    format!("@font-face{{font-family:'Onest';font-weight:100 900;font-display:swap;src:url(data:font/woff2;base64,{b}) format('woff2');}}")
}

#[component]
fn App() -> Element {
    let mut state = use_signal(|| {
        let mut st = State::load();
        if let Ok(t) = std::env::var("BOLO_THEME") {
            st.theme = t;
        }
        st
    });
    let route = use_signal(|| match std::env::var("BOLO_ROUTE").ok().as_deref() {
        Some("talk") => Route::Talk,
        Some("dialogues") => Route::Dialogues,
        Some(r) if r.starts_with("dialogue:") => Route::Dialogue(r[9..].to_string()),
        Some("lessons") => Route::Lessons,
        Some("words") => Route::Words,
        Some("speak") => Route::Speak,
        Some("practice") => Route::Practice,
        Some("mistakes") => Route::Mistakes,
        Some("settings") => Route::Settings,
        Some(r) if r.starts_with("lesson:") => Route::Lesson(r[7..].to_string()),
        Some(r) if r.starts_with("topic:") => Route::Topic(r[6..].parse().unwrap_or(0)),
        Some(r) if r.starts_with("pack:") => Route::Pack(r[5..].to_string()),
        Some(r) if r.starts_with("drill:") => Route::Drill(drill::DRILLS.iter().map(|d| d.0).chain(["mixed", "myfix"]).find(|k| *k == &r[6..]).unwrap_or("meaning")),
        Some("learn") => {
            let ids: Vec<String> = FLAT.iter().take(5).map(|f| f.id.clone()).collect();
            Route::Cards { ids, review: false, title: i18n::tr("Naye shabd").into() }
        }
        _ => Route::Today,
    });
    let sys_dark = use_signal(|| true);
    let ctx = use_context_provider(|| Ctx { state, route, sys_dark });

    // follow the system theme
    use_hook(move || {
        let mut sys = sys_dark;
        spawn(async move {
            let mut ev = document::eval("(function(){var m=window.matchMedia('(prefers-color-scheme: dark)'); dioxus.send(m.matches); m.addEventListener('change',function(e){dioxus.send(e.matches);});})();");
            while let Ok(v) = ev.recv::<bool>().await {
                sys.set(v);
            }
        });
    });
    use_effect(move || {
        let snap = state.read().clone();
        let _ = snap.save();
    });

    let lang_en = state.read().lang == "english";
    i18n::set_english(lang_en);
    let collapsed = state.read().side_collapsed;
    let dark = match state.read().theme.as_str() {
        "dark" => true,
        "light" => false,
        _ => *sys_dark.read(),
    };
    let fonts = use_hook(font_css);
    let css = format!("{fonts}{CSS}");
    let current = route.read().clone();
    let streak = state.read().streak();
    let due = state.read().due_ids().len();

    let nav: [(&str, &str, Route); 9] = [
        ("🏠", "Aaj · Today", Route::Today),
        ("💬", "Partner · Talk", Route::Talk),
        ("🎭", "Conversations", Route::Dialogues),
        ("📘", "Seekho · Lessons", Route::Lessons),
        ("🔤", "Shabd · Words", Route::Words),
        ("🎤", "Bolo · Speak", Route::Speak),
        ("🎯", "Abhyas · Practice", Route::Practice),
        ("🛠️", "Galtiyan · Mistakes", Route::Mistakes),
        ("⚙️", "Settings", Route::Settings),
    ];
    let section = |r: &Route| -> usize {
        match r {
            Route::Today => 0,
            Route::Talk => 1,
            Route::Dialogues | Route::Dialogue(_) => 2,
            Route::Lessons | Route::Lesson(_) => 3,
            Route::Words | Route::Topic(_) | Route::Cards { .. } => 4,
            Route::Speak | Route::Pack(_) => 5,
            Route::Practice | Route::Drill(_) => 6,
            Route::Mistakes => 7,
            Route::Settings => 8,
        }
    };
    let cur_sec = section(&current);

    rsx! {
        style { "{css}" }
        div { class: format!("app {}{}", if dark { "dark" } else { "light" }, if collapsed { " collapsed" } else { "" }),
            nav { class: "side",
                div { class: "brand", span { class: "logo", "B" } div { class: "bw", div { class: "bn", {i18n::tr("Bolo")} } div { class: "bt", "{i18n::tr(\"English seekho, aaram se\")}" } }
                    button { class: "side-toggle", title: if collapsed { i18n::tr("Sidebar kholo") } else { i18n::tr("Sidebar chhota karo") }, onclick: move |_| state.with_mut(|s| s.side_collapsed = !s.side_collapsed),
                        icon::Ico { name: if collapsed { "panel-left-open".to_string() } else { "panel-left-close".to_string() }, size: 20 } } }
                for (i, (ico, label, r)) in nav.iter().enumerate() {
                    {
                        let r2 = r.clone();
                        rsx! {
                            button { key: "{i}", title: "{i18n::tr(label)}", class: if i == cur_sec { "nav on" } else { "nav" }, onclick: move |_| navigate(ctx, r2.clone()),
                                span { class: "ni", icon::Ico { name: ico.to_string() } } span { class: "nl", "{i18n::tr(label)}" }
                                if i == 4 && due > 0 { span { class: "pill", "{due}" } }
                            }
                        }
                    }
                }
                div { class: "side-foot",
                    div { class: "streak", icon::Ico { name: "flame".to_string(), size: 20 }, span { class: "nl", " {streak} {i18n::tr(\"din\")}" } }
                    div { class: "muted sm nl", "{i18n::tr(\"Roz thoda — bas yahi kaafi hai\")}" }
                }
            }
            div { id: "scroller", class: "main", key: "{lang_en}",
                {match current {
                    Route::Today => rsx! { study::Today {} },
                    Route::Talk => rsx! { talk::Talk {} },
                    Route::Dialogues => rsx! { dialog::DialogueList {} },
                    Route::Dialogue(id) => rsx! { dialog::DialogueView { id } },
                    Route::Lessons => rsx! { study::LessonList {} },
                    Route::Lesson(id) => rsx! { study::LessonView { id } },
                    Route::Words => rsx! { study::WordTopics {} },
                    Route::Topic(i) => rsx! { study::TopicView { idx: i } },
                    Route::Cards { ids, review, title } => rsx! { study::Cards { ids, review, title } },
                    Route::Speak => rsx! { study::SpeakPacks {} },
                    Route::Pack(id) => rsx! { study::PackView { id } },
                    Route::Practice => rsx! { study::PracticeHub {} },
                    Route::Drill(k) => rsx! { study::DrillHost { kind: k } },
                    Route::Mistakes => rsx! { study::MistakeList {} },
                    Route::Settings => rsx! { study::Settings {} },
                }}
            }
        }
    }
}
