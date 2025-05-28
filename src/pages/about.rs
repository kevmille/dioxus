/// About Content component
use dioxus::prelude::*;

pub fn AboutMain() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-6 py-12",
            div { class: "bg-white rounded-xl shadow-lg p-8 mb-8",
                h1 { class: "text-4xl font-bold text-gray-900 mb-6",
                    "About Dioxus"
                }
                p { class: "text-lg text-green-600 bg-green-50 rounded-lg p-4 mb-6",
                    "Check out the "
                    Link {
                        class: "font-semibold hover:underline hover:decoration-dashed uppercase text-green-700",
                        rel: "noopener noreferrer",
                        to: "https://dioxuslabs.com",
                        "Dioxus"
                    }
                    " website for more information."
                }

                h2 { class: "text-2xl font-semibold text-gray-800 mb-4 uppercase tracking-wide", "Key Features" }
                ul { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-8",
                    li { class: "flex items-center p-3 bg-bamboo-50 rounded-lg",
                        span { class: "w-2 h-2 bg-bamboo-500 rounded-full mr-3" }
                        span { class: "font-medium text-gray-700", "Modern Rust" }
                    }
                    li { class: "flex items-center p-3 bg-bamboo-50 rounded-lg",
                        span { class: "w-2 h-2 bg-bamboo-500 rounded-full mr-3" }
                        span { class: "font-medium text-gray-700", "Blazingly Fast" }
                    }
                    li { class: "flex items-center p-3 bg-bamboo-50 rounded-lg",
                        span { class: "w-2 h-2 bg-bamboo-500 rounded-full mr-3" }
                        span { class: "font-medium text-gray-700", "Responsive Design" }
                    }
                    li { class: "flex items-center p-3 bg-bamboo-50 rounded-lg",
                        span { class: "w-2 h-2 bg-bamboo-500 rounded-full mr-3" }
                        span { class: "font-medium text-gray-700", "Highly Scalable" }
                    }
                }

                p { class: "text-lg text-blue-700 bg-blue-50 rounded-lg p-6 leading-relaxed",
                    "Dioxus is a modern Rust framework for building web applications. It is designed to be fast, responsive, and scalable, making it an ideal choice for building large-scale web applications with the safety and performance of Rust."
                }
            }

        }
    }
}
