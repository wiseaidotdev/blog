pub(crate) mod grid;
pub(crate) mod item;

use crate::components::features::grid::Grid;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let section_bg = if is_light { "bg-gray-50" } else { "bg-gray-900" };
    let features = vec![
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-infinity",
            }},
            title: "Large Mathematical Models (LMMs)",
            description: "Go beyond text. Empower your agents to encode and simulate reality precisely using pure mathematical equations.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-bolt",
            }},
            title: "Blazing Fast Native Execution",
            description: "Built entirely in pure Rust, offering zero-cost abstractions and fearless concurrency for real-time ASI capabilities.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-brain",
            }},
            title: "Equation Discovery & Simulation",
            description: "Agents capable of autonomous symbolic regression and simulation to predict outcomes before they happen.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-network-wired",
            }},
            title: "Multi-Agent Architecture",
            description: "Deploy swarms of fault-tolerant agents that seamlessly collaborate to solve complex, multi-step problems.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-eye",
            }},
            title: "Inspired by the Eye of Horus",
            description: "A philosophy of deep perception and wholeness. Let your AI see the underlying mathematical structure of the universe.",
        },
        Feature {
            icon: rsx! {i {
                width: 30,
                height: 30,
                class: "text-4xl fa-solid fa-shield-halved",
            }},
            title: "Safe and Type-Driven",
            description: "Leverage Rust's uncompromising safety guarantees to build autonomous agents you can trust in production.",
        },
    ];

    rsx! {
        section {
            id: "features",
            class: format!("{} py-28 px-16 md:px-4 font-roboto flex min-h-screen justify-center transition-colors duration-300", section_bg),
            div { class: "",
                Grid { features: features, is_light: is_light }
            }
        }
    }
}
