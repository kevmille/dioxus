use crate::{About, Blog, DioxusTailwind, Home};
/// HeaderNav component
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaCss3, FaRust};
use dioxus_free_icons::icons::ld_icons::{LdInfo, LdPencilLine};
use dioxus_free_icons::Icon;

const ROCKYPOD: Asset = asset!("/assets/rockypod.svg");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderNav)]
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
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "icon", href: ROCKYPOD }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> { }
    }
}

pub fn HeaderNav() -> Element {
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
                        Icon {
                                width: 30,
                                height: 30,
                                fill: "black",
                                icon: LdInfo,
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
                        Icon {
                                width: 30,
                                height: 30,
                                fill: "black",
                                icon: LdPencilLine,
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
                                        Icon {
                                                width: 30,
                                                height: 30,
                                                fill: "black",
                                                icon: LdInfo,
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
                                        Icon {
                                                width: 30,
                                                height: 30,
                                                fill: "black",
                                                icon: LdPencilLine,
                                            }
                                        span { class: "ml-2 uppercase hover:text-indigo-500", "Blog" }
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
