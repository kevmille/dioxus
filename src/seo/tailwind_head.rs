use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn TailwindHead() -> Element {
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
  "@type": ["WebPage", "TechArticle"],
  "name": "Dioxus + Tailwind | cryptonezumi.com",
  "headline": "Setting up Tailwind CSS 4 with Dioxus",
  "description": "A technical guide on how Tailwind 4 requires changes to the initial setup process for Dioxus web applications.",
  "url": "https://cryptonezumi.com/dioxus-tailwind",
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
  },
  "about": [
    {
      "@type": "SoftwareApplication",
      "name": "Dioxus",
      "applicationCategory": "Web Framework"
    },
    {
      "@type": "SoftwareApplication",
      "name": "Tailwind CSS",
      "applicationCategory": "CSS Framework"
    }
  ],
  "programmingLanguage": "Rust"
}"#;
                        script.set_text_content(Some(json_ld));
                        head.append_child(&script).ok();
                        web_sys::console::log_1(
                            &"Tailwind tutorial JSON-LD added successfully".into(),
                        );
                    }
                }
            }
        }
    });

    rsx! {
        head {
            dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
            dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
            dioxus::document::Link { rel: "canonical", href: "https://cryptonezumi.com/dioxus-tailwind" }
            dioxus::document::Meta { charset: "utf-8" }
            dioxus::document::Meta { name: "robots", content: "index, follow" }
            dioxus::document::Meta { name: "theme-color", content: "#FBE9E9" }
            dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            dioxus::document::Title { "Dioxus + Tailwind | cryptonezumi.com" }
            dioxus::document::Meta { name: "description", content: "A technical guide on how Tailwind 4 requires changes to the initial setup process for Dioxus web applications." }
            dioxus::document::Meta { name: "keywords", content: "Dioxus, Tailwind CSS, Rust, Web Framework, CSS Framework, Tutorial" }

            // --- Open Graph tags ---
            dioxus::document::Meta { property: "og:title", content: "Dioxus + Tailwind | cryptonezumi.com" }
            dioxus::document::Meta { property: "og:type", content: "article" }
            dioxus::document::Meta { property: "og:url", content: "https://cryptonezumi.com/dioxus-tailwind" }
            dioxus::document::Meta { property: "og:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { property: "og:description", content: "A technical guide on how Tailwind 4 requires changes to the initial setup process for Dioxus web applications." }
            dioxus::document::Meta { property: "og:site_name", content: "cryptonezumi.com" }
            dioxus::document::Meta { property: "article:author", content: "Kevin Matsunaga" }
            dioxus::document::Meta { property: "article:section", content: "Technology" }
            dioxus::document::Meta { property: "article:tag", content: "Dioxus" }
            dioxus::document::Meta { property: "article:tag", content: "Tailwind CSS" }
            dioxus::document::Meta { property: "article:tag", content: "Rust" }

            // --- Twitter Card tags ---
            dioxus::document::Meta { name: "twitter:card", content: "summary_large_image" }
            dioxus::document::Meta { name: "twitter:title", content: "Dioxus + Tailwind | cryptonezumi.com" }
            dioxus::document::Meta { name: "twitter:description", content: "A technical guide on how Tailwind 4 requires changes to the initial setup process for Dioxus web applications." }
            dioxus::document::Meta { name: "twitter:image", content: "https://cryptonezumi.com/assets/og-image.avif" }
            dioxus::document::Meta { name: "twitter:site", content: "@MatsunagaKevin" }
            dioxus::document::Meta { name: "twitter:creator", content: "@MatsunagaKevin" }

            // --- Analytics ---
            dioxus::document::Script {
                "src": "https://stats.rockypodno.de/script.js",
                "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
            }
        }
    }
}
