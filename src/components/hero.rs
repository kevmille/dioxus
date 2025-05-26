/// Hero
use dioxus::prelude::*;

const ZED_AVIF: Asset = asset!("/assets/zed.avif");
const ZED_WEBP: Asset = asset!("/assets/zed.webp");
const ZED_JPG: Asset = asset!("/assets/zed.jpg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "overflow-hidden bg-bamboo-50 py-24 sm:py-32 border-b-2 border-forest-700",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2",
                    div { class: "lg:pt-4 lg:pr-8",
                        div { class: "lg:max-w-lg",
                            h2 { class: "text-lg font-semibold text-sakura-500", "Learning Rust." }
                            p { class: "mt-2 text-2xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-4xl",
                                "A Rustacean convert"
                            }
                            p { class: "mt-6 text-lg/8 text-gray-600",
                                "I am rockypod, a fulltime webapp developer. During the day, I develop with Drupal and Twig. In my freetime, I am making the transition to Rust and Dioxus. Below will be the focus of this blog:"
                            }
                            dl { class: "mt-10 max-w-xl space-y-8 text-base/7 text-gray-600 lg:max-w-none",
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold text-gray-900",
                                        svg {
                                            role: "img",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            title { "Zed Industries" }
                                            path { d: "M2.25 1.5a.75.75 0 0 0-.75.75v16.5H0V2.25A2.25 2.25 0 0 1 2.25 0h20.095c1.002 0 1.504 1.212.795 1.92L10.764 14.298h3.486V12.75h1.5v1.922a1.125 1.125 0 0 1-1.125 1.125H9.264l-2.578 2.578h11.689V9h1.5v9.375a1.5 1.5 0 0 1-1.5 1.5H5.185L2.562 22.5H21.75a.75.75 0 0 0 .75-.75V5.25H24v16.5A2.25 2.25 0 0 1 21.75 24H1.655C.653 24 .151 22.788.86 22.08L13.19 9.75H9.75v1.5h-1.5V9.375A1.125 1.125 0 0 1 9.375 8.25h5.314l2.625-2.625H5.625V15h-1.5V5.625a1.5 1.5 0 0 1 1.5-1.5h13.19L21.438 1.5z" }
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
                                            role: "img",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            title { "Tailwind CSS" }
                                            path { d: "M12.001,4.8c-3.2,0-5.2,1.6-6,4.8c1.2-1.6,2.6-2.2,4.2-1.8c0.913,0.228,1.565,0.89,2.288,1.624 C13.666,10.618,15.027,12,18.001,12c3.2,0,5.2-1.6,6-4.8c-1.2,1.6-2.6,2.2-4.2,1.8c-0.913-0.228-1.565-0.89-2.288-1.624 C16.337,6.182,14.976,4.8,12.001,4.8z M6.001,12c-3.2,0-5.2,1.6-6,4.8c1.2-1.6,2.6-2.2,4.2-1.8c0.913,0.228,1.565,0.89,2.288,1.624 c1.177,1.194,2.538,2.576,5.512,2.576c3.2,0,5.2-1.6,6-4.8c-1.2,1.6-2.6,2.2-4.2,1.8c-0.913-0.228-1.565-0.89-2.288-1.624 C10.337,13.382,8.976,12,6.001,12z" }
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
                                            role: "img",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            title { "Rocky Linux" }
                                            path { d: "M23.332 15.957c.433-1.239.668-2.57.668-3.957 0-6.627-5.373-12-12-12S0 5.373 0 12c0 3.28 1.315 6.251 3.447 8.417L15.62 8.245l3.005 3.005zm-2.192 3.819l-5.52-5.52L6.975 22.9c1.528.706 3.23 1.1 5.025 1.1 3.661 0 6.94-1.64 9.14-4.224z" }
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
                                            role: "img",
                                            class: "absolute top-1 left-1 size-5 text-indigo-600",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            title { "Fly.io" }
                                            path { d: "M11.987 0c-2.45-.01-5.002.925-6.541 2.897-1.17 1.502-1.664 3.474-1.49 5.356.29 2.112 1.476 3.96 2.676 5.672a41.5 41.5 0 0 0 4.216 4.831c-1.063.832-1.943 2.286-1.357 3.644.821 2.32 4.665 2.05 5.122-.372.39-1.288-.694-2.533-1.428-3.309 2.388-2.431 4.706-5.036 6.17-8.145.595-1.32.902-2.802.614-4.24-.28-2.341-1.823-4.473-3.967-5.46C14.76.266 13.364.016 11.987 0m-.236 1.577v15.534C9.881 13.483 7.724 9.266 8.73 5.069c.35-1.539 1.253-3.309 3.02-3.492m1.996.04c1.534.357 3.031 1.096 3.906 2.48 1.3 1.93 1.318 4.55.1 6.521-1.268 2.395-3.06 4.463-4.916 6.415 1.472-2.974 3.074-6.106 3.182-9.5-.043-2.08-.438-4.612-2.272-5.916M11.97 20.103c.848.342 1.597 1.983.153 2.173-.664.15-1.367-.599-.995-1.222.213-.355.488-.73.842-.95" }
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
                    div { class: "sm:ml-16 md:ml-0",
                        picture {
                            source {
                                "media": "(min-width: 1px)",
                                "srcset": "{ZED_AVIF}",
                                "type": "image/avif"
                            }
                            source {
                                "media": "(min-width: 1px)",
                                "srcset": "{ZED_WEBP}",
                                "type": "image/webp"
                            }
                            img {
                                alt: "Zed screenshot",
                                class: "max-w-none rounded-xl shadow-xl ring-2 ring-sakura-500 sm:w-228 md:-ml-4 lg:-ml-0",
                                src: "{ZED_JPG}"
                            }
                        }
                    }
                }
            }
        }
    }
}
