mod blog;
mod components;
mod header;
mod hero;
mod pages;
mod router;
mod theme;

use crate::router::Route;
use crate::theme::WelcomeScreen;

use ::theme::dioxus::ThemeProvider;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use i18nrs::dioxus::I18nProvider;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

#[cfg(feature = "web")]
fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting client");
    dioxus::launch(App);
}

#[cfg(not(feature = "web"))]
#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting server");

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let router = axum::Router::new()
        .route(
            "/robots.txt",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/robots.txt").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "text/plain")],
                    content,
                )
            }),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/sitemap.xml").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "application/xml")],
                    content,
                )
            }),
        )
        .route(
            "/llms.txt",
            axum::routing::get(|| async {
                let content = std::fs::read_to_string("assets/llms.txt").unwrap_or_default();
                (
                    axum::http::StatusCode::OK,
                    [("content-type", "text/plain")],
                    content,
                )
            }),
        )
        .serve_dioxus_application(
            ServeConfig::builder()
                .incremental(IncrementalRendererConfig::new().static_dir("static"))
                .build()
                .unwrap(),
            App,
        )
        .into_make_service();
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
    // TODO
    let translations = HashMap::from([
        ("en", include_str!("../i18n/en/base.json")),
        ("es", include_str!("../i18n/es/base.json")),
        ("fr", include_str!("../i18n/fr/base.json")),
        ("ar", include_str!("../i18n/ar/base.json")),
    ]);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, maximum-scale=1" }
        document::Meta { name: "description", content: "Build type safe super agents blazingly fast with Rust" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "keywords", content: "Wise AI, asi, Rust, type safe super agents, blazingly fast, web application, LMM, large mathematical models" }
        document::Meta { name: "author", content: "Mahmoud Harmouch" }
        document::Meta { name: "copyright", content: "© 2026 Wise AI. All rights reserved." }
        document::Meta { name: "revisit-after", content: "7 days" }
        document::Meta { name: "language", content: "English" }
        document::Meta { name: "rating", content: "General" }
        document::Meta { name: "designer", content: "Mahmoud Harmouch" }
        document::Meta { name: "reply-to", content: "oss@wiseai.dev" }
        document::Meta { name: "target", content: "all" }
        document::Meta { name: "audience", content: "all" }
        document::Meta { name: "mobile-web-app-capable", content: "yes" }

        document::Meta { property: "og:title", content: "Wise AI - Build type safe super agents blazingly fast with Rust" }
        document::Meta { property: "og:description", content: "Build type safe super agents blazingly fast with Rust" }
        document::Meta { property: "og:image", content: "https://wiseai.dev/assets/og-image.jpg" }
        document::Meta { property: "og:url", content: "https://wiseai.dev/" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Wise AI | Build type safe super agents blazingly fast with Rust" }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "Wise AI - Build type safe super agents blazingly fast with Rust" }
        document::Meta { name: "twitter:description", content: "Build type safe super agents blazingly fast with Rust" }
        document::Meta { name: "twitter:image", content: "https://wiseai.dev/assets/og-image.jpg" }
        document::Meta { name: "twitter:site", content: "@wiseaidev" }
        document::Meta { name: "twitter:creator", content: "@wiseaidev" }
        document::Meta { name: "twitter:url", content: "https://wiseai.dev/" }

        document::Meta { name: "msapplication-TileColor", content: "#ffffff" }
        document::Meta { name: "msapplication-TileImage", content: "/assets/ms-icon-144x144.png" }
        document::Meta { name: "theme-color", content: "#ffffff" }
        document::Meta { name: "pinterest-rich-pin", content: "true" }

        document::Title { "Wise AI | Type Safe ASI" }

        document::Link { rel: "icon", r#type: "image/x-icon", href: "/assets/favicon.ico" }
        document::Link { rel: "apple-touch-icon", sizes: "57x57", href: "/assets/apple-icon-57x57.png" }
        document::Link { rel: "apple-touch-icon", sizes: "60x60", href: "/assets/apple-icon-60x60.png" }
        document::Link { rel: "apple-touch-icon", sizes: "72x72", href: "/assets/apple-icon-72x72.png" }
        document::Link { rel: "apple-touch-icon", sizes: "76x76", href: "/assets/apple-icon-76x76.png" }
        document::Link { rel: "apple-touch-icon", sizes: "114x114", href: "/assets/apple-icon-114x114.png" }
        document::Link { rel: "apple-touch-icon", sizes: "120x120", href: "/assets/apple-icon-120x120.png" }
        document::Link { rel: "apple-touch-icon", sizes: "144x144", href: "/assets/apple-icon-144x144.png" }
        document::Link { rel: "apple-touch-icon", sizes: "152x152", href: "/assets/apple-icon-152x152.png" }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/assets/apple-icon-180x180.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "192x192", href: "/assets/android-icon-192x192.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: "/assets/favicon-32x32.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "96x96", href: "/assets/favicon-96x96.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: "/assets/favicon-16x16.png" }

        document::Link { rel: "stylesheet", href: "/assets/fontawesome/fa-local.css" }
        document::Link { rel: "canonical", href: "https://wiseai.dev/" }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: "/assets/fonts/fonts.css" }

        ThemeProvider {
            I18nProvider {
                translations: translations.clone(),
                default_language: "en".to_string(),
                storage_name: "i18nrs".to_string(),
                WelcomeScreen {}
                Router::<Route> {}
            }
        }
    }
}
