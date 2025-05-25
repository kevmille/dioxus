/// Call-to-action component
use dioxus::prelude::*;

pub fn CallToAction() -> Element {
    rsx! {
        div { class: "bg-forest-100 py-24",
            div { class: "mx-auto max-w-7xl px-6 lg:flex lg:items-center lg:justify-between lg:px-8",
                h2 { class: "max-w-2xl text-4xl font-semibold tracking-tight text-gray-900 lg:text-5xl",
                    "Ready to dive in? "
                    br { class: "hidden sm:inline" }
                    span { class: "text-bamboo-600", "Start your free trial today." }
                }
                div { class: "mt-10 flex flex-col sm:flex-row items-center gap-4 lg:mt-0 lg:shrink-0",
                    a {
                        href: "/",
                        class: "w-full sm:w-auto rounded-lg bg-bamboo-600 px-6 py-3 text-base font-semibold text-white shadow-lg hover:bg-bamboo-500 focus:outline-none focus:ring-2 focus:ring-bamboo-600 focus:ring-offset-2 transition-colors",
                        "Get started"
                    }
                    a {
                        href: "/",
                        class: "w-full sm:w-auto text-base font-semibold text-gray-900 hover:text-bamboo-600 transition-colors flex items-center justify-center",
                        "Learn more "
                        span { class: "ml-2", "→" }
                    }
                }
            }
        }
    }
}
