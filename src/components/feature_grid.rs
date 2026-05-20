use dioxus::prelude::*;

#[component]
pub fn FeatureGrid() -> Element {
    rsx! {
        section { class: "product-showcase",
            div { class: "showcase-text",
                article { class: "showcase-step",
                    div { class: "step-icon",
                        svg { width: "100%", height: "80", "viewBox": "0 0 300 100", fill: "none", stroke: "currentColor", "stroke-width": "6", "stroke-linecap": "round", "stroke-linejoin": "round",
                            path { d: "M 60 20 L 30 50 L 60 80 M 90 50 L 110 50 L 110 20 L 150 20 L 150 80 L 190 80 L 190 50 L 210 50 M 240 20 L 270 50 L 240 80" }
                        }
                    }
                    div { class: "step-content",
                        h2 { "Purely Algorithmic" }
                        p { "Zero image assets. Every knob and bezel is rendered in real-time using juce::Graphics." }
                    }
                }
                article { class: "showcase-step",
                    svg { class: "bg-wave", "viewBox": "0 0 400 100", preserve_aspect_ratio: "none", fill: "none", stroke: "currentColor", "stroke-width": "2",
                        path { d: "M 0 40 C 25 -10, 75 90, 100 40 C 125 -10, 175 90, 200 40 C 225 -10, 275 90, 300 40 C 325 -10, 375 90, 400 40" }
                        path { d: "M 0 50 C 30 10, 70 90, 100 50 C 130 10, 170 90, 200 50 C 230 10, 270 90, 300 50 C 330 10, 370 90, 400 50" }
                        path { d: "M 0 60 C 35 30, 65 70, 100 60 C 135 30, 165 70, 200 60 C 235 30, 265 70, 300 60 C 335 30, 365 70, 400 60" }
                    }
                    div { class: "step-content",
                        h2 { "Topology-Preserving" }
                        p { "A state-variable filter that drifts like physical hardware, bringing analog warmth to your signal." }
                    }
                }
                article { class: "showcase-step",
                    div { class: "step-icon",
                        svg { width: "100%", height: "80", "viewBox": "0 0 300 100", fill: "none", stroke: "currentColor", "stroke-width": "6", "stroke-linecap": "round", "stroke-linejoin": "round",
                            path { d: "M 0 50 L 100 50 L 120 20 L 160 80 L 180 50 L 300 50" }
                        }
                    }
                    div { class: "step-content",
                        h2 { "Real-Time Safe" }
                        p { "Zero memory allocations and lock-free processing. Guaranteed interruption-free audio." }
                    }
                }
                article { class: "showcase-step",
                    div { class: "step-icon",
                        svg { width: "100%", height: "80", "viewBox": "0 0 300 100", fill: "none", stroke: "currentColor", "stroke-width": "6", "stroke-linecap": "round", "stroke-linejoin": "round",
                            path { d: "M 0 80 L 120 40 L 180 60 L 300 20" }
                            circle { cx: "0", cy: "80", r: "6", fill: "currentColor" }
                            circle { cx: "120", cy: "40", r: "6", fill: "currentColor" }
                            circle { cx: "180", cy: "60", r: "6", fill: "currentColor" }
                            circle { cx: "300", cy: "20", r: "6", fill: "currentColor" }
                        }
                    }
                    div { class: "step-content",
                        h2 { "Rock-Solid State" }
                        p { 
                            "Built entirely on AudioProcessorValueTreeState "
                            br {}
                            "for flawless DAW automation and reliable session recall."
                        }
                    }
                }
            }
        }
    }
}
