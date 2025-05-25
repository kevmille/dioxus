/// Footer
use dioxus::prelude::*;

pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-bamboo-500",
            div { class: "mx-auto max-w-7xl px-6 py-12 md:flex md:items-center md:justify-between lg:px-8",

                p { class: "mt-8 text-center text-sm leading-6 text-gray-600 md:order-1 md:mt-0",
                    "© 2025 CyberNezumi.com. All rights reserved."
                }
            }
        }
    }
}
