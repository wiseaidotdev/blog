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
                id: "blog-title",
                class: format!("text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold tracking-tight uppercase {}", text_color),
                span { class: "mx-4", "Latest" }
                span { class: "text-green-500", "Insights" }
            }
            span {
                class: format!("block mt-4 text-sm uppercase {}", subtitle_color),
                "Explore our latest posts, expert tips, and updates on everything wise ai."
            }
        }
    }
}
