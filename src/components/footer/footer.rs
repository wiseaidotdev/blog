use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterSectionProps {
    pub title: String,
    pub content: String,
    pub is_light: bool,
}

#[component]
pub fn FooterSection(props: FooterSectionProps) -> Element {
    let title_color = if props.is_light {
        "text-green-700"
    } else {
        "text-green-500"
    };
    let content_color = if props.is_light {
        "text-gray-600"
    } else {
        "text-white"
    };

    rsx! {
        div {
            class: "flex flex-col gap-[25px]",

            h3 {
                class: format!("font-['Lexend'] text-[30px] font-bold uppercase {}", title_color),
                "{props.title}"
            }

            p {
                class: format!("font-['Lexend'] text-[15px] {}", content_color),
                "{props.content}"
            }
        }
    }
}
