use dioxus::prelude::*;
use web_sys::window;

const OG_IMAGE: Asset = asset!("/assets/og-image.avif");

#[component]
pub fn HomeHead() -> Element {
    // Use use_resource instead of use_effect for better reliability
    let _json_ld_injection = use_resource(move || async move {
        gloo_timers::future::TimeoutFuture::new(100).await; // Small delay to ensure DOM is ready

        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Set lang attribute
                if let Some(html_element) = document.document_element() {
                    html_element.set_attribute("lang", "en").ok();
                }

                // Check if JSON-LD already exists to prevent duplicates
                if document
                    .query_selector("script[type='application/ld+json'][data-site='cryptonezumi']")
                    .ok()
                    .flatten()
                    .is_none()
                {
                    if let Some(head) = document.head() {
                        if let Ok(script) = document.create_element("script") {
                            script.set_attribute("type", "application/ld+json").ok();
                            script.set_attribute("data-site", "cryptonezumi").ok(); // Identifier
                            let json_ld = r#"{
  "@context": "https://schema.org",
  "@type": "WebSite",
  "name": "cryptonezumi.com",
  "url": "https://cryptonezumi.com",
  "datePublished": "2025-05-25",
  "dateModified": "2025-06-01",
  "sameAs": "https://rockypod.com",
  "author": {
    "@type": "Person",
    "name": "Kevin Matsunaga",
    "sameAs": "https://twitter.com/MatsunagaKevin"
  }
}"#;
                            script.set_text_content(Some(json_ld));
                            if let Err(_) = head.append_child(&script) {
                                web_sys::console::log_1(&"Failed to append JSON-LD script".into());
                            } else {
                                web_sys::console::log_1(
                                    &"JSON-LD script successfully added".into(),
                                );
                            }
                        }
                    }
                }
            }
        }
    });

    rsx! {
        head {
            dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
            dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
            dioxus::document::Link { rel: "canonical", href: "https://cryptonezumi.com" }
            dioxus::document::Meta { charset: "utf-8" }
            dioxus::document::Meta { name: "robots", content: "index, follow" }
            dioxus::document::Meta { name: "theme-color", content: "#FBE9E9" }
            dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            dioxus::document::Title { "Home | cryptonezumi.com" }
            dioxus::document::Meta { name: "description", content: "During the day, I develop with Drupal and Twig. At night, I focus on Rust and Dioxus." }

            // --- Open Graph tags ---
            dioxus::document::Meta { property: "og:title", content: "Home | cryptonezumi.com" }
            dioxus::document::Meta { property: "og:type", content: "website" }
            dioxus::document::Meta { property: "og:url", content: "https://cryptonezumi.com" }
            dioxus::document::Meta { property: "og:image", content: "{OG_IMAGE}" }
            dioxus::document::Meta { property: "og:description", content: "During the day, I develop with Drupal and Twig. At night, I focus on Rust and Dioxus." }
            dioxus::document::Meta { property: "og:site_name", content: "cryptonezumi.com" }

            // --- Twitter Card tags ---
            dioxus::document::Meta { name: "twitter:card", content: "summary_large_image" }
            dioxus::document::Meta { name: "twitter:title", content: "Home | cryptonezumi.com" }
            dioxus::document::Meta { name: "twitter:description", content: "During the day, I develop with Drupal and Twig. At night, I focus on Rust and Dioxus." }
            dioxus::document::Meta { name: "twitter:image", content: "{OG_IMAGE}" }
            dioxus::document::Meta { name: "twitter:site", content: "@MatsunagaKevin" }

            // --- Analytics ---
            dioxus::document::Script {
                "src": "https://stats.rockypodno.de/script.js",
                "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
            }
        }
    }
}
