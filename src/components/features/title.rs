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
    let subtitle_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-300"
    };

    rsx! {
        div {
            class: "text-center",
            h2 {
                id: "features-title",
                class: "text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold tracking-tight uppercase",
                span { class: format!("mx-4 {}", text_color), "Discover" }
                span { class: "text-green-500", "The Future" }
                br {}
                span { class: format!("mx-4 {}", text_color), "of" }
                span { class: "text-green-500", "asi" }
                span { class: format!("mx-4 {}", text_color), "Research" }
            }
            span {
                class: format!("block mt-4 text-sm uppercase {}", subtitle_color),
                "Cutting-edge breakthroughs, one innovation at a time."
            }
        }
    }
}
