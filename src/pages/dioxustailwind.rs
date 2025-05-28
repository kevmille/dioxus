/// About Content component
use dioxus::prelude::*;

pub fn DioxusTailwindMain() -> Element {
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
    }
}
