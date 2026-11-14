use crate::icon_data::{body, name_for};
use dioxus::prelude::*;

/// SVG icon. `name` may be an emoji key (from content files) or a Lucide name.
#[component]
pub fn Ico(name: String, #[props(default = 20)] size: u32) -> Element {
    let n = if name.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) { name.clone() } else { name_for(&name).to_string() };
    let inner = body(&n);
    rsx! {
        svg {
            class: "svgi", width: "{size}", height: "{size}", view_box: "0 0 24 24", fill: "none",
            stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            dangerous_inner_html: "{inner}",
        }
    }
}
