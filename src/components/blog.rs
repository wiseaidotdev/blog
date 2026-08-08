pub(crate) mod card;
pub(crate) mod code;
pub(crate) mod header;
pub(crate) mod title;

use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::card::BlogHomeCard;
use crate::components::blog::title::Title;
use crate::router::Route;
use std::collections::HashSet;

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Clone, PartialEq)]
struct PostData {
    title: String,
    category: String,
    slug: String,
    date: String,
    description: String,
    img: String,
    route: BlogRoute,
}

#[component]
pub fn Blog() -> Element {
    let mut cat = use_signal(|| None::<String>);
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let section_bg = if is_light {
        "bg-[var(--bg-secondary)]"
    } else {
        "bg-black"
    };

    let mut posts = use_signal(Vec::<PostData>::new);

    use_effect(move || {
        let mut all: Vec<PostData> = BlogRoute::static_routes()
            .into_iter()
            .rev()
            .filter(|r| !r.page().title.contains("[draft]"))
            .filter_map(|route| {
                let raw = route.page().title.clone();
                let parts: Vec<&str> = raw.splitn(8, " |---| ").collect();
                if let [_, title, category, slug, date, desc, img, ..] = parts.as_slice() {
                    Some(PostData {
                        title: title.to_string(),
                        category: category.to_string(),
                        slug: slug.to_string(),
                        date: date.to_string(),
                        description: desc.to_string(),
                        img: img.to_string(),
                        route,
                    })
                } else {
                    None
                }
            })
            .take(3)
            .collect();
        posts.set(all);
    });

    let all_categories: Vec<String> = {
        let mut seen = HashSet::new();
        BlogRoute::static_routes()
            .into_iter()
            .rev()
            .filter(|r| !r.page().title.contains("[draft]"))
            .filter_map(|route| {
                let raw = route.page().title.clone();
                let parts: Vec<&str> = raw.splitn(8, " |---| ").collect();
                if let [_, _, category, ..] = parts.as_slice() {
                    let c = category.to_string();
                    if seen.insert(c.clone()) {
                        Some(c)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    let display_posts = use_memo(move || {
        let selected = cat();
        posts()
            .into_iter()
            .filter(|p| selected.as_ref().map_or(true, |c| &p.category == c))
            .collect::<Vec<_>>()
    });

    rsx! {
        section {
            id: "blog",
            class: format!("flex flex-col items-center px-4 sm:px-8 md:px-16 py-16 min-h-screen justify-center transition-colors duration-300 {}", section_bg),

            Title {}

            div {
                class: "flex flex-row flex-wrap gap-2 mt-6 mb-8 justify-center items-center",

                button {
                    class: if cat().is_none() { "category-pill active" } else { "category-pill" },
                    onclick: move |_| cat.set(None),
                    "All"
                }

                for item in &all_categories {
                    button {
                        class: if Some(item.to_string()) == cat() { "category-pill active" } else { "category-pill" },
                        onclick: {
                            let item = item.clone();
                            move |_| cat.set(Some(item.clone()))
                        },
                        "{item}"
                    }
                }
            }

            div {
                class: "mb-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full max-w-6xl mx-auto",

                for post in display_posts() {
                    BlogHomeCard {
                        key: "{post.slug}",
                        title: post.title.clone(),
                        desc: post.description.clone(),
                        route: post.route,
                        img: Some(post.img.clone()),
                        created_at: post.date.clone(),
                        category: post.category.clone(),
                        slug: post.slug.clone(),
                    }
                }
            }

            div {
                class: "mt-10 flex justify-center",
                Link {
                    to: Route::Blogs {},
                    class: "inline-flex items-center gap-2 px-8 py-3 rounded-xl font-['Lexend'] font-semibold text-base transition-all duration-300 bg-green-600 text-white hover:bg-green-700 shadow-lg hover:shadow-xl hover:-translate-y-0.5",
                    i { class: "fa-solid fa-newspaper text-sm" }
                    "View All Blogs"
                    svg {
                        class: "w-4 h-4",
                        stroke_linejoin: "round",
                        stroke: "currentColor",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        path { d: "M5 12h14M12 5l7 7-7 7" }
                    }
                }
            }
        }
    }
}
