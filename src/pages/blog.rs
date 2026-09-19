use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::header::BlogHeader;
use crate::components::footer::Footer;
use crate::router::Route;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Clone, PartialEq)]
struct PostInfo {
    title: String,
    category: String,
    slug: String,
    date: String,
    description: String,
    img: String,
    dev_url: String,
    github_url: String,
}

#[component]
pub fn Blog() -> Element {
    let path: Route = use_route();
    let slug_from_url: String = path
        .to_string()
        .rsplitn(2, '/')
        .next()
        .unwrap_or("")
        .to_string();

    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let outer_bg = if is_light {
        "bg-[var(--bg-primary)]"
    } else {
        ""
    };
    let meta_text = if is_light {
        "text-gray-500"
    } else {
        "text-gray-400"
    };
    let author_name_color = if is_light {
        "text-gray-800"
    } else {
        "text-gray-100"
    };
    let title_color = if is_light {
        "text-gray-900"
    } else {
        "text-white"
    };
    let category_color = if is_light {
        "text-green-600"
    } else {
        "text-green-400"
    };

    let mut blog_info = use_signal(|| None::<PostInfo>);

    let blog_post = BlogRoute::static_routes().into_iter().rev().find(|route| {
        let raw_title = &route.page().title;
        if raw_title.contains("[draft]") {
            return false;
        }
        let clean_title = raw_title
            .replace(" |---| |---| ", " |---|  |---| ")
            .replace(" |---| |---| ", " |---|  |---| ");
        let items = clean_title.splitn(10, " |---| ").collect::<Vec<_>>();
        let [_, _, _, slug, ..] = items.as_slice() else {
            return false;
        };
        *slug == slug_from_url
    });

    if let Some(route) = blog_post {
        let raw_title = &route.page().title;
        let clean_title = raw_title
            .replace(" |---| |---| ", " |---|  |---| ")
            .replace(" |---| |---| ", " |---|  |---| ");
        let items = clean_title.splitn(10, " |---| ").collect::<Vec<_>>();
        if let [_id, title, category, slug, date, description, img, dev_url, github_url, ..] =
            items.as_slice()
        {
            blog_info.set(Some(PostInfo {
                title: title.to_string(),
                category: category.to_string(),
                slug: slug.to_string(),
                date: date.to_string(),
                description: description.to_string(),
                img: img.to_string(),
                dev_url: dev_url.trim().to_string(),
                github_url: github_url.trim().to_string(),
            }));
        }
    } else {
        blog_info.set(None);
    }

    let canonical_url = if let Some(ref info) = blog_info() {
        format!("https://wiseai.dev/blogs/{}", info.slug)
    } else {
        "https://wiseai.dev/blogs".to_string()
    };

    let page_title = if let Some(ref info) = blog_info() {
        format!("{} | Wise AI Blog", info.title)
    } else {
        "Wise AI Blog".to_string()
    };

    let page_description = if let Some(ref info) = blog_info() {
        info.description.clone()
    } else {
        "Explore Wise AI's latest insights on Rust, ASI, and advanced agent architectures."
            .to_string()
    };

    let og_image = if let Some(ref info) = blog_info() {
        format!("https://wiseai.dev/{}", info.img)
    } else {
        "https://wiseai.dev/assets/og-image.jpg".to_string()
    };

    rsx! {
        document::Title { "{page_title}" }
        document::Meta { name: "description", content: "{page_description}" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "author", content: "Mahmoud Harmouch" }
        document::Meta { property: "og:title", content: "{page_title}" }
        document::Meta { property: "og:description", content: "{page_description}" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:image", content: "{og_image}" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }
        document::Meta { property: "og:type", content: "article" }
        document::Meta { property: "og:site_name", content: "Wise AI Blog" }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta { property: "article:author", content: "Mahmoud Harmouch" }
        document::Meta { property: "article:section", content: "{blog_info().map(|i| i.category).unwrap_or_default()}" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{page_title}" }
        document::Meta { name: "twitter:description", content: "{page_description}" }
        document::Meta { name: "twitter:image", content: "{og_image}" }
        document::Meta { name: "twitter:site", content: "@wiseaidev" }
        document::Meta { name: "twitter:creator", content: "@wiseaidev" }
        document::Link { rel: "canonical", href: "{canonical_url}" }
        document::Script {
            r#type: "application/ld+json",
            dangerous_inner_html: {
                let title = page_title.clone();
                let desc = page_description.clone();
                let url = canonical_url.clone();
                let img = og_image.clone();
                format!(
                    r#"{{"@context":"https://schema.org","@type":"Article","headline":"{title}","description":"{desc}","url":"{url}","image":"{img}","author":{{"@type":"Person","name":"Mahmoud Harmouch","url":"https://github.com/wiseaidev"}},"publisher":{{"@type":"Organization","name":"Wise AI","url":"https://wiseai.dev","logo":{{"@type":"ImageObject","url":"https://wiseai.dev/assets/logo.png"}}}}}}"#
                )
            }
        }

        div {
            class: format!("min-h-screen transition-colors duration-300 {}", outer_bg),
            style: "padding-top: 72px; background: var(--bg-primary);",

            BlogHeader {}

            if let Some(post) = blog_info() {
                div { class: "w-full overflow-hidden",
                    style: "max-height: 480px;",
                    img {
                        src: "/{post.img}",
                        alt: "{post.title}",
                        class: "w-full object-cover",
                        style: "max-height: 480px;",
                        loading: "eager",
                    }
                }

                div {
                    class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8",

                    article {
                        class: "blog-article",

                        div {
                            class: "flex items-center gap-3 mb-4 mt-2",

                            img {
                                src: asset!("/assets/ceo.webp"),
                                class: "w-11 h-11 rounded-full ring-2 ring-green-500 shrink-0",
                                alt: "Mahmoud Harmouch",
                                loading: "lazy",
                            }
                            div { class: "flex flex-col",
                                span {
                                    class: format!("font-semibold text-sm font-['Lexend'] {}", author_name_color),
                                    "Mahmoud Harmouch"
                                }
                                span {
                                    class: format!("text-xs {}", meta_text),
                                    "{post.date}"
                                }
                            }
                        }

                        div {
                            class: "flex flex-wrap items-center gap-2 mb-5",

                            span {
                                class: "post-badge post-badge-ai",
                                title: "This post is AI-assisted. Core ideas and facts are human-authored and verified.",
                                i { class: "fa-solid fa-robot" }
                                "AI Assisted"
                            }

                            if !post.dev_url.is_empty() {
                                a {
                                    href: "{post.dev_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "post-badge post-badge-dev",
                                    i { class: "fa-brands fa-dev" }
                                    "Read on Dev"
                                }
                            }

                            if !post.github_url.is_empty() {
                                a {
                                    href: "{post.github_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "post-badge post-badge-github",
                                    i { class: "fa-brands fa-github" }
                                    "Edit on GitHub"
                                }
                            }
                        }

                        h1 {
                            class: format!("text-2xl sm:text-3xl md:text-4xl font-black font-['Lexend'] leading-tight mb-2 {}", title_color),
                            "{post.title}"
                        }

                        p {
                            class: format!("text-xs mb-6 font-mono {}", meta_text),
                            "#{post.slug}"
                        }

                        div {
                            class: "no-tailwind",
                            style: "max-width: 72ch; margin: 0 auto;",
                            Outlet::<Route> {}
                        }

                        div {
                            class: "mt-16 pt-8 flex items-center justify-between flex-wrap gap-4",
                            style: "border-top: 1px solid var(--border-color);",
                            Link {
                                to: Route::Blogs {},
                                class: "inline-flex items-center gap-2 px-5 py-2.5 rounded-lg font-['Lexend'] font-semibold text-sm transition-all duration-200 bg-green-600 text-white hover:bg-green-700",
                                i { class: "fa-solid fa-arrow-left text-xs" }
                                "All Blogs"
                            }
                            Link {
                                to: Route::Home {},
                                class: "inline-flex items-center gap-2 px-5 py-2.5 rounded-lg font-['Lexend'] font-semibold text-sm transition-all duration-200",
                                style: "background: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-color);",
                                i { class: "fa-solid fa-house text-xs" }
                                "Home"
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                    div {
                        class: "flex flex-col items-center justify-center py-32 gap-6",
                        i { class: format!("fa-solid fa-file-circle-question text-5xl {}", meta_text) }
                        p {
                            class: format!("text-lg font-['Lexend'] {}", meta_text),
                            "Loading post content..."
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
