use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn TailwindHead() -> Element {
    use_effect(move || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html_element) = document.document_element() {
                    html_element.set_attribute("lang", "en").ok();
                }
            }
        }
    });

    // JSON-LD structured data as a string
    let json_ld = r#"
    {
      "@context": "https://schema.org",
      "@type": "WebPage",
      "name": "Dioxus + Tailwind | cryptonezumi.com",
      "description": "Tailwind 4 requires changes to the initial setup process for Dioxus.",
      "url": "https://cryptonezumi.com/dioxus-tailwind",
      "sameAs": "https://rockypod.com/dioxus-tailwind"
    }
    "#;

    rsx! {
        head {
            dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
            dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
            dioxus::document::Meta { charset: "utf-8" }
            dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            dioxus::document::Title { "Dioxus + Tailwind | cryptonezumi.com" }
            dioxus::document::Meta { name: "description", content: "Tailwind 4 requires changes to the initial setup process for Dioxus." }
            // --- Open Graph tags ---
            dioxus::document::Meta { property: "og:title", content: "Dioxus + Tailwind | cryptonezumi.com" }
            dioxus::document::Meta { property: "og:type", content: "website" }
            dioxus::document::Meta { property: "og:url", content: "https://cryptonezumi.com/dioxus-tailwind" }
            dioxus::document::Meta { property: "og:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { property: "og:description", content: "Tailwind 4 requires changes to the initial setup process for Dioxus." }
            dioxus::document::Meta { property: "og:site_name", content: "cryptonezumi.com" }
            // --- Twitter Card tags (optional, recommended) ---
            dioxus::document::Meta { name: "twitter:card", content: "summary_large_image" }
            dioxus::document::Meta { name: "twitter:title", content: "Home | cryptonezumi.com" }
            dioxus::document::Meta { name: "twitter:description", content: "Tailwind 4 requires changes to the initial setup process for Dioxus." }
            dioxus::document::Meta { name: "twitter:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { name: "twitter:site", content: "@MatsunagaKevin" }
            // --- Analytics and JSON-LD ---
            dioxus::document::Script {
                "src": "https://stats.rockypodno.de/script.js",
                "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
            }
            dioxus::document::Script {
                r#"type"#: "application/ld+json",
                dangerous_inner_html: "{json_ld}",
            }
        }
    }
}
