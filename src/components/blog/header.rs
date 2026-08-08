use crate::router::Route;
use crate::theme::ThemeToggle;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn BlogHeader() -> Element {
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

    let title_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    let back_btn_class = if is_light {
        "text-gray-600 bg-gray-100 border border-gray-200 hover:bg-gray-200 hover:text-gray-900 px-4 py-2 rounded-lg text-sm font-['Lexend'] transition-colors duration-200 flex items-center gap-2"
    } else {
        "text-gray-300 bg-gray-800 border border-gray-700 hover:bg-gray-700 hover:text-white px-4 py-2 rounded-lg text-sm font-['Lexend'] transition-colors duration-200 flex items-center gap-2"
    };

    let hamburger_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };

    let header_class = format!(
        "w-full fixed top-0 left-0 right-0 transition-all duration-300 {}",
        if is_scrolled() {
            "backdrop-blur-md"
        } else {
            ""
        }
    );

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
            class: "{header_class}",
            style: "z-index: 1024;",

            div {
                class: "{nav_bg} flex justify-between items-center w-full max-w-[1260px] mx-auto px-4 py-3 rounded-xl relative transition-colors duration-300 border border-transparent mt-2",
                style: if is_light { "border-color: var(--border-color);" } else { "" },

                div { class: "flex items-center gap-2",
                    img {
                        src: asset!("/assets/logo.webp"),
                        alt: "Wise AI Logo",
                        class: "w-8 h-8 object-contain shrink-0",
                        loading: "lazy",
                    }
                    span {
                        class: format!("text-base font-bold font-['Lexend'] hidden sm:block {}", title_color),
                        "wise ai"
                    }
                }

                nav {
                    class: "hidden md:flex items-center gap-6",
                    aria_label: "Blog Navigation",
                    a {
                        href: "/#home",
                        class: format!("text-sm font-['Lexend'] uppercase whitespace-nowrap transition-colors duration-200 {}", link_color),
                        i { class: "fa-solid fa-house-chimney mr-1.5", aria_hidden: "true" }
                        "Home"
                    }
                    a {
                        href: "/#blog",
                        class: format!("text-sm font-['Lexend'] uppercase whitespace-nowrap transition-colors duration-200 {}", link_color),
                        i { class: "fa-solid fa-newspaper mr-1.5", aria_hidden: "true" }
                        "Blog"
                    }
                }

                div { class: "flex items-center gap-3",
                    div { class: "hidden md:flex items-center gap-3",
                        ThemeToggle {}
                        Link {
                            to: Route::Home {},
                            class: "{back_btn_class}",
                            i { class: "fa-solid fa-arrow-left text-xs" }
                            "Go Back"
                        }
                    }

                    div { class: "flex md:hidden items-center gap-2",
                        ThemeToggle {}
                        button {
                            class: format!("p-2 {}", hamburger_color),
                            onclick: move |_| is_menu_open.set(!is_menu_open()),
                            aria_expanded: "{is_menu_open()}",
                            aria_label: "Toggle menu",
                            i { class: "{menu_icon_class}" }
                        }
                    }
                }
            }

            if is_menu_open() {
                nav {
                    class: format!("{} md:hidden w-full py-3 px-4 transition-colors duration-300", nav_bg),
                    aria_label: "Mobile Blog Navigation",
                    ul { class: "flex flex-col gap-3",
                        li {
                            a {
                                href: "/#home",
                                class: format!("flex items-center text-sm font-['Lexend'] uppercase transition-colors duration-200 {}", link_color),
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-house-chimney mr-2", aria_hidden: "true" }
                                "Home"
                            }
                        }
                        li {
                            a {
                                href: "/#blog",
                                class: format!("flex items-center text-sm font-['Lexend'] uppercase transition-colors duration-200 {}", link_color),
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-newspaper mr-2", aria_hidden: "true" }
                                "Blog"
                            }
                        }
                        li { class: "mt-2",
                            Link {
                                to: Route::Home {},
                                class: "{back_btn_class}",
                                onclick: move |_| is_menu_open.set(false),
                                i { class: "fa-solid fa-arrow-left text-xs" }
                                "Go Back"
                            }
                        }
                    }
                }
            }
        }
    }
}
