mod components;
use components::{AboutContent, BentoGrid, CallToAction, Footer, Hero, FAQ};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaCss3, FaRust};
use dioxus_free_icons::Icon;
use lucide_dioxus::{Info, PencilLine};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/dioxus-tailwind")]
    DioxusTailwind {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const ROCKYPOD: Asset = asset!("/assets/rockypod.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "icon", href: ROCKYPOD }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> { }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        BentoGrid {}
        Echo {}
        Footer {}
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        CallToAction {}
        AboutContent {}
        FAQ {}
        Footer {}
    }
}

/// Dioxus Tailwind page
#[component]
pub fn DioxusTailwind() -> Element {
    let package_json = r#"
{
  "name": "cryptonezumi.com-dioxus",
  "version": "1.0.0",
  "scripts": {
    "dev": "concurrently \"npm run css:watch\" \"dx serve\"",
    "build": "npm run css:build && dx build --release",
    "css:build": "tailwindcss -i ./assets/input.css -o ./assets/tailwind.css",
    "css:watch": "tailwindcss -i ./assets/input.css -o ./assets/tailwind.css --watch"
  },
  "dependencies": {
    "@tailwindcss/cli": "^4.1.7",
    "tailwindcss": "^4.1.7"
  },
  "devDependencies": {
    "concurrently": "^9.1.2"
  }
}
"#;

    rsx! {
        div { class: "bg-bamboo-50 px-6 py-32 lg:px-8",
            div { class: "mx-auto max-w-3xl text-bamboo-800",
                p { class: "font-bold text-sakura-800 uppercase", "Tailwind 4" }
                h1 { class: "mt-2 text-4xl font-bold tracking-tight text-pretty text-sakura-500 sm:text-5xl",
                    "Dioxus + Tailwind CSS"
                }
                p { class: "mt-6 text-xl font-bold",
                    "Tailwind 4 requires changes to the initial setup process for Dioxus."
                }
                div { class: "mt-10 text-lg font-semibold max-w-2xl",
                    p {
                        "With the release of Tailwind 4 came some changes that removed several older steps during the installation process. You just need to install the Tailwind CLI. You no longer need to use the npx tailwindcss init command."
                        br {}
                        br {}
                        "When you install the Dioxus CLI, select false for TailwindCSS. If you select true, you will get the setup for Tailwind 3. Then go to the following URL and install the Tailwind CLI: "
                        br {}
                        br {}
                        Link {
                            to: "https://tailwindcss.com/docs/installation/tailwind-cli",
                            rel: "noopener noreferrer",
                            class: "text-sakura-500 hover:underline hover:text-indigo-600 hover:decoration-indigo-600 hover:decoration-wavy ml-4",
                            "https://tailwindcss.com/docs/installation/tailwind-cli"
                        }
                        br {}
                        br {}
                        "Be sure to place " strong{"input.css"}  " and " strong{"tailwind.css"} " in the " strong{"assets"} " folder."
                    }
                    p { class: "mt-8",
                        "You will not need " strong{"tailwind.config.js"} " with Tailwind 4. For " strong{"input.css"} ", just add "
                       br{} br{} strong{ class: "ml-4", "@import tailwindcss;"} br{} br{} " to the file. All other of your customizations will added to this file as well."
                    }
                    p { class: "mt-8",
                        "Now for the important part, install the " strong{"concurrently"} " package. This will allow you to run both dx and tailwindcss concurrently, meaning your Tailwind changes will get rendered (Yes, this is one of the main reasons your new Tailwind classes were not being rebuilt.)."
                    }
                    p { class: "mt-8 bg-gray-900 text-green-200 p-4 rounded overflow-x-auto text-sm",
                        "$ npm install --save-dev concurrently"
                    }
                    p { class: "mt-8",
                        "Check the relative paths for both input.css and tailwindcss. Update your " strong{"package.json"} " file to match the following:"
                    }
                    pre { class: "mt-8 bg-gray-900 text-green-200 p-4 rounded overflow-x-auto text-sm",
                        code { class: "language-json", "{package_json}" }
                    }
                    p { class: "mt-8",
                        "Now you can execute the following command in your project root folder to run tailwindcss and dx concurrently:"
                    }
                    p { class: "mt-8 bg-gray-900 text-green-200 p-4 rounded overflow-x-auto text-sm",
                        "$ npm run dev"
                    }
                }
            }
        }
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
        header { class: "bg-sakura-100 border-b-2 border-forest-700 sticky top-0 z-10",
            nav {
                class: "mx-auto flex max-w-7xl items-center justify-between p-2 lg:px-4",
                "aria-label": "Global",
                Link {
                    to: Route::Home {},
                    class: "-m-1.5 p-1.5 flex items-center",
                    span { class: "sr-only", "CryptoNezumi.com" }
                    img {
                        class: "h-16 w-auto rounded-full border border-sakura-800 border-4",
                        src: ROCKYPOD,
                        alt: "RockyPod Avatar"
                    }
                    span { class: "ml-3 text-2xl sm:text-3xl lg:text-4xl font-bold text-sakura-800",
                        span { class: "text-forest-800", "crypto" }
                        "nezumi.com"
                    }
                }
                div { class: "flex lg:hidden",
                    button {
                        r#type: "button",
                        class: "-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-sakura-900 hover:bg-sakura-100",
                        onclick: move |_| mobile_menu_open.set(true),
                        span { class: "sr-only", "Open main menu" }
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
                        class: "text-xl font-bold text-sakura-500 flex items-center",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: FaRust,
                        }
                        span { class: "ml-2 uppercase hover:text-indigo-500 hover:underline hover:decoration-wavy", "Home" }
                    }
                    Link {
                        to: Route::About {},
                        class: "text-xl font-bold text-sakura-500 flex items-center",
                        Info {
                            color: "black",
                            size: 30,
                        }
                        span { class: "ml-2 uppercase hover:text-indigo-500 hover:underline hover:decoration-wavy", "About" }
                    }
                    Link {
                        to: Route::DioxusTailwind {},
                        class: "text-xl font-bold text-sakura-500 flex items-center",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "black",
                            icon: FaCss3,
                        }
                        span { class: "ml-2 uppercase hover:text-indigo-500 hover:underline hover:decoration-wavy", "Dioxus + Tailwind" }
                    }
                    Link {
                        to: Route::Blog { id: 1 },
                        class: "text-xl font-bold text-sakura-500 flex items-center",
                        PencilLine {
                            color: "black",
                            size: 30,
                        }
                        span { class: "ml-2 uppercase hover:text-indigo-500 hover:underline hover:decoration-wavy", "Blog" }
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
                        class: "fixed inset-y-0 right-0 z-10 w-full overflow-y-auto bg-sakura-50 px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-forest-900/10",
                        div { class: "flex items-center justify-between",
                            Link {
                                to: Route::Home {},
                                class: "-m-1.5 p-1.5",
                                span { class: "sr-only", "CryptoNezumi.com" }
                                img {
                                    class: "h-18 w-auto rounded-full border-4 border-sakura-800",
                                    src: ROCKYPOD,
                                    alt: "RockyPod Avatar"
                                }
                            }
                            button {
                                r#type: "button",
                                class: "-m-2.5 rounded-md p-2.5 text-sakura-900 hover:bg-sakura-100",
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
                            div { class: "-my-6 divide-y divide-sakura-700",
                                div { class: "space-y-2 py-6",
                                    Link {
                                        to: Route::Home {},
                                        class: "-mx-3 flex items-center rounded-lg px-3 py-2 text-lg font-semibold text-sakura-500 hover:bg-forest-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        Icon {
                                            width: 30,
                                            height: 30,
                                            fill: "black",
                                            icon: FaRust,
                                        }
                                        span { class: "ml-2 uppercase hover:text-indigo-500", "Home" }
                                    }
                                    Link {
                                        to: Route::About {},
                                        class: "-mx-3 flex items-center rounded-lg px-3 py-2 text-lg font-semibold text-sakura-500 hover:bg-forest-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        Info {
                                            color: "black",
                                            size: 30,
                                        }
                                        span { class: "ml-2 uppercase hover:text-indigo-500", "About" }
                                    }
                                    Link {
                                        to: Route::DioxusTailwind {},
                                        class: "-mx-3 flex items-center rounded-lg px-3 py-2 text-lg font-semibold text-sakura-500 hover:bg-forest-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        Icon {
                                            width: 30,
                                            height: 30,
                                            fill: "black",
                                            icon: FaCss3,
                                        }
                                        span { class: "ml-2 uppercase hover:text-indigo-500", "Dioxus + Tailwind" }
                                    }
                                    Link {
                                        to: Route::Blog { id: 1 },
                                        class: "-mx-3 flex items-center rounded-lg px-3 py-2 text-lg font-semibold text-sakura-500 hover:bg-forest-50",
                                        onclick: move |_| mobile_menu_open.set(false),
                                        PencilLine {
                                            color: "black",
                                            size: 30,
                                        }
                                        span { class: "ml-2 uppercase hover:text-indigo-500", "Blog" }
                                    }
                                }span { class: "ml-3 text-2xl text-4xl font-bold text-sakura-800",
                                    span { class: "text-forest-800", "crypto" }
                                    "nezumi.com"
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
