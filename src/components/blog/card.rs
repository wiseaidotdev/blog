use crate::blog::router_blog::BookRoute as BlogRoute;
use crate::router::Route;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct BlogHomeCardProps {
    pub title: String,
    pub route: BlogRoute,
    pub desc: String,
    pub img: Option<String>,
    pub created_at: String,
    pub category: String,
    pub slug: String,
}

#[component]
pub fn BlogHomeCard(props: BlogHomeCardProps) -> Element {
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

    let page_url = format!("https://wiseai.dev/blogs/{}", props.slug);
    let fb_url = format!("https://www.facebook.com/sharer/sharer.php?u={}", page_url);
    let x_url = format!(
        "https://twitter.com/intent/tweet?url={}&text={}",
        page_url,
        props.title.replace(' ', "%20")
    );
    let li_url = format!(
        "https://www.linkedin.com/sharing/share-offsite/?url={}",
        page_url
    );

    rsx! {
        div {
            class: "blog-card flex flex-col",

            if let Some(img_url) = &props.img {
                div { class: "relative overflow-hidden",
                    img {
                        src: "{img_url}",
                        alt: "{props.title}",
                        class: "w-full h-48 object-cover transition-transform duration-300 hover:scale-105",
                        loading: "lazy",
                    }
                    span {
                        class: format!("absolute top-3 left-3 text-xs font-semibold uppercase px-2 py-1 rounded-full tracking-wide {}", category_badge),
                        "{props.category}"
                    }
                }
            }

            div {
                class: "blog-card-body p-4 flex flex-col gap-2 flex-1",

                h2 {
                    class: format!("text-base font-bold leading-snug font-['Lexend'] line-clamp-2 {}", title_color),
                    "{props.title}"
                }

                p {
                    class: format!("text-sm leading-relaxed flex-1 line-clamp-3 {}", desc_color),
                    "{&props.desc}"
                }

                div {
                    class: "flex items-center justify-between mt-auto pt-2",
                    style: "border-top: 1px solid var(--border-color);",
                    span {
                        class: format!("text-xs flex items-center gap-1 {}", date_color),
                        i { class: "fa-regular fa-calendar text-xs" }
                        "{props.created_at}"
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
                            class: "inline-flex items-center gap-1 text-sm font-semibold text-green-500 hover:text-green-400 transition-colors duration-200",
                            to: Route::BlogPost { child: props.route },
                            "Read more"
                            ArrowRight {}
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn ArrowRight() -> Element {
    rsx! {
        svg {
            class: "w-3.5 h-3.5",
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
