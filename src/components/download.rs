use dioxus::prelude::*;

#[component]
pub fn Download() -> Element {
    let mut show_modal = use_context::<Signal<bool>>();

    rsx! {
        div { class: "download-container",
            header { class: "download-header",
                h2 { "GET IT" }
            }
            section { class: "download-group support-group",
                a { href: "https://gumroad.com/l/treblemaker", target: "_blank", class: "download-btn support-btn", "[ SUPPORT ON GUMROAD ]" }
            }
            section { class: "download-group",
                h3 { "macOS" }
                p { "AU, VST3, CLAP" }
                a { 
                    class: "download-btn", 
                    href: "https://github.com/leonardonapoless/TrebleMaker/releases/download/v1.0.0/TrebleMaker_macOS_Installer_1.0.0.pkg",
                    onclick: move |_| show_modal.set(true),
                    "[ DOWNLOAD .PKG ]" 
                }
            }
            section { class: "download-group",
                h3 { "Windows" }
                p { "VST3, CLAP" }
                a { 
                    class: "download-btn", 
                    href: "https://github.com/leonardonapoless/TrebleMaker/releases/download/v1.0.0/TrebleMaker_Windows_Installer_1.0.0.exe",
                    onclick: move |_| show_modal.set(true),
                    "[ DOWNLOAD .EXE ]" 
                }
            }
        }
    }
}
