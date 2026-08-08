use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Hero() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let text_primary = if is_light {
        "text-gray-900"
    } else {
        "text-white"
    };
    let text_secondary = if is_light {
        "text-gray-600"
    } else {
        "text-gray-200"
    };
    let section_bg = if is_light {
        "bg-[var(--bg-primary)]"
    } else {
        ""
    };

    rsx! {
        section {
            id: "home",
            class: format!(
                "mt-10 min-h-screen text-center flex w-full pt-28 md:pt-32 flex-col gap-10 \
                 items-center relative mx-auto overflow-x-hidden {}",
                section_bg
            ),
            aria_labelledby: "hero-title",

            div {
                class: "w-full max-w-[1500px] h-auto md:h-[600px] shrink-0 relative z-10 px-4",

                div {
                    class: "flex w-full md:max-w-[1328px] flex-col gap-6 items-center relative mt-10 md:mt-20 mx-auto",

                    div {
                        class: "mt-20 w-full text-center relative z-30",

                        h1 {
                            id: "hero-title",
                            class: "relative animate-gradient w-full font-['Lexend'] uppercase \
                                    text-green-500 font-black tracking-tight leading-tight \
                                    text-3xl sm:text-4xl md:text-5xl lg:text-6xl z-30 \
                                    break-words hyphens-auto text-center",
                            "Real Super Intelligence"
                        }
                    }

                    div {
                        class: "mt-20 flex flex-row justify-center items-end gap-4 md:gap-6 w-full relative z-30",

                        div {
                            class: "flex flex-col text-left gap-2 md:gap-3 shrink-0",
                            for word in "for By".split(' ') {
                                span {
                                    class: format!(
                                        "font-['Lexend'] text-2xl sm:text-3xl md:text-5xl lg:text-6xl \
                                         font-medium lowercase {}",
                                        text_primary
                                    ),
                                    "{word}"
                                }
                            }
                        }

                        span {
                            class: "text-green-400 font-['Lexend'] text-4xl sm:text-5xl md:text-7xl lg:text-9xl \
                                    font-black uppercase tracking-tight break-words text-center shrink min-w-0",
                            "Rustaceans"
                        }
                    }

                    span {
                        class: format!(
                            "mt-10 font-['Lexend'] text-xs sm:text-sm md:text-base lg:text-lg \
                             font-normal capitalize z-30 {}",
                            text_secondary
                        ),
                        "Build Type Safe Super Agents Blazingly Fast!"
                    }
                }

                div {
                    class: "absolute inset-0 w-full h-full pointer-events-none z-[13] hidden md:block",
                    div {
                        img {
                            src: asset!("/assets/ver-line.svg"),
                            class: "absolute top-0 left-1/3 -translate-x-1/2 z-[17]",
                            alt: ""
                        }
                        img {
                            src: asset!("/assets/ver-line.svg"),
                            class: "absolute top-0 left-2/3 -translate-x-1/2 z-[17]",
                            alt: ""
                        }
                    }
                    div {
                        img {
                            src: asset!("/assets/hor-line.svg"),
                            class: "absolute top-1/3 w-full h-[2px] z-[17]",
                            alt: ""
                        }
                        img {
                            src: asset!("/assets/hor-line.svg"),
                            class: "absolute top-2/3 w-full h-[2px] z-[17]",
                            alt: ""
                        }
                    }
                }
            }

            div {
                class: "flex w-full max-w-[606px] items-center justify-center flex-col md:flex-row \
                        gap-4 md:gap-10 z-40 px-4",

                div {
                    class: format!(
                        "w-full md:w-auto md:text-left text-center font-['Lexend'] \
                         text-xs sm:text-sm md:text-base lg:text-lg leading-snug z-40 {}",
                        text_secondary
                    ),
                    span { class: "capitalize", "Bold challenges need bold solutions" }
                }

                a {
                    href: "https://autogpt.wiseai.dev",
                    target: "_blank",
                    class: "flex items-center gap-2 bg-green-600 hover:bg-green-700 \
                            text-white font-bold py-2.5 px-7 rounded-xl transition-all \
                            duration-300 z-50 whitespace-nowrap shadow-lg text-sm shrink-0",
                    aria_label: "let's build",
                    i { class: "fa fa-cogs w-4 h-4" }
                    "let's build"
                }
            }
        }
    }
}
