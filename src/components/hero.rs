/// Hero
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

pub fn Hero() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 text-white",
            div { class: "text-center mb-12",
                img {
                    src: HEADER_SVG,
                    class: "w-32 h-32 mx-auto mb-8 drop-shadow-lg",
                    alt: "Dioxus Logo"
                }
                h1 { class: "text-5xl font-bold mb-4", "Welcome to Dioxus" }
                p { class: "text-xl opacity-90", "Build fast, reliable web applications with Rust" }
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl px-6",
                a {
                    href: "https://dioxuslabs.com/learn/0.6/",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "📚" }
                    h3 { class: "font-semibold mb-2", "Learn Dioxus" }
                    p { class: "text-sm opacity-80", "Get started with comprehensive guides" }
                }
                a {
                    href: "https://dioxuslabs.com/awesome",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "🚀" }
                    h3 { class: "font-semibold mb-2", "Awesome Dioxus" }
                    p { class: "text-sm opacity-80", "Discover amazing projects and resources" }
                }
                a {
                    href: "https://github.com/dioxus-community/",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "📡" }
                    h3 { class: "font-semibold mb-2", "Community Libraries" }
                    p { class: "text-sm opacity-80", "Extend your apps with community tools" }
                }
                a {
                    href: "https://github.com/DioxusLabs/sdk",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "⚙️" }
                    h3 { class: "font-semibold mb-2", "Development Kit" }
                    p { class: "text-sm opacity-80", "Professional development tools" }
                }
                a {
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "💫" }
                    h3 { class: "font-semibold mb-2", "VSCode Extension" }
                    p { class: "text-sm opacity-80", "Enhanced development experience" }
                }
                a {
                    href: "https://discord.gg/XgGxMSkvUM",
                    class: "bg-white/10 backdrop-blur-sm rounded-lg p-6 hover:bg-white/20 transition-all transform hover:scale-105 border border-white/20",
                    div { class: "text-3xl mb-3", "👋" }
                    h3 { class: "font-semibold mb-2", "Community Discord" }
                    p { class: "text-sm opacity-80", "Join our friendly community" }
                }
            }
        }
    }
}
