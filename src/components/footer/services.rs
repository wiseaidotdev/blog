use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ServicesListProps {
    pub services: Vec<String>,
    pub is_light: bool,
}

#[component]
pub fn ServicesList(props: ServicesListProps) -> Element {
    let title_color = if props.is_light {
        "text-green-700"
    } else {
        "text-green-500"
    };
    let link_color = if props.is_light {
        "text-gray-700 hover:text-green-700"
    } else {
        "text-white hover:text-green-500"
    };

    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[233px]",

            h3 {
                class: format!("font-['Lexend'] text-[30px] font-bold uppercase {}", title_color),
                "Services"
            }

            ul {
                class: "flex flex-col gap-[10px]",
                for (i, item) in props.services.iter().enumerate() {
                    li {
                        key: "{i}",
                        class: "font-['Lexend'] text-[15px]",
                        a {
                            href: "#",
                            class: format!("transition-colors duration-200 {}", link_color),
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}
