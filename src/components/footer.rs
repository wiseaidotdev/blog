pub mod contacts;
pub mod footer;
pub mod services;
pub mod social;
pub mod subscribe;

use crate::components::footer::contacts::LocationContact;
use crate::components::footer::services::ServicesList;
use crate::components::footer::social::LogoSocial;
use crate::components::footer::subscribe::SubscribeForm;

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

    let services = vec![
        "AI Consulting".to_string(),
        "ML Training".to_string(),
        "Model Deployment".to_string(),
        "Edge AI Solutions".to_string(),
    ];

    rsx! {
        footer {
            class: format!("w-full py-16 transition-colors duration-300 {}", footer_bg),
            aria_labelledby: "footer-heading",

            h2 {
                id: "footer-heading",
                class: "sr-only",
                "Footer"
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 max-w-[1313.667px] mx-auto px-4 md:px-0 relative z-[220]",

                LogoSocial { is_light }

                LocationContact { is_light }

                ServicesList { services, is_light }

                SubscribeForm { is_light }
            }
        }
    }
}
