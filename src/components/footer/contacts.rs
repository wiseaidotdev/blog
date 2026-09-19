use crate::components::footer::footer::FooterSection;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LocationContactProps {
    pub is_light: bool,
    pub section: String,
}

#[component]
pub fn LocationContact(props: LocationContactProps) -> Element {
    if props.section == "location" {
        rsx! {
            FooterSection {
                title: "Location",
                content: "The Cosmos",
                is_light: props.is_light,
            }
        }
    } else {
        rsx! {
            FooterSection {
                title: "Contact Us",
                content: "oss@wiseai.dev",
                is_light: props.is_light,
            }
        }
    }
}
