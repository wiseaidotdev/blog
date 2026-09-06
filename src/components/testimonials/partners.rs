use dioxus::prelude::*;

#[component]
pub fn Partners() -> Element {
    let logos = vec![
        (asset!("/assets/rustacean-1.png"), "120", "117"),
        (asset!("/assets/rustacean-2.png"), "120", "102"),
        (asset!("/assets/rustacean-3.png"), "120", "102"),
        (asset!("/assets/rustacean-1.png"), "120", "112"),
        (asset!("/assets/rustacean-2.png"), "120", "112"),
    ];

    rsx! {
        div {
            class: "flex w-[1634px] gap-[100px] justify-center items-start shrink-0 flex-nowrap relative z-[162]",
            role: "region",
            aria_label: "Partner logos",

            for (src, width, height) in logos.iter() {
                img {
                    src: "{src}",
                    width: "{width}",
                    height: "{height}",
                    class: "shrink-0 object-contain",
                    alt: "Partner logo",
                    aria_hidden: "true"
                }
            }
        }
    }
}
