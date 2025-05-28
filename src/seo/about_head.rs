use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn AboutHead() -> Element {
    use_effect(move || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html_element) = document.document_element() {
                    html_element.set_attribute("lang", "en").ok();
                }
            }
        }
    });
    rsx! {
            head {
                dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
                dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
                dioxus::document::Meta { charset: "utf-8" }
                dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
                dioxus::document::Title { "About | cryptonezumi.com" }
                dioxus::document::Meta { name: "description", content: "About Rockypod, the creator of cryptonezumi.com." }
                dioxus::document::Script {
                    "src": "https://stats.rockypodno.de/script.js",
                    "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
                }
        }
    }
}
