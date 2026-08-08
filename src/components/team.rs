pub mod images;
pub mod title;

use crate::components::team::images::Images;
use crate::components::team::title::Title;

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Team() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let section_bg = if is_light {
        "bg-[var(--bg-secondary)]"
    } else {
        "bg-black"
    };

    rsx! {
        section {
            id: "team",
            class: format!("min-h-screen mb-[-6px] py-12 md:py-24 transition-colors duration-300 {}", section_bg),
            aria_labelledby: "team-title",

            Title {}

            div {
                class: "mt-16",
                Images {}
            }
        }
    }
}
