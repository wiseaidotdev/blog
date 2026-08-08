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
            class: "flex flex-col gap-6 md:gap-8 items-center justify-center w-full px-6 md:px-0 h-[345px] text-center",

            h2 {
                id: "team-title",
                class: format!("font-['Lexend'] text-3xl sm:text-5xl md:text-6xl lg:text-8xl font-bold uppercase {}", text_color),
                span { class: "mx-2", "The minds" }
                span { class: "text-green-500", "behind" }
            }

            div {
                class: "flex flex-col md:flex-row gap-6 md:gap-12 items-center",
                div {
                    class: format!("text-xl md:text-5xl font-bold uppercase text-center md:text-right {}", text_color),
                    span { "Our" }
                    span { class: "mx-2 text-green-500", "creative" }
                    span { "team" }
                }

                a {
                    href: "https://github.com/wiseaidev",
                    target: "_blank",
                    class: "px-5 md:px-8 py-2 md:py-3 inline-flex items-center gap-2 text-white bg-green-500 rounded-full font-semibold shadow-md hover:bg-green-600 transition-colors duration-200 text-sm md:text-base",
                    i { class: "fas fa-arrow-right text-white text-base md:text-lg" }
                    span { "Meet the team" }
                }
            }
        }
    }
}
