use crate::components::blog::Blog;
use crate::components::features::Features;
use crate::components::footer::Footer;
use crate::components::team::Team;
use crate::components::testimonials::Testimonials;
use crate::header::Header;
use crate::hero::Hero;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Home() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let bg = if is_light {
        "bg-[var(--bg-primary)]"
    } else {
        "bg-black"
    };

    rsx! {
        div {
            class: format!("main-container min-h-screen transition-colors duration-300 {}", bg),
            Header {}
            main {
                id: "main-content",
                Hero {}
                Features {}
                Testimonials {}
                Team {}
                Blog {}
            }
            Footer {}
        }
    }
}
