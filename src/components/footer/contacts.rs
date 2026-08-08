use crate::components::footer::footer::FooterSection;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LocationContactProps {
    pub is_light: bool,
}

#[component]
pub fn LocationContact(props: LocationContactProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-[80px] w-full md:w-[210px]",

            FooterSection {
                title: "Location",
                content: "The Cosmos",
                is_light: props.is_light,
            }

            FooterSection {
                title: "Contact Us",
                content: "oss@wiseai.dev",
                is_light: props.is_light,
            }
        }
    }
}
