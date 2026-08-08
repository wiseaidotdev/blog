use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SubscribeFormProps {
    pub is_light: bool,
}

#[component]
pub fn SubscribeForm(props: SubscribeFormProps) -> Element {
    let title_color = if props.is_light {
        "text-green-700"
    } else {
        "text-green-500"
    };
    let input_class = if props.is_light {
        "flex-1 px-4 py-2 rounded-lg border border-gray-300 bg-white text-gray-800 placeholder-gray-400 focus:outline-none focus:border-green-600 text-sm"
    } else {
        "flex-1 px-4 py-2 rounded-lg border border-gray-700 bg-gray-800 text-white placeholder-gray-500 focus:outline-none focus:border-green-500 text-sm"
    };

    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[250px]",

            h3 {
                class: format!("font-['Lexend'] text-[30px] font-bold uppercase {}", title_color),
                "Subscribe"
            }

            form {
                class: "flex gap-2",
                onsubmit: |evt: Event<FormData>| { evt.prevent_default(); },

                input {
                    r#type: "email",
                    class: "{input_class}",
                    placeholder: "Enter your email",
                    aria_label: "Enter your email",
                }

                button {
                    r#type: "submit",
                    class: "px-4 py-2 rounded-lg bg-green-600 text-white font-semibold text-sm hover:bg-green-700 transition-colors duration-200 shrink-0",
                    i { class: "fa-solid fa-paper-plane" }
                }
            }
        }
    }
}
