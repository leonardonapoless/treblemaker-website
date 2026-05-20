use dioxus::prelude::*;

mod components;
use components::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut show_modal = use_context_provider(|| Signal::new(false));

    rsx! {
        document::Title { "TrebleMaker" }
        document::Stylesheet { href: asset!("/assets/base.css") }
        document::Stylesheet { href: asset!("/assets/hero.css") }
        document::Stylesheet { href: asset!("/assets/feature_grid.css") }
        document::Stylesheet { href: asset!("/assets/download.css") }
        document::Stylesheet { href: asset!("/assets/modal.css") }
        if show_modal() {
            div { 
                class: "modal-overlay",
                onclick: move |_| show_modal.set(false),
                div { 
                    class: "modal-content",
                    onclick: move |e| e.stop_propagation(),
                    h2 { "Thank you for downloading!" }
                    p { "Your download should begin automatically." }
                    p { class: "modal-subtitle", "TrebleMaker is completely free, but if you enjoy using it, please consider supporting its development on Gumroad!" }
                    a { 
                        href: "https://gumroad.com/l/treblemaker", 
                        target: "_blank", 
                        class: "download-btn support-btn modal-gumroad-btn", 
                        "[ SUPPORT ON GUMROAD ]" 
                    }
                    button { 
                        class: "modal-close-btn",
                        onclick: move |_| show_modal.set(false),
                        "[ CLOSE ]"
                    }
                }
            }
        }
        div { class: "layout-wrapper",
            div { class: "layout-hero", Hero {} }
            div { class: "layout-download", Download {} }
            div { class: "layout-features", FeatureGrid {} }
        }
    }
}
