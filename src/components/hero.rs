use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            img {
                class: "hero-image",
                src: asset!("/assets/treblemaker.png"),
                alt: "TrebleMaker Plugin Interface"
            }
            header { class: "hero-copy",
                h1 { class: "hero-title",
                    span { class: "title-word", "TREBLE" }
                    span { class: "title-word highlight", "MAKER" }
                }
                div { class: "hero-tagline-group",
                    p { class: "hero-tagline",
                        "Here's to the crazy ones. The misfits. The rebels. The TrebleMakers."
                    }
                    p { class: "hero-specs",
                        "A simple high-shelf filter plugin built with JUCE."
                    }
                }
            }
            div { class: "hero-credits",
                span { "who made it:" }
                a {
                    href: "https://github.com/leonardonapoless",
                    target: "_blank",
                    class: "github-link",
                    dangerous_inner_html: include_str!("../../assets/github.svg")
                }
            }
        }
    }
}
