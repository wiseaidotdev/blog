use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: Element,
    title: String,
    description: String,
    is_light: bool,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    let title_color = if props.is_light {
        "text-gray-900"
    } else {
        "text-white"
    };
    let desc_color = if props.is_light {
        "text-gray-600"
    } else {
        "text-gray-300"
    };
    let icon_color = if props.is_light {
        "text-green-600"
    } else {
        "text-green-400"
    };

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: format!("w-12 h-12 {}", icon_color), {props.icon} }
            h3 { class: format!("text-2xl font-bold leading-snug font-['Lexend'] {}", title_color), "{props.title}" }
            p { class: format!("text-base leading-relaxed {}", desc_color), "{props.description}" }
        }
    }
}
