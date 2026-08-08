pub mod client;
pub mod partners;
pub mod title;

use crate::components::testimonials::client::Client;
use crate::components::testimonials::partners::Partners;
use crate::components::testimonials::title::Title;

use dioxus::prelude::*;
use serde::Deserialize;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ClientTestimonial {
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub testimonial: Option<String>,
}

#[component]
pub fn Testimonials() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let section_bg = if is_light { "bg-gray-50" } else { "bg-black" };
    let clients = vec![
        ClientTestimonial {
            name: "Ferris The Crab".to_string(),
            role: "Senior Borrow Checker".to_string(),
            testimonial: Some("Implemented 17 traits in one night. I don't even know what my code does anymore. But it compiles. Thanks!".to_string()),
        },
        ClientTestimonial {
            name: "Crabby Karen".to_string(),
            role: "AI Compiler Whisperer".to_string(),
            testimonial: Some("This Rust-based system is so fast, I benchmarked it against light itself. Light lost.".to_string()),
        },
        ClientTestimonial {
            name: "Borrow Ben".to_string(),
            role: "Fearless Concurrency Manager".to_string(),
            testimonial: Some("Deploying super agents in Rust? It's like `match`ing your soulmate with zero runtime panics.".to_string()),
        },
    ];

    let client_images = vec![
        (
            asset!("/assets/feature.webp"),
            Some(asset!("/assets/rustacean-1.png")),
        ),
        (
            asset!("/assets/feature.webp"),
            Some(asset!("/assets/rustacean-2.png")),
        ),
        (
            asset!("/assets/feature.webp"),
            Some(asset!("/assets/rustacean-3.png")),
        ),
    ];

    let client_elements = clients
        .into_iter()
        .enumerate()
        .map(|(index, client)| {
            rsx! {
                Client {
                    name: client.name,
                    role: client.role,
                    testimonial: client.testimonial,
                    image_url: client_images.get(index).map(|(img, _)| img.clone()).unwrap(),
                    overlay_url: client_images.get(index).and_then(|(_, overlay)| overlay.clone()),
                    is_active: index == 1
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        section {
            id: "testimonials",
            class: format!("min-h-screen w-full px-4 md:px-8 py-20 flex flex-col items-center gap-24 transition-colors duration-300 {}", section_bg),
            aria_labelledby: "testimonials-title",

            Title {}

            div {
                class: "flex flex-col gap-16 w-full items-center",

                div {
                    class: "flex flex-col md:flex-row flex-wrap justify-center gap-10 w-full",
                    {client_elements.into_iter()}
                }
            }

            div {
                class: "w-full flex justify-center",
                Partners {}
            }
        }
    }
}
