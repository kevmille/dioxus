mod components;
use components::{CallToAction, Footer, Hero, FAQ};

use crate::CallToAction::CallToAction;
use crate::Footer::Footer;
use crate::Hero::Hero;
use crate::FAQ::FAQ;
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

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
        Footer {}
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        CallToAction {}
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
        FAQ {}
        Footer {}
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
        Footer {}
    }
}

/// Shared navbar component
#[component]
fn Navbar() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        header { class: "bg-white",
            nav {
                class: "mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8",
                "aria-label": "Global",
                Link {
                    to: Route::Home {},
                    class: "-m-1.5 p-1.5",
                    span { class: "sr-only", "Your Company" }
                    img {
                        class: "h-8 w-auto",
                        src: "https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600",
                        alt: ""
                    }
                }
                div { class: "flex lg:hidden",
                    button {
                        r#type: "button",
                        class: "-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700",
                        onclick: move |_| mobile_menu_open.set(true),
                        span { class: "sr-only bg-red-500", "Open main menu" }
                        svg {
                            class: "size-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            "aria-hidden": "true",
                            "data-slot": "icon",
                            path {
                                view_box: "0 0 24 24",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                            }
                        }
                    }
                }
                div { class: "hidden lg:flex lg:gap-x-12",
                    Link {
                        to: Route::Home {},
                        class: "text-sm/6 font-semibold text-gray-900",
                        "🏠 Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "text-sm/6 font-semibold text-gray-900",
                        "ℹ️ About"
                    }
                    Link {
                        to: Route::Blog { id: 1 },
                        class: "text-sm/6 font-semibold text-gray-900",
                        "📝 Blog"
                    }
                }
            }

            if mobile_menu_open() {
                div {
                    class: "lg:hidden",
                    role: "dialog",
                    "aria-modal": "true",
                    div { class: "fixed inset-0 z-10" }
                    div {
                        class: "fixed inset-y-0 right-0 z-10 w-full overflow-y-auto bg-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10",
                        div { class: "flex items-center justify-between",
                            Link {
                                to: Route::Home {},
                                class: "-m-1.5 p-1.5",
                                span { class: "sr-only", "Your Company" }
                                img {
                                    class: "h-8 w-auto",
                                    src: "https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600",
                                    alt: ""
                                }
                            }
                            button {
                                r#type: "button",
                                class: "-m-2.5 rounded-md p-2.5 text-gray-700",
                                onclick: move |_| mobile_menu_open.set(false),
                                span { class: "sr-only", "Close menu" }
                                svg {
                                    class: "size-6",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    "aria-hidden": "true",
                                    "data-slot": "icon",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        d: "M6 18 18 6M6 6l12 12"
                                    }
                                }
                            }
                        }
                        div { class: "mt-6 flow-root",
                            div { class: "-my-6 divide-y divide-gray-500/10",
                                div { class: "space-y-2 py-6",
                                    Link {
                                        to: Route::Home {},
                                        class: "-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        "🏠 Home"
                                    }
                                    Link {
                                        to: Route::About {},
                                        class: "-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        "ℹ️ About"
                                    }
                                    Link {
                                        to: Route::Blog { id: 1 },
                                        class: "-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        "📝 Blog"
                                    }
                                }
                            }
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
