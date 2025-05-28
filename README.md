# Dioxus + Tailwind CSS

## Info

My first Rust and Dioxus Project.

- URL: [https://cryptonezumi.com](https://cryptonezumi.com)
- Repo: [https://github.com/rockypod/cryptonezumi.com](https://github.com/rockypod/cryptonezumi.com)
- Fly Machines: [https://toki-nezu.fly.dev/](https://toki-nezu.fly.dev/)

## Tools and Resources
- [Rust](https://www.rust-lang.org/)
- [Dioxus](https://dioxuslabs.com) Rust framework
  - [HTML to Rust Converter](https://wheregmis.github.io/dioxus_html_rsx/)
  - [dioxus-free-icons](https://github.com/dioxus-community/dioxus-free-icons)
- [Tailwind CSS](https://tailwindcss.com) framework
  - [Tailwind Plus](https://tailwindcss.com/plus) components
- [Zed](https://zed.dev/)
  - [Claude API](https://www.anthropic.com/api) - AI Agent
  - [Perplexity AI](https://www.perplexity.ai/) - AI Agent
  - [Google Gemini Pro](https://gemini.google.com) - Generated Rockypod avatar
- [Unsplash](https://unsplash.com/)
- [Github](https://github.com/users/rockypod/)
- [Fly.io](https://fly.io)

## Folder organization

```
toki/
├─ assets/ # Any assets that are used by the app should be placed here including input.css and tailwind.css
├─ src/
│  ├─ main.rs # main.rs is the entry point to the application and currently contains all components for the app
│  ├─ components # all reusable components
│  ├─ pages # all page content
│  ├─ seo # all metatags and SEO related components
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
├─ Dioxus.toml # The Dioxus.toml file defines the dependencies and feature flags for your project
├─ Dockerfile # Dockerfile for building and running the application with Fly.io
├─ package.json # Scripts to run development server and build Tailwind CSS styles
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Install concurrently: https://www.npmjs.com/package/concurrently
    - This will run Tailwind CSS CLI at the same time as dx serve --platform web
4. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
concurrently \"npm run css:watch\" \"dx serve --platform web\"
```
or
```bash
npm run dev
```

### Future features
- Will add decoupled Drupal to fetch data via JSON:api
- Axum API server: https://docs.rs/axum/latest/axum/
- Pull data from Coinbase API: https://docs.cloud.coinbase.com/exchange/docs/api-overview
- Deployment to Podman server

# cryptonezumi.com
