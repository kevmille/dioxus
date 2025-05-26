/// Footer
use dioxus::prelude::*;

const ROCKYPOD: Asset = asset!("/assets/rockypod.svg");

pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-sakura-200 border-t-2 border-forest-800",
            div { class: "mx-auto max-w-7xl px-6 py-12 md:flex md:items-center md:justify-between lg:px-8",
                div { class: "flex items-center",
                    img {
                        class: "h-16 w-auto rounded-full border border-sakura-800 border-4 mr-3",
                        src: "{ROCKYPOD}",
                        alt: "RockyPod Avatar"
                    }
                    span { class: "text-2xl sm:text-3xl lg:text-4xl font-bold text-sakura-800",
                        span { class: "text-forest-800", "crypto" }
                        "nezumi.com"
                    }
                }
                p { class: "mt-8 text-center text-lg leading-6 text-sakura-600 font-bold md:order-1 md:mt-0",
                    "© 2025 "
                    a { class: "hover:text-forest-800 hover:underline hover:decoration-2", href: "https://cryptonezumi.com",
                        "cryptonezumi.com"
                    }
                    ". All rights reserved."
                }
            }
        }
    }
}
