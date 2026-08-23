use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Images() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let text_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-3 gap-6 place-items-center w-full py-8 px-4",
            aria_label: "Team member photos",

            div {
                class: "flex flex-col items-center gap-4 text-center",
                img {
                    src: asset!("/assets/team1.webp"),
                    class: "w-[290px] md:w-[310px] object-cover rounded-tl-[34px] rounded-bl-[34px]",
                    alt: "Team member 1"
                }
                p { class: format!("font-['Lexend'] text-xl md:text-2xl font-bold {}", text_color), "Frontend Engineer" }
            }

            div {
                class: "flex flex-col items-center gap-4 text-center",
                img {
                    src: asset!("/assets/team2.webp"),
                    class: "w-[320px] md:w-[350px] object-cover rounded-full",
                    alt: "Team member 2"
                }
                p { class: format!("font-['Lexend'] text-xl md:text-2xl font-bold {}", text_color), "Backend Engineer" }
            }

            div {
                class: "flex flex-col items-center gap-4 text-center",
                img {
                    src: asset!("/assets/team3.webp"),
                    class: "w-[290px] md:w-[310px] object-cover rounded-tr-[34px] rounded-bl-[34px]",
                    alt: "Team member 3"
                }
                p { class: format!("font-['Lexend'] text-xl md:text-2xl font-bold {}", text_color), "Devops Engineer" }
            }
        }
    }
}
