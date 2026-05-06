use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "home",
            class: "mt-10 min-h-screen text-center flex w-full pt-28 md:pt-32 flex-col gap-10 items-center relative mx-auto px-4 md:px-0",
            aria_labelledby: "hero-title",

            div {
                class: "w-full max-w-[1500px] h-auto md:h-[600px] shrink-0 relative z-10",

                div {
                    class: "flex w-full md:max-w-[1328px] flex-col gap-6 items-center relative mt-10 md:mt-20 mx-auto",

                    div {
                        class: "mt-20 w-full text-left md:text-center relative z-30",

                        h1 {
                            id: "hero-title",
                            class: "animate-gradient w-full font-['Lexend'] uppercase text-green-500 font-black tracking-tight leading-none text-3xl sm:text-4xl md:text-5xl lg:text-6xl z-30 whitespace-nowrap text-center",
                            "Real Super Intelligence"
                        }
                    }

                    div {
                        class: "mt-20 flex flex-row justify-center items-end gap-6 w-full relative z-30",

                        div {
                            class: "flex flex-col text-left gap-2 md:gap-3",
                            for word in "for By".split(" ") {
                                span {
                                    class: "text-white font-['Lexend'] text-2xl sm:text-3xl md:text-5xl lg:text-6xl font-medium lowercase",
                                    "{word}"
                                }
                            }
                        }

                        span {
                            class: "text-green-400 font-['Lexend'] text-4xl sm:text-6xl md:text-7xl lg:text-9xl font-black uppercase tracking-tight whitespace-nowrap",
                            "Rustaceans"
                        }
                    }

                    span {
                        class: "text-white mt-20 font-['Lexend'] text-xs sm:text-sm md:text-base lg:text-lg font-normal capitalize z-30 mt-2",
                        "Build Type Safe Super Agents Blazingly Fast!"
                    }
                }

                div {
                    class: "absolute inset-0 w-full h-full pointer-events-none z-[13]",

                    div {
                        img {
                            src: asset!("/assets/ver-line.svg"),
                            class: "absolute top-0 left-1/3 -translate-x-1/2 z-[17] gap-[500px]",
                            alt: ""
                        }
                        img {
                            src: asset!("/assets/ver-line.svg"),
                            class: "absolute top-0 left-2/3 -translate-x-1/2 z-[17] gap-[500px]",
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
                class: "flex w-full md:max-w-[606px] items-center justify-center flex-col md:flex-row gap-4 md:gap-10 z-40",

                div {
                    class: "w-full md:w-auto text-white md:text-left text-center font-['Lexend'] text-xs sm:text-sm md:text-base lg:text-lg leading-snug z-40",
                    span { class: "text-white capitalize", "Bold challenges need bold solutions" }
                }

                a {
                    href: "https://autogpt.wiseai.dev",
                    target: "_blank",
                    class: "flex items-center gap-2 bg-green-800 text-white font-bold py-2 px-6 rounded-lg hover:bg-green-900 transition-all duration-300 z-50 whitespace-nowrap",
                    aria_label: "let's build",

                    i {
                        class: "fa fa-cogs w-4 h-4",
                        alt: "CTA Icon",
                    }
                    "let's build"
                }
            }
        }
    }
}
