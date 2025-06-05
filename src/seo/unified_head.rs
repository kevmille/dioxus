use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PageType {
    Home,
    About,
    DioxusTailwind,
}

#[component]
pub fn UnifiedHead(page_type: PageType) -> Element {
    // Ensure lang attribute is set on html element
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html_element) = document.document_element() {
                    let _ = html_element.set_attribute("lang", "en");
                }
            }
        }
    });
    let (title, description, url, og_type, keywords) = match page_type {
        PageType::Home => (
            "Home | cryptonezumi.com",
            "During the day, I develop with Drupal and Twig. At night, I focus on Rust and Dioxus.",
            "https://cryptonezumi.com",
            "website",
            None,
        ),
        PageType::About => (
            "About | cryptonezumi.com",
            "About Kevin Matsunaga (Rockypod), a developer who works with Drupal and Twig by day, Rust and Dioxus by night.",
            "https://cryptonezumi.com/about",
            "website",
            None,
        ),
        PageType::DioxusTailwind => (
            "Dioxus + Tailwind | cryptonezumi.com",
            "A technical guide on how Tailwind 4 requires changes to the initial setup process for Dioxus web applications.",
            "https://cryptonezumi.com/dioxus-tailwind",
            "article",
            Some("Dioxus, Tailwind CSS, Rust, Web Framework, CSS Framework, Tutorial"),
        ),
    };

    let json_ld = match page_type {
        PageType::Home => r#"{
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
}"#,
        PageType::About => r#"{
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
}"#,
        PageType::DioxusTailwind => r#"{
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
}"#,
    };

    rsx! {
        head {
            // Basic meta tags
            dioxus::document::Meta { charset: "utf-8" }
            dioxus::document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            dioxus::document::Meta { name: "robots", content: "index, follow" }
            dioxus::document::Meta { name: "theme-color", content: "#FBE9E9" }
            
            // Title and description
            dioxus::document::Title { "{title}" }
            dioxus::document::Meta { name: "description", content: "{description}" }
            
            // Keywords (only for certain pages)
            if let Some(kw) = keywords {
                dioxus::document::Meta { name: "keywords", content: "{kw}" }
            }
            
            // Links
            dioxus::document::Link { rel: "icon", href: "/favicon.ico" }
            dioxus::document::Link { rel: "stylesheet", href: "/tailwind.css" }
            dioxus::document::Link { rel: "canonical", href: "{url}" }

            // Open Graph tags
            dioxus::document::Meta { property: "og:title", content: "{title}" }
            dioxus::document::Meta { property: "og:type", content: "{og_type}" }
            dioxus::document::Meta { property: "og:url", content: "{url}" }
            dioxus::document::Meta { property: "og:locale", content: "en_US" }
            dioxus::document::Meta { property: "og:image", content: "https://res.cloudinary.com/shinkirin/image/upload/v1748553775/rockypod/h1cd1gz4ycs04nnc0wzi.avif" }
            dioxus::document::Meta { 
                property: "og:image:alt", 
                content: match page_type {
                    PageType::Home => "cryptonezumi.com website logo",
                    PageType::About => "About Kevin Matsunaga - cryptonezumi.com",
                    PageType::DioxusTailwind => "Dioxus + Tailwind CSS Tutorial - cryptonezumi.com",
                }
            }
            dioxus::document::Meta { property: "og:image:width", content: "1200" }
            dioxus::document::Meta { property: "og:image:height", content: "630" }
            dioxus::document::Meta { property: "og:image:type", content: "image/avif" }
            dioxus::document::Meta { property: "og:description", content: "{description}" }
            dioxus::document::Meta { property: "og:site_name", content: "cryptonezumi.com" }

            // Article-specific Open Graph tags
            if page_type == PageType::DioxusTailwind {
                dioxus::document::Meta { property: "article:author", content: "Kevin Matsunaga" }
                dioxus::document::Meta { property: "article:section", content: "Technology" }
                dioxus::document::Meta { property: "article:tag", content: "Dioxus" }
                dioxus::document::Meta { property: "article:tag", content: "Tailwind CSS" }
                dioxus::document::Meta { property: "article:tag", content: "Rust" }
            }

            // Twitter Card tags
            dioxus::document::Meta { name: "twitter:card", content: "summary_large_image" }
            dioxus::document::Meta { property: "twitter:domain", content: "cryptonezumi.com" }
            dioxus::document::Meta { property: "twitter:url", content: "{url}" }
            dioxus::document::Meta { name: "twitter:title", content: "{title}" }
            dioxus::document::Meta { name: "twitter:description", content: "{description}" }
            dioxus::document::Meta { name: "twitter:image", content: "https://res.cloudinary.com/shinkirin/image/upload/v1748553775/rockypod/h1cd1gz4ycs04nnc0wzi.avif" }
            dioxus::document::Meta { 
                name: "twitter:image:alt", 
                content: match page_type {
                    PageType::Home => "cryptonezumi.com website logo",
                    PageType::About => "About Kevin Matsunaga - cryptonezumi.com", 
                    PageType::DioxusTailwind => "Dioxus + Tailwind CSS Tutorial - cryptonezumi.com",
                }
            }
            dioxus::document::Meta { name: "twitter:site", content: "@MatsunagaKevin" }
            
            // Twitter creator (only for article pages)
            if page_type == PageType::DioxusTailwind {
                dioxus::document::Meta { name: "twitter:creator", content: "@MatsunagaKevin" }
            }

            // JSON-LD structured data
            dioxus::document::Script {
                r#type: "application/ld+json",
                "{json_ld}"
            }

            // Analytics
            dioxus::document::Script {
                src: "https://stats.rockypodno.de/script.js",
                "data-website-id": "a2c48057-24d6-4c9a-b664-815071c1e3ff"
            }
        }
    }
}