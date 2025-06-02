use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn AboutHead() -> Element {
    use_effect(move || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Set lang attribute
                if let Some(html_element) = document.document_element() {
                    html_element.set_attribute("lang", "en").ok();
                }

                // Inject JSON-LD script
                if let Some(head) = document.head() {
                    if let Ok(script) = document.create_element("script") {
                        script.set_attribute("type", "application/ld+json").ok();
                        let json_ld = r#"{
  "@context": "https://schema.org",
  "@type": "WebPage",
  "name": "About | cryptonezumi.com",
  "description": "About Kevin Matsunaga (Rockypod), a developer who works with Drupal and Twig by day, Rust and Dioxus by night.",
  "url": "https://cryptonezumi.com/about",
  "datePublished": "2025-05-25",
  "dateModified": "2025-06-01",
  "isPartOf": {
    "@type": "WebSite",
    "name": "cryptonezumi.com",
    "url": "https://cryptonezumi.com"
  },
  "author": {
    "@type": "Person",
    "name": "Kevin Matsunaga",
    "sameAs": "https://twitter.com/MatsunagaKevin"
  }
}"#;
                        script.set_text_content(Some(json_ld));
                        head.append_child(&script).ok();
                        web_sys::console::log_1(&"About JSON-LD added successfully".into());
                    }
                }
            }
        }
    });

    rsx! {
        head {
            dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
            dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
            dioxus::document::Link { rel: "canonical", href: "https://cryptonezumi.com/about" }
            dioxus::document::Meta { charset: "utf-8" }
            dioxus::document::Meta { name: "robots", content: "index, follow" }
            dioxus::document::Meta { name: "theme-color", content: "#FBE9E9" }
            dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            dioxus::document::Title { "About | cryptonezumi.com" }
            dioxus::document::Meta { name: "description", content: "About Kevin Matsunaga (Rockypod), a developer who works with Drupal and Twig by day, Rust and Dioxus by night." }

            // --- Open Graph tags ---
            dioxus::document::Meta { property: "og:title", content: "About | cryptonezumi.com" }
            dioxus::document::Meta { property: "og:type", content: "website" }
            dioxus::document::Meta { property: "og:url", content: "https://cryptonezumi.com/about" }
            dioxus::document::Meta { property: "og:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { property: "og:description", content: "About Kevin Matsunaga (Rockypod), a developer who works with Drupal and Twig by day, Rust and Dioxus by night." }
            dioxus::document::Meta { property: "og:site_name", content: "cryptonezumi.com" }

            // --- Twitter Card tags ---
            dioxus::document::Meta { name: "twitter:card", content: "summary_large_image" }
            dioxus::document::Meta { name: "twitter:title", content: "About | cryptonezumi.com" }
            dioxus::document::Meta { name: "twitter:description", content: "About Kevin Matsunaga (Rockypod), a developer who works with Drupal and Twig by day, Rust and Dioxus by night." }
            dioxus::document::Meta { name: "twitter:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { name: "twitter:site", content: "@MatsunagaKevin" }

            // --- Analytics ---
            dioxus::document::Script {
                "src": "https://stats.rockypodno.de/script.js",
                "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
            }
        }
    }
}
