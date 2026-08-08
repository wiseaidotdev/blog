use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Title() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let text_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    rsx! {
        div {
            class: "text-center",

            h2 {
                id: "testimonials-title",
                class: format!("text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold tracking-tight uppercase {}", text_color),

                span { class: "mx-4", "What" }
                span { class: "text-green-500", "Crabs" }
                br {}
                span { class: "mx-4", "say" }
                span { class: "text-green-500", "about" }
                span { class: "mx-4", "us 🦀" }
                span { "?" }
            }

            span {
                class: format!("block mt-4 text-sm uppercase {}", if is_light { "text-gray-500" } else { "text-gray-300" }),
                "Unfiltered thoughts from Rustaceans who definitely didn't unwrap() this by accident"
            }
        }
    }
}
