use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LogoSocialProps {
    pub is_light: bool,
}

#[component]
pub fn LogoSocial(props: LogoSocialProps) -> Element {
    let socials = vec![
        ("Facebook", "fab fa-facebook-f"),
        ("Twitter", "fab fa-x-twitter"),
        ("Instagram", "fab fa-instagram"),
        ("LinkedIn", "fab fa-linkedin-in"),
    ];

    let icon_color = if props.is_light {
        "text-gray-600 hover:text-green-700"
    } else {
        "text-gray-300 hover:text-green-500"
    };

    rsx! {
        div {
            class: "flex flex-col gap-[25px] w-full md:w-[320px]",

            img {
                src: asset!("/assets/logo.png"),
                class: "w-[60px] h-[49px] object-cover",
                width: 60,
                height: 60,
                alt: "Wise AI Logo",
            }

            div {
                class: "flex",
                {socials
                    .iter()
                    .map(|(label, icon_class)| rsx! {
                        a {
                            href: "#",
                            class: format!("flex items-center justify-center transition-colors duration-200 text-[20px] {}", icon_color),
                            aria_label: "{label}",
                            i { class: "{icon_class} text-xl px-2" }
                        }
                    })}
            }
        }
    }
}
