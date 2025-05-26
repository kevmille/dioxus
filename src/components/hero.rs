use dioxus::prelude::global_attributes::aria_hidden;
/// Hero
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

pub fn Hero() -> Element {
    rsx! {
        div { class: "overflow-hidden bg-bamboo-50 py-24 sm:py-32 border-b-2 border-forest-700",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2",
                    div { class: "lg:pt-4 lg:pr-8",
                        div { class: "lg:max-w-lg",
                            h2 { class: "text-lg font-semibold text-sakura-500", "Learning Rust." }
                            p { class: "mt-2 text-2xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-4xl",
                                "A new Rustacean convert"
                            }
                            p { class: "mt-6 text-lg/8 text-gray-600",
                                "I am rockypod, a fulltime webapp developer. During the day, I develop with Drupal and Twig. In my freetime, I am making the transition to Rust and Dioxus. Below will be the focus of this blog:"
                            }
                            dl { class: "mt-10 max-w-xl space-y-8 text-base/7 text-gray-600 lg:max-w-none",
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold text-gray-900",
                                        svg {
                                            aria_hidden: "true",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            "data-slot": "icon",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path { d: "M4.632 3.533A2 2 0 0 1 6.577 2h6.846a2 2 0 0 1 1.945 1.533l1.976 8.234A3.489 3.489 0 0 0 16 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234Z" }
                                            path {
                                                clip_rule: "evenodd",
                                                d: "M4 13a2 2 0 1 0 0 4h12a2 2 0 1 0 0-4H4Zm11.24 2a.75.75 0 0 1 .75-.75H16a.75.75 0 0 1 .75.75v.01a.75.75 0 0 1-.75.75h-.01a.75.75 0 0 1-.75-.75V15Zm-2.25-.75a.75.75 0 0 0-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 0 0 .75-.75V15a.75.75 0 0 0-.75-.75h-.01Z",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        " Zed + Claude AI. "
                                    }
                                    dd { class: "inline",
                                        "Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."
                                    }
                                }
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold text-gray-900",
                                        svg {
                                            aria_hidden: "true",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            "data-slot": "icon",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path { d: "M4.632 3.533A2 2 0 0 1 6.577 2h6.846a2 2 0 0 1 1.945 1.533l1.976 8.234A3.489 3.489 0 0 0 16 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234Z" }
                                            path {
                                                clip_rule: "evenodd",
                                                d: "M4 13a2 2 0 1 0 0 4h12a2 2 0 1 0 0-4H4Zm11.24 2a.75.75 0 0 1 .75-.75H16a.75.75 0 0 1 .75.75v.01a.75.75 0 0 1-.75.75h-.01a.75.75 0 0 1-.75-.75V15Zm-2.25-.75a.75.75 0 0 0-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 0 0 .75-.75V15a.75.75 0 0 0-.75-.75h-.01Z",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        " Dioxus + Tailwind CSS development. "
                                    }
                                    dd { class: "inline",
                                        "Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."
                                    }
                                }
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold text-gray-900",
                                        svg {
                                            aria_hidden: "true",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            "data-slot": "icon",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path {
                                                clip_rule: "evenodd",
                                                d: "M5.5 17a4.5 4.5 0 0 1-1.44-8.765 4.5 4.5 0 0 1 8.302-3.046 3.5 3.5 0 0 1 4.504 4.272A4 4 0 0 1 15 17H5.5Zm3.75-2.75a.75.75 0 0 0 1.5 0V9.66l1.95 2.1a.75.75 0 1 0 1.1-1.02l-3.25-3.5a.75.75 0 0 0-1.1 0l-3.25 3.5a.75.75 0 1 0 1.1 1.02l1.95-2.1v4.59Z",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        " Rocky Linux Podman server. "
                                    }
                                    dd { class: "inline",
                                        "Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."
                                    }
                                }
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold text-gray-900",
                                        svg {
                                            aria_hidden: "true",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            "data-slot": "icon",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            path {
                                                clip_rule: "evenodd",
                                                d: "M10 1a4.5 4.5 0 0 0-4.5 4.5V9H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2h-.5V5.5A4.5 4.5 0 0 0 10 1Zm3 8V5.5a3 3 0 1 0-6 0V9h6Z",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        " Fly.io deployment. "
                                    }
                                    dd { class: "inline",
                                        "Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo."
                                    }
                                }
                            }
                        }
                    }
                    img {
                        alt: "Product screenshot",
                        class: "max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-228 md:-ml-4 lg:-ml-0",

                        src: HEADER_SVG,

                    }
                }
            }
        }
    }
}
