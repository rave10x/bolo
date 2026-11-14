//! Interface language. Source strings are Hinglish; `content/en.tsv` maps each one to English.
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static EN: AtomicBool = AtomicBool::new(false);
static TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_english(on: bool) {
    EN.store(on, Ordering::Relaxed);
}
pub fn is_english() -> bool {
    EN.load(Ordering::Relaxed)
}

fn table() -> &'static HashMap<String, String> {
    TABLE.get_or_init(|| {
        let mut m = HashMap::new();
        for line in include_str!("../content/en.tsv").lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once('\t') {
                m.insert(k.trim().to_string(), v.trim().replace("\\n", "\n"));
            }
        }
        m
    })
}

/// Translate one source string (leading/trailing spaces are kept).
pub fn tr(s: &str) -> String {
    if !is_english() {
        return s.to_string();
    }
    let core = s.trim();
    let key = core.replace('\n', "\\n");
    match table().get(&key) {
        Some(v) => {
            let lead = &s[..s.len() - s.trim_start().len()];
            let trail = &s[s.trim_end().len()..];
            format!("{lead}{v}{trail}")
        }
        None => s.to_string(),
    }
}

/// Translate a template containing `{}` placeholders, then fill them in order.
pub fn trf(template: &str, args: &[String]) -> String {
    let mut out = tr(template);
    for a in args {
        if let Some(i) = out.find("{}") {
            out.replace_range(i..i + 2, a);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_and_falls_back() {
        set_english(true);
        assert_eq!(tr("Seekho · Lessons"), "Lessons");
        assert_eq!(tr(" Dheere"), " Slowly");
        assert_eq!(tr("not in the table"), "not in the table");
        assert_eq!(trf("Aaj {} sawaal kiye", &["4".into()]), "4 questions today");
        // lesson titles, stage text, dialogue and mistake notes must all be translated
        let mut missing = vec![];
        for l in &crate::data::LESSONS.lessons {
            if !table().contains_key(l.hi.trim()) { missing.push(l.hi.clone()); }
        }
        for d in &crate::data::DIALOGUES.items {
            if !table().contains_key(d.hi.trim()) { missing.push(d.hi.clone()); }
        }
        for m in &crate::data::MISTAKES.items {
            if !table().contains_key(m.why.trim()) { missing.push(m.why.clone()); }
        }
        for d in &crate::data::DIALOGUES.items {
            for k in &d.keys {
                if !table().contains_key(k.hi.trim()) { missing.push(k.hi.clone()); }
            }
        }
        for m in &crate::data::CALLS.items {
            if !table().contains_key(m.q.trim()) && m.q.chars().any(|c| c == '\u{27}') == false && !m.q.starts_with("Interviewer:") { missing.push(m.q.clone()); }
            if !table().contains_key(m.why.trim()) { missing.push(m.why.clone()); }
        }
        set_english(false);
        assert_eq!(tr("Seekho · Lessons"), "Seekho · Lessons");
        assert!(missing.is_empty(), "untranslated: {missing:?}");
    }
}
