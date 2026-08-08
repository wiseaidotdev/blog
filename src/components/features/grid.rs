use crate::components::features::item::FeatureItem;
use crate::components::features::Feature;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FeatureGridProps {
    features: Vec<Feature>,
    is_light: bool,
}

#[component]
pub fn Grid(props: FeatureGridProps) -> Element {
    let card_class = if props.is_light {
        "border border-gray-200 shadow-md p-6 rounded-xl bg-white cursor-pointer hover:shadow-xl hover:-translate-y-1 transition-all duration-300"
    } else {
        "border border-gray-800 shadow-md p-6 rounded-xl bg-gray-900 cursor-pointer hover:bg-gray-800 hover:shadow-lg hover:-translate-y-1 transition-all duration-300"
    };

    rsx! {
        div { class: "mt-8 grid grid-cols-1 md:grid-cols-3 gap-12",
            for feature in &props.features {
                div {
                    class: "{card_class}",
                    FeatureItem {
                        icon: feature.icon.clone(),
                        title: feature.title.clone(),
                        description: feature.description.clone(),
                        is_light: props.is_light,
                    }
                }
            }
        }
    }
}
