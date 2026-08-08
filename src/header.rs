use crate::theme::ThemeToggle;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

struct MenuItem {
    key: &'static str,
    icon_class: &'static str,
    label: &'static str,
}

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let mut is_scrolled = use_signal(|| false);
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);

    use_effect(move || {
        let window = web_sys::window().expect("no global `window` exists");
        let mut is_scrolled_inner = is_scrolled;

        let closure = Closure::wrap(Box::new(move || {
            let window = web_sys::window().expect("no global `window` exists");
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            is_scrolled_inner.set(scroll_y > 50.0);
        }) as Box<dyn FnMut()>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .expect("failed to add scroll event listener");

        closure.forget();
    });

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    let menu_items = vec![
        MenuItem {
            key: "home",
            icon_class: "fa-solid fa-house-chimney",
            label: "Home",
        },
        MenuItem {
            key: "features",
            icon_class: "fa-solid fa-cubes",
            label: "Features",
        },
        MenuItem {
            key: "testimonials",
            icon_class: "fa-solid fa-comments",
            label: "Testimonials",
        },
        MenuItem {
            key: "team",
            icon_class: "fa-solid fa-people-group",
            label: "Team",
        },
        MenuItem {
            key: "blog",
            icon_class: "fa-solid fa-newspaper",
            label: "Blog",
        },
    ];

    let nav_bg = if is_light {
        "bg-white shadow-sm"
    } else {
        "bg-black"
    };
    let link_color = if is_light {
        "text-gray-800 hover:text-green-600"
    } else {
        "text-white hover:text-green-400"
    };
    let hamburger_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    let blur_class = if is_scrolled() {
        "backdrop-blur-md"
    } else {
        ""
    };

    let menu_icon_class = format!(
        "fa {}",
        if is_menu_open() {
            "fa-times"
        } else {
            "fa-bars"
        }
    );

    rsx! {
        header {
            class: format!("fixed top-0 left-0 right-0 w-full transition-all duration-300 {}", blur_class),
            style: "z-index: 1024; max-width: 100vw;",

            div {
                class: format!(
                    "{} flex justify-between items-center w-full max-w-[1260px] \
                     mx-auto px-4 py-3 rounded-xl transition-colors duration-300 \
                     border border-transparent",
                    nav_bg
                ),
                style: if is_light { "border-color: var(--border-color);" } else { "" },

                img {
                    src: asset!("/assets/logo.png"),
                    alt: "Wise AI Logo",
                    class: "w-8 h-8 object-contain shrink-0"
                }

                nav {
                    class: "hidden md:flex flex-1 justify-center items-center",
                    aria_label: "Main Navigation",
                    ul {
                        class: "flex gap-8 lg:gap-12",
                        for item in &menu_items {
                            li {
                                a {
                                    href: format!("#{}", item.key),
                                    class: format!(
                                        "font-['Lexend'] text-[15px] font-normal uppercase \
                                         whitespace-nowrap transition-colors duration-200 {}",
                                        link_color
                                    ),
                                    i { class: format!("{} mr-1.5", item.icon_class), aria_hidden: "true" }
                                    {item.label}
                                }
                            }
                        }
                    }
                }

                div {
                    class: "flex items-center gap-3 shrink-0",
                    div {
                        class: "hidden md:flex gap-3 items-center",
                        ThemeToggle {}
                    }
                    button {
                        class: format!("md:hidden p-2 {}", hamburger_color),
                        onclick: toggle_menu,
                        aria_expanded: "{is_menu_open()}",
                        aria_label: "Toggle menu",
                        i { class: "{menu_icon_class}" }
                    }
                }
            }

            if is_menu_open() {
                nav {
                    class: format!(
                        "{} md:hidden w-full py-4 transition-colors duration-300",
                        nav_bg
                    ),
                    aria_label: "Mobile Navigation",
                    ul {
                        class: "flex flex-col gap-4 px-6",
                        for item in &menu_items {
                            li {
                                a {
                                    href: format!("#{}", item.key),
                                    class: format!(
                                        "flex items-center font-['Lexend'] text-base \
                                         font-normal uppercase whitespace-nowrap \
                                         transition-colors duration-200 {}",
                                        link_color
                                    ),
                                    onclick: move |_| is_menu_open.set(false),
                                    i { class: format!("{} mr-2", item.icon_class), aria_hidden: "true" }
                                    {item.label}
                                }
                            }
                        }
                        li {
                            class: "mt-2 flex gap-4 items-center",
                            ThemeToggle {}
                        }
                    }
                }
            }
        }
    }
}
