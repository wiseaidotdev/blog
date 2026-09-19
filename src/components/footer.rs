pub mod contacts;
pub mod footer;
pub mod services;
pub mod social;
pub mod subscribe;

use crate::components::footer::contacts::LocationContact;
use crate::components::footer::social::LogoSocial;

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Footer() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let footer_bg = if is_light {
        "bg-gray-100 border-t border-gray-200"
    } else {
        "bg-[#0d0d0d]"
    };

    let bottom_bar_bg = if is_light {
        "bg-gray-200 border-t border-gray-300"
    } else {
        "bg-black border-t border-gray-800"
    };

    let bottom_text_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-500"
    };

    let legal_link_color = if is_light {
        "text-gray-500 hover:text-green-700 transition-colors duration-200"
    } else {
        "text-gray-500 hover:text-green-400 transition-colors duration-200"
    };

    rsx! {
        footer {
            class: format!("w-full transition-colors duration-300 {}", footer_bg),
            aria_labelledby: "footer-heading",

            h2 {
                id: "footer-heading",
                class: "sr-only",
                "Footer"
            }

            div {
                class: "max-w-[1313.667px] mx-auto px-6 md:px-10 py-16 relative z-[220]",

                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-10 items-start",

                    LogoSocial { is_light }

                    LocationContact { is_light, section: "location" }

                    LocationContact { is_light, section: "contact" }
                }
            }

            div {
                class: format!("w-full py-4 {}", bottom_bar_bg),

                div {
                    class: "max-w-[1313.667px] mx-auto px-4 flex flex-col sm:flex-row items-center justify-between gap-3",

                    span {
                        class: format!("text-xs font-['Lexend'] {}", bottom_text_color),
                        "© 2026 Wise AI. All rights reserved."
                    }

                    div {
                        class: "flex items-center gap-4",

                        a {
                            href: "/blogs/privacy-policy",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: format!("text-xs font-['Lexend'] {}", legal_link_color),
                            i { class: "fa-solid fa-shield-halved mr-1 text-xs" }
                            "Privacy Policy"
                        }

                        span {
                            class: format!("text-xs {}", bottom_text_color),
                            "·"
                        }

                        a {
                            href: "/blogs/terms-of-service",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: format!("text-xs font-['Lexend'] {}", legal_link_color),
                            i { class: "fa-solid fa-file-contract mr-1 text-xs" }
                            "Terms of Service"
                        }
                    }
                }
            }
        }
    }
}
