use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> { }
    }
}

#[component]
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

/// Call-to-action component
#[component]
fn CallToAction() -> Element {
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
/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        CallToAction {}
        div { class: "max-w-4xl mx-auto px-6 py-12",
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

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            div { class: "bg-white rounded-xl shadow-lg p-8",
                h1 { class: "text-3xl font-bold text-gray-900 mb-6",
                    "Blog Post #{id}"
                }
                p { class: "text-gray-700 mb-8 text-lg leading-relaxed",
                    "In blog post #{id}, we explore how the Dioxus router works and demonstrate how URL parameters can be seamlessly passed as props to our route components. This showcases the power and flexibility of Dioxus's routing system."
                }

                div { class: "flex items-center justify-between pt-6 border-t border-gray-200",
                    Link {
                        to: Route::Blog { id: id - 1 },
                        class: "flex items-center px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors",
                        span { class: "mr-2", "←" }
                        "Previous Post"
                    }
                    Link {
                        to: Route::Blog { id: id + 1 },
                        class: "flex items-center px-4 py-2 bg-bamboo-600 hover:bg-bamboo-500 text-white rounded-lg transition-colors",
                        "Next Post"
                        span { class: "ml-2", "→" }
                    }
                }
            }
        }
    }
}

/// Shared navbar component
#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-sm border-b border-gray-200",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between h-16",
                    div { class: "flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: "text-gray-900 hover:text-bamboo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                            "🏠 Home"
                        }
                        Link {
                            to: Route::About {},
                            class: "text-gray-700 hover:text-bamboo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                            "ℹ️ About"
                        }
                        Link {
                            to: Route::Blog { id: 1 },
                            class: "text-gray-700 hover:text-bamboo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                            "📝 Blog"
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div { class: "max-w-2xl mx-auto px-6 py-12",
            div { class: "bg-white rounded-xl shadow-lg p-8",
                h4 { class: "text-2xl font-bold text-gray-900 mb-6",
                    "🔄 ServerFn Echo Demo"
                }
                div { class: "space-y-4",
                    input {
                        class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-bamboo-500 focus:border-transparent transition-all",
                        placeholder: "Type here to echo from server...",
                        oninput: move |event| async move {
                            let data = echo_server(event.value()).await.unwrap_or_else(|_| "Error".to_string());
                            response.set(data);
                        },
                    }

                    if !response().is_empty() {
                        div { class: "bg-bamboo-50 border border-bamboo-200 rounded-lg p-4",
                            p { class: "text-bamboo-800",
                                "Server echoed: "
                                span { class: "font-mono font-semibold", "{response}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Echo the user input on the server
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
