//! Embedded content (lessons, words, phrases, mistakes) and the saved progress.

use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::LazyLock;

#[derive(RustEmbed)]
#[folder = "content/"]
struct Content;

fn read(path: &str) -> String {
    Content::get(path).and_then(|f| String::from_utf8(f.data.into_owned()).ok()).unwrap_or_default()
}

// ---------------------------------------------------------------- content types

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Pair {
    pub en: String,
    pub hi: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "t")]
pub enum Block {
    #[serde(rename = "p")]
    P { x: String },
    #[serde(rename = "h")]
    H { x: String },
    #[serde(rename = "ex")]
    Ex { items: Vec<Pair> },
    #[serde(rename = "tbl")]
    Tbl { head: Vec<String>, rows: Vec<Vec<String>> },
    #[serde(rename = "tip")]
    Tip { x: String },
    #[serde(rename = "fix")]
    Fix { wrong: String, right: String, why: String },
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Mcq {
    pub q: String,
    pub opts: Vec<String>,
    pub ans: usize,
    pub why: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Lesson {
    pub id: String,
    pub stage: String,
    pub title: String,
    pub hi: String,
    pub blocks: Vec<Block>,
    pub quiz: Vec<Mcq>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub hi: String,
    pub desc: String,
}

#[derive(Deserialize, Debug)]
pub struct LessonFile {
    pub stages: Vec<Stage>,
    pub lessons: Vec<Lesson>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Word {
    pub en: String,
    pub hi: String,
    pub pr: String,
    pub ex: String,
    pub exhi: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub hi: String,
    pub icon: String,
    pub words: Vec<Word>,
}

#[derive(Deserialize, Debug)]
pub struct WordFile {
    pub topics: Vec<Topic>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub hi: String,
    pub icon: String,
    pub items: Vec<Pair>,
}

#[derive(Deserialize, Debug)]
pub struct PackFile {
    pub packs: Vec<Pack>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Mistake {
    pub wrong: String,
    pub right: String,
    pub why: String,
}

#[derive(Deserialize, Debug)]
pub struct MistakeFile {
    pub items: Vec<Mistake>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct DLine {
    pub who: String,
    pub text: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Dialogue {
    pub id: String,
    pub title: String,
    pub hi: String,
    pub icon: String,
    pub cat: String,
    pub role: String,
    pub lines: Vec<DLine>,
    pub keys: Vec<Pair>,
}

#[derive(Deserialize, Debug)]
pub struct DialogueFile {
    pub items: Vec<Dialogue>,
}

pub static DIALOGUES: LazyLock<DialogueFile> = LazyLock::new(|| serde_json::from_str(&read("dialogues.json")).expect("dialogues.json"));

pub static LESSONS: LazyLock<LessonFile> = LazyLock::new(|| serde_json::from_str(&read("lessons.json")).expect("lessons.json"));
pub static WORDS: LazyLock<WordFile> = LazyLock::new(|| serde_json::from_str(&read("words.json")).expect("words.json"));
pub static PACKS: LazyLock<PackFile> = LazyLock::new(|| serde_json::from_str(&read("phrases.json")).expect("phrases.json"));
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct CallFile {
    pub items: Vec<Mcq>,
}

pub static CALLS: LazyLock<CallFile> = LazyLock::new(|| serde_json::from_str(&read("calls.json")).expect("calls.json"));
pub static MISTAKES: LazyLock<MistakeFile> = LazyLock::new(|| serde_json::from_str(&read("mistakes.json")).expect("mistakes.json"));

/// Every word once, in learning order, with a stable id ("topic:word").
pub struct FlatWord {
    pub id: String,
    pub topic: usize,
    pub w: &'static Word,
}

pub static FLAT: LazyLock<Vec<FlatWord>> = LazyLock::new(|| {
    let mut out = Vec::new();
    for (ti, t) in WORDS.topics.iter().enumerate() {
        for w in &t.words {
            out.push(FlatWord { id: format!("{}:{}", t.id, w.en.to_lowercase()), topic: ti, w });
        }
    }
    out
});

pub fn word_by_id(id: &str) -> Option<&'static FlatWord> {
    FLAT.iter().find(|f| f.id == id)
}

// ---------------------------------------------------------------- progress

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct WordProg {
    pub boxn: u8,
    pub due: String,
    pub right: u32,
    pub wrong: u32,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct DayLog {
    pub talks: u32,
    pub new_words: u32,
    pub reviews: u32,
    pub phrases: u32,
    pub drills: u32,
    pub lessons: u32,
    /// words spoken to the Talk partner today, and filler words among them
    pub spoken_words: u32,
    pub fillers: u32,
}

impl DayLog {
    pub fn any(&self) -> bool {
        self.talks + self.new_words + self.reviews + self.phrases + self.drills + self.lessons > 0
    }
}

/// A sentence the learner said that was corrected in a conversation.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Fix {
    pub said: String,
    pub better: String,
    pub tip: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct State {
    pub words: HashMap<String, WordProg>,
    /// lesson id -> best quiz percent
    pub lessons: HashMap<String, u32>,
    /// phrase key -> times practised
    pub phrases: HashMap<String, u32>,
    pub days: BTreeMap<String, DayLog>,
    /// "auto" | "light" | "dark"
    pub theme: String,
    /// extra-slow speech for the turtle button, words per minute
    pub slow_wpm: u32,
    pub normal_wpm: u32,
    /// drills answered: (right, total) per drill kind
    pub drill_stats: HashMap<String, (u32, u32)>,
    pub mistakes_wrong: HashMap<String, u32>,
    pub my_fixes: Vec<Fix>,
    /// dialogue id -> best role-play match percent
    pub dialogs: HashMap<String, u32>,
    /// "hinglish" | "english" (interface language)
    pub lang: String,
    pub side_collapsed: bool,
    /// ollama model used by Talk; empty = default
    pub chat_model: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            words: HashMap::new(),
            lessons: HashMap::new(),
            phrases: HashMap::new(),
            days: BTreeMap::new(),
            theme: "auto".into(),
            slow_wpm: 95,
            normal_wpm: 150,
            drill_stats: HashMap::new(),
            mistakes_wrong: HashMap::new(),
            my_fixes: Vec::new(),
            dialogs: HashMap::new(),
            lang: "hinglish".into(),
            side_collapsed: false,
            chat_model: String::new(),
        }
    }
}

const INTERVALS: [i64; 6] = [0, 1, 3, 7, 21, 60];

pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn add_days(date: &str, n: i64) -> String {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| (d + chrono::Duration::days(n)).format("%Y-%m-%d").to_string())
        .unwrap_or_else(|_| date.to_string())
}

impl State {
    pub fn dir() -> PathBuf {
        let base = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("bolo")
    }
    pub fn path() -> PathBuf {
        Self::dir().join("state.json")
    }
    pub fn load() -> State {
        std::fs::read_to_string(Self::path()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
    }
    pub fn save(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(Self::dir())?;
        let tmp = Self::path().with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(self).map_err(std::io::Error::other)?)?;
        std::fs::rename(tmp, Self::path())
    }

    pub fn day_mut(&mut self) -> &mut DayLog {
        self.days.entry(today()).or_default()
    }

    /// Consecutive days with some practice, ending today (or yesterday if today is still empty).
    pub fn streak(&self) -> u32 {
        let t = today();
        let mut day = if self.days.get(&t).is_some_and(|d| d.any()) { t } else { add_days(&t, -1) };
        let mut n = 0;
        while self.days.get(&day).is_some_and(|d| d.any()) {
            n += 1;
            day = add_days(&day, -1);
        }
        n
    }

    /// Record one answer about a word. `first_time` words enter the system here.
    pub fn grade_word(&mut self, id: &str, known: bool) {
        let t = today();
        let new = !self.words.contains_key(id);
        let p = self.words.entry(id.to_string()).or_default();
        if known {
            p.right += 1;
            p.boxn = if new { 2 } else { (p.boxn + 1).min(5) };
        } else {
            p.wrong += 1;
            p.boxn = if new { 1 } else { 0 };
        }
        p.due = add_days(&t, INTERVALS[p.boxn as usize].max(if known { 1 } else { 0 }));
        let log = self.day_mut();
        if new {
            log.new_words += 1;
        } else {
            log.reviews += 1;
        }
    }

    pub fn is_due(&self, id: &str) -> bool {
        self.words.get(id).is_some_and(|p| p.due.as_str() <= today().as_str())
    }

    pub fn due_ids(&self) -> Vec<String> {
        let t = today();
        let mut v: Vec<(&String, &WordProg)> = self.words.iter().filter(|(_, p)| p.due.as_str() <= t.as_str()).collect();
        v.sort_by(|a, b| a.1.due.cmp(&b.1.due).then(a.0.cmp(b.0)));
        v.into_iter().map(|(k, _)| k.clone()).filter(|k| word_by_id(k).is_some()).collect()
    }

    pub fn record_drill(&mut self, kind: &str, ok: bool) {
        let e = self.drill_stats.entry(kind.to_string()).or_default();
        e.1 += 1;
        if ok {
            e.0 += 1;
        }
    }

    pub fn lessons_done(&self) -> usize {
        LESSONS.lessons.iter().filter(|l| self.lessons.get(&l.id).is_some_and(|p| *p >= 60)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_progress_files_still_load() {
        // a state saved before talks/my_fixes existed must not be thrown away
        let old = r#"{"words":{"basics:hello":{"boxn":2,"due":"2026-01-01","right":1,"wrong":0}},"lessons":{"l01":80},"days":{"2026-01-01":{"new_words":3}}}"#;
        let st: State = serde_json::from_str(old).expect("old state parses");
        assert_eq!(st.words.len(), 1);
        assert_eq!(st.lessons["l01"], 80);
        assert!(st.my_fixes.is_empty());
        assert_eq!(st.normal_wpm, 150);
    }

    #[test]
    fn dialogues_are_well_formed() {
        assert!(DIALOGUES.items.len() >= 30);
        for d in &DIALOGUES.items {
            assert_eq!(d.lines[0].who, "q", "{} should start with the partner", d.id);
            assert!(d.lines.iter().any(|l| l.who == "y"), "{}", d.id);
            assert!(!d.keys.is_empty(), "{}", d.id);
        }
        // every category used by the list screen exists
        for c in ["Interview", "Office", "HR & Career", "Client & Calls", "Team & Social"] {
            assert!(DIALOGUES.items.iter().any(|d| d.cat == c), "{c}");
        }
    }

    #[test]
    fn streak_counts_consecutive_days() {
        let mut st = State::default();
        for i in 0..3 {
            st.days.insert(add_days(&today(), -i), DayLog { talks: 1, ..Default::default() });
        }
        assert_eq!(st.streak(), 3);
        st.days.remove(&today());
        assert_eq!(st.streak(), 2); // today not done yet keeps yesterday's streak
    }

    #[test]
    fn grading_moves_words_through_boxes() {
        let mut st = State::default();
        st.grade_word("basics:hello", true);
        assert_eq!(st.words["basics:hello"].boxn, 2);
        st.grade_word("basics:hello", false);
        assert_eq!(st.words["basics:hello"].boxn, 0);
        assert!(st.is_due("basics:hello"));
    }
}
