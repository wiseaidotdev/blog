use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::components::blog::card::ArrowRight;
use crate::components::blog::header::BlogHeader;
use crate::components::footer::Footer;
use crate::router::Route;
use std::collections::HashSet;

use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

fn parse_query_params() -> (String, Option<String>, usize) {
    #[cfg(target_arch = "wasm32")]
    {
        let search = web_sys::window()
            .unwrap()
            .location()
            .search()
            .unwrap_or_default();

        let query = search.trim_start_matches('?');
        let mut search_val = String::new();
        let mut category_val: Option<String> = None;
        let mut page_val = 1usize;

        for pair in query.split('&') {
            let mut kv = pair.splitn(2, '=');
            let key = kv.next().unwrap_or("").trim();
            let val = kv.next().unwrap_or("").trim();
            let val = js_sys::decode_uri_component(val)
                .map(|s| s.as_string().unwrap_or_default())
                .unwrap_or_else(|_| val.to_string());

            match key {
                "search" => search_val = val,
                "category" => {
                    if !val.is_empty() {
                        category_val = Some(val);
                    }
                }
                "page" => {
                    if let Ok(p) = val.parse::<usize>() {
                        page_val = p.max(1);
                    }
                }
                _ => {}
            }
        }

        (search_val, category_val, page_val)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        (String::new(), None, 1)
    }
}

fn push_query_state(search: &str, category: &Option<String>, page: usize) {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let history = window.history().unwrap();

        let mut parts: Vec<String> = vec![];
        if !search.is_empty() {
            let encoded = js_sys::encode_uri_component(search)
                .as_string()
                .unwrap_or_default();
            parts.push(format!("search={}", encoded));
        }
        if let Some(cat) = category {
            let encoded = js_sys::encode_uri_component(cat)
                .as_string()
                .unwrap_or_default();
            parts.push(format!("category={}", encoded));
        }
        if page > 1 {
            parts.push(format!("page={}", page));
        }

        let query = if parts.is_empty() {
            String::from("/blogs")
        } else {
            format!("/blogs?{}", parts.join("&"))
        };

        let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&query));
    }
}

#[component]
pub fn Blogs() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let (initial_search, initial_cat, initial_page) = parse_query_params();

    let mut page = use_signal(|| initial_page);
    let mut cat = use_signal(|| initial_cat);
    let mut search_query = use_signal(|| initial_search);

    let posts_per_page: usize = 6;

    let all_posts = use_memo(|| {
        BlogRoute::static_routes()
            .into_iter()
            .rev()
            .filter(|route| !route.page().title.contains("[draft]"))
            .collect::<Vec<_>>()
    });

    let filtered_posts = use_memo(move || {
        let query = search_query().to_lowercase();
        all_posts()
            .into_iter()
            .filter(|route| {
                let raw_title = &route.page().title;
                let items = raw_title.splitn(8, " |---| ").collect::<Vec<_>>();
                let [_, title, category, _, _, description, _, ..] = items.as_slice() else {
                    return false;
                };

                let matches_search = query.is_empty()
                    || title.to_lowercase().contains(&query)
                    || description.to_lowercase().contains(&query);

                let matches_cat = match cat() {
                    Some(ref selected) => category.trim() == selected.trim(),
                    None => true,
                };

                matches_search && matches_cat
            })
            .collect::<Vec<_>>()
    });

    let total_pages = use_memo(move || {
        let total = filtered_posts().len();
        (total as f64 / posts_per_page as f64).ceil() as usize
    });

    let paginated_posts = use_memo(move || {
        let posts = filtered_posts();
        let start = (page() - 1) * posts_per_page;
        let end = (start + posts_per_page).min(posts.len());
        if start < posts.len() {
            posts[start..end].to_vec()
        } else {
            vec![]
        }
    });

    let trending_posts = use_memo(move || {
        all_posts()
            .into_iter()
            .take(5)
            .filter_map(|route| {
                let raw_title = &route.page().title;
                let items = raw_title.splitn(8, " |---| ").collect::<Vec<_>>();
                if let [_, title, _, slug, _, _, img, ..] = items.as_slice() {
                    Some((title.to_string(), slug.to_string(), img.to_string()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    let mut unique_categories = HashSet::new();
    let category_items: Vec<String> = BlogRoute::static_routes()
        .into_iter()
        .rev()
        .filter(|route| !route.page().title.contains("[draft]"))
        .filter_map(|route| {
            let raw_title = &route.page().title;
            let items = raw_title.splitn(8, " |---| ").collect::<Vec<_>>();
            if let [_, _, category, ..] = items.as_slice() {
                let cat_str = category.to_string();
                if unique_categories.insert(cat_str.clone()) {
                    Some(cat_str)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let sidebar_bg = if is_light {
        "bg-white border border-gray-200"
    } else {
        "bg-gray-900 border border-gray-800"
    };
    let sidebar_heading_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };
    let trending_title_color = if is_light {
        "text-gray-900"
    } else {
        "text-gray-100"
    };
    let trending_item_hover = if is_light {
        "hover:bg-gray-50"
    } else {
        "hover:bg-gray-800"
    };
    let status_text_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-400"
    };
    let title_color = if is_light {
        "text-gray-900"
    } else {
        "text-white"
    };
    let empty_icon_color = if is_light {
        "text-gray-300"
    } else {
        "text-gray-700"
    };
    let empty_text_color = if is_light {
        "text-gray-400"
    } else {
        "text-gray-600"
    };

    let has_prev = page() > 1;
    let has_next = page() < total_pages();

    let post_count = filtered_posts().len();
    let status_text = if let Some(ref c) = cat() {
        format!(
            "{} - {} post{}",
            c,
            post_count,
            if post_count == 1 { "" } else { "s" }
        )
    } else if !search_query().is_empty() {
        format!(
            "{} result{} for \"{}\"",
            post_count,
            if post_count == 1 { "" } else { "s" },
            search_query()
        )
    } else {
        format!(
            "{} post{} available",
            post_count,
            if post_count == 1 { "" } else { "s" }
        )
    };

    rsx! {
        div {
            class: "blogs-page",
            style: "background: var(--bg-primary); min-height: 100vh; padding-top: 80px;",

            BlogHeader {}

            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",

                div {
                    class: "mb-6",
                    h1 {
                        class: format!("text-3xl sm:text-4xl font-black font-['Lexend'] uppercase {}", title_color),
                        span { class: "text-green-500", "Latest " }
                        "Blogs"
                    }
                    p {
                        class: format!("text-sm mt-1 {}", status_text_color),
                        "{status_text}"
                    }
                }

                div {
                    class: "blogs-layout",

                    div {
                        class: "blogs-main",

                        if filtered_posts().is_empty() {
                            div {
                                class: "flex flex-col items-center justify-center py-24 gap-4",
                                i { class: format!("fa-solid fa-newspaper text-5xl {}", empty_icon_color) }
                                p {
                                    class: format!("text-lg font-['Lexend'] {}", empty_text_color),
                                    "No posts found. Try adjusting your search or filter."
                                }
                            }
                        } else {
                            div {
                                class: "blog-grid mb-8",
                                for route in paginated_posts() {
                                    BlogPostItem { route }
                                }
                            }

                            div {
                                class: "flex justify-center gap-2 mt-4 flex-wrap",

                                if has_prev {
                                    button {
                                        class: "page-btn",
                                        onclick: {
                                            let cat_val = cat.clone();
                                            let search_val = search_query.clone();
                                            move |_| {
                                                let new_page = page() - 1;
                                                page.set(new_page);
                                                push_query_state(&search_val(), &cat_val(), new_page);
                                            }
                                        },
                                        "← Previous"
                                    }
                                }

                                for p in 1..=total_pages() {
                                    button {
                                        class: if page() == p { "page-btn active" } else { "page-btn" },
                                        onclick: {
                                            let cat_val = cat.clone();
                                            let search_val = search_query.clone();
                                            move |_| {
                                                page.set(p);
                                                push_query_state(&search_val(), &cat_val(), p);
                                            }
                                        },
                                        "{p}"
                                    }
                                }

                                if has_next {
                                    button {
                                        class: "page-btn",
                                        onclick: {
                                            let cat_val = cat.clone();
                                            let search_val = search_query.clone();
                                            move |_| {
                                                let new_page = page() + 1;
                                                page.set(new_page);
                                                push_query_state(&search_val(), &cat_val(), new_page);
                                            }
                                        },
                                        "Next →"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: format!("blogs-sidebar flex flex-col gap-6 p-5 rounded-xl {}", sidebar_bg),

                        div {
                            h2 {
                                class: format!("text-base font-bold font-['Lexend'] mb-3 flex items-center gap-2 {}", sidebar_heading_color),
                                i { class: "fa-solid fa-magnifying-glass text-green-500 text-sm" }
                                "Search"
                            }
                            input {
                                class: "search-input",
                                placeholder: "Search posts...",
                                value: "{search_query()}",
                                oninput: {
                                    let mut cat_inner = cat.clone();
                                    move |e: Event<FormData>| {
                                        let val = e.value();
                                        search_query.set(val.clone());
                                        cat_inner.set(None::<String>);
                                        page.set(1);
                                        push_query_state(&val, &None, 1);
                                    }
                                }
                            }
                        }

                        div {
                            h2 {
                                class: format!("text-base font-bold font-['Lexend'] mb-3 flex items-center gap-2 {}", sidebar_heading_color),
                                i { class: "fa-solid fa-tags text-green-500 text-sm" }
                                "Categories"
                            }
                            div {
                                class: "flex flex-col gap-1.5",

                                button {
                                    class: if cat().is_none() { "category-btn active" } else { "category-btn" },
                                    onclick: {
                                        let search_val = search_query.clone();
                                        move |_| {
                                            cat.set(None);
                                            page.set(1);
                                            push_query_state(&search_val(), &None, 1);
                                        }
                                    },
                                    i { class: "fa-solid fa-border-all mr-2 text-xs" }
                                    "All"
                                }

                                for item in &category_items {
                                    button {
                                        class: if Some(item.to_string()) == cat() { "category-btn active" } else { "category-btn" },
                                        onclick: {
                                            let item_clone = item.to_string();
                                            let search_val = search_query.clone();
                                            move |_| {
                                                cat.set(Some(item_clone.clone()));
                                                page.set(1);
                                                push_query_state(&search_val(), &Some(item_clone.clone()), 1);
                                            }
                                        },
                                        "{item}"
                                    }
                                }
                            }
                        }

                        div {
                            h2 {
                                class: format!("text-base font-bold font-['Lexend'] mb-3 flex items-center gap-2 {}", sidebar_heading_color),
                                i { class: "fa-solid fa-fire text-orange-500 text-sm" }
                                "Trending"
                            }
                            div {
                                class: "flex flex-col gap-1",
                                for post in trending_posts() {
                                    a {
                                        href: "/blogs/{post.1}",
                                        class: format!("flex items-start gap-3 p-2 rounded-lg transition-colors duration-200 {}", trending_item_hover),
                                        img {
                                            src: "{post.2}",
                                            alt: "{post.0}",
                                            class: "w-12 h-12 object-cover rounded-lg shrink-0",
                                            loading: "lazy",
                                        }
                                        span {
                                            class: format!("text-sm font-medium leading-snug pt-0.5 line-clamp-2 {}", trending_title_color),
                                            "{post.0}"
                                        }
                                    }
                                }
                            }
                        }

                    }
                }
            }

            Footer {}
        }
    }
}

#[component]
fn BlogPostItem(route: BlogRoute) -> Element {
    let raw_title = &route.page().title;

    if raw_title.contains("[draft]") {
        return rsx! {};
    }

    let items = raw_title.splitn(8, " |---| ").collect::<Vec<_>>();
    let [_, title, category, slug, date, description, img, ..] = items.as_slice() else {
        return rsx! {};
    };

    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    let title_color = if is_light {
        "text-gray-900"
    } else {
        "text-white"
    };
    let category_badge = if is_light {
        "text-green-700 bg-green-50 border border-green-200"
    } else {
        "text-green-400 bg-green-950 border border-green-900"
    };
    let desc_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-400"
    };
    let date_color = if is_light {
        "text-gray-400"
    } else {
        "text-gray-500"
    };
    let icon_color = if is_light {
        "text-gray-400 hover:text-gray-700"
    } else {
        "text-gray-500 hover:text-gray-300"
    };

    let page_url = format!("https://wiseai.dev/blogs/{}", slug);
    let fb_url = format!("https://www.facebook.com/sharer/sharer.php?u={}", page_url);
    let x_url = format!(
        "https://twitter.com/intent/tweet?url={}&text={}",
        page_url,
        title.replace(' ', "%20")
    );
    let li_url = format!(
        "https://www.linkedin.com/sharing/share-offsite/?url={}",
        page_url
    );

    rsx! {
        div {
            class: "blog-card flex flex-col",

            div { class: "relative overflow-hidden",
                img {
                    src: "{img}",
                    alt: "{title}",
                    class: "w-full h-48 object-cover transition-transform duration-300 hover:scale-105",
                    loading: "lazy",
                }
                span {
                    class: format!("absolute top-3 left-3 text-xs font-semibold uppercase px-2 py-1 rounded-full tracking-wide {}", category_badge),
                    "{category}"
                }
            }

            div {
                class: "blog-card-body p-4 flex flex-col gap-2 flex-1",

                h2 {
                    class: format!("text-base font-bold leading-snug font-['Lexend'] line-clamp-2 {}", title_color),
                    "{title}"
                }

                p {
                    class: format!("text-sm leading-relaxed flex-1 line-clamp-3 {}", desc_color),
                    "{description.chars().take(120).collect::<String>()}..."
                }

                div {
                    class: "flex items-center justify-between mt-auto pt-2",
                    style: "border-top: 1px solid var(--border-color);",

                    span {
                        class: format!("text-xs flex items-center gap-1 {}", date_color),
                        i { class: "fa-regular fa-calendar text-xs" }
                        "{date}"
                    }

                    div { class: "flex items-center gap-2",
                        a {
                            href: "{fb_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: format!("text-sm transition-colors duration-200 {}", icon_color),
                            title: "Share on Facebook",
                            i { class: "fa-brands fa-facebook" }
                        }
                        a {
                            href: "{x_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: format!("text-sm transition-colors duration-200 {}", icon_color),
                            title: "Share on X",
                            i { class: "fa-brands fa-x-twitter" }
                        }
                        a {
                            href: "{li_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: format!("text-sm transition-colors duration-200 {}", icon_color),
                            title: "Share on LinkedIn",
                            i { class: "fa-brands fa-linkedin" }
                        }
                        Link {
                            class: "inline-flex items-center gap-1 text-sm font-semibold text-green-500 hover:text-green-400 transition-colors duration-200 whitespace-nowrap",
                            to: Route::BlogPost { child: route },
                            "Read"
                            ArrowRight {}
                        }
                    }
                }
            }
        }
    }
}
