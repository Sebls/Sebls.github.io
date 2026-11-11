# Technical plan

## Selected stack and scope

Preserve the owner's original technical selection: Rust, Yew functional components/hooks, WebAssembly, Cargo, Trunk, Yew Router, and static GitHub Pages hosting. Rust owns frontend logic where practical; Trunk's generated browser loader is expected. No backend or runtime content API is required.

**Proposed default:** plain CSS with custom properties. Tailwind was mentioned as an option, not a requirement; avoid a second build pipeline unless it has a concrete benefit. Choose compatible stable versions and record prerequisites when implementation begins. Current dependency selections: Yew 0.23, Yew Router 0.20, pulldown-cmark 0.13, Ammonia 4; exact versions are locked in Cargo.lock.

## Pages versus rendering architecture

The product needs separate Home, About, Experience, Work, Hobbies, Gallery, Blog, and article views. Static hosting means serving built files; it does not automatically mean each view has pre-rendered HTML.

A standard Yew client app uses one HTML entry point and renders routed views in the browser. That is a multi-page user experience implemented as a single-page application. It requires JavaScript/WASM to display its main content and does not provide pre-rendered per-article metadata or guaranteed search/social previews.

**Resolved by the owner on 11 November 2026:** generate separate HTML documents. Hash routing was proposed and declined.

One Rust crate shares Yew components between a native build-time renderer (`src/bin/render.rs`) and the WASM entry. Trunk builds the browser assets, then the native renderer uses Yew `ServerRenderer` to generate `/index.html`, `/about/index.html`, `/experience/index.html`, `/work/index.html`, `/hobbies/index.html`, `/gallery/index.html`, `/blog/index.html`, and one `/blog/<slug>/index.html` for each published article. A `404.html` supplies the unknown-page view. No runtime server or generator service is deployed.

Yew Router defines and recognizes routes. An in-memory router renders the same component tree for generation and browser hydration. Page navigation deliberately uses normal document links, not client history interception. Trailing-slash URLs map directly to directories on GitHub Pages. Native fragment anchors serve the current-page index without conflicting with routes. Refresh, back/forward, and no-JavaScript reading use normal browser behavior.

Yew hydrates the pre-rendered HTML to add menu state and the cat interaction. Native build dependencies are Tokio (Yew renderer runtime), pulldown-cmark (Markdown), and Ammonia (sanitization). Draft metadata and bodies are excluded by `build.rs` before either target is compiled. Missing published bodies fail the build. Content, page titles, descriptions, and Open Graph text are present in each HTML document. The production hostname is `https://sebls.github.io/`; canonical URLs, social images, and a sitemap are still deferred improvements; no search ranking or rich-preview guarantees are implied.

This modest build-time renderer is the additional work required by the selected separate-document architecture, not a separate content platform. The owner explicitly authorized this rendering choice. Official Yew SSR documentation was checked when implementing; hydration is marked experimental upstream.

## Frontend structure

Use one Rust crate. Keep files beside the feature they support; create directories only when needed.

```text
AGENTS.md
README.md
docs/                         # brief, source content, references, prompts
Cargo.toml
Cargo.lock
Trunk.toml
index.html                    # Trunk entry and static head metadata
src/
  main.rs                     # mount application
  app.rs                      # shared shell and route switch
  routes.rs                   # route definitions
  pages/                      # home, about, experience, work, hobbies, gallery, blog, article, not_found
  components/                 # sidebar, page_index; shared components only when reused
  effects/                    # isolated cat/black-hole component and rendering lifecycle
  content/                    # small typed models and loading/selection helpers
content/
  portfolio.json              # curated public profile, experience, work data
  gallery.json                # supplied image metadata; initially empty
  posts.json                  # article metadata and selected asset manifests
  posts/                      # local Markdown article bodies when supplied
styles/
  tokens.css
  global.css
  layout.css
  components.css
assets/
  images/                     # cat artwork and selected photos
  fonts/                      # only authorized local fonts
.github/workflows/pages.yml   # automatic static build/deploy workflow
```

The tree above is the organizational guide; small pages and components are grouped in module files. The implementation also contains `build.rs`, `src/bin/render.rs`, and `scripts/build.sh` for static generation. Merge tiny modules when it improves clarity; split styles only as they grow. Do not add repositories, services, dependency injection, global state libraries, a component package, or a custom CMS.

Load curated content locally, preferably at build time for this small site. A small manifest mapping article metadata to bundled Markdown is enough initially; document the one step needed to register a new article. Use a maintained Markdown renderer rather than inventing a parser. No MDX or Node framework is needed. Bundled content grows with the blog; reconsider loading only if measured size justifies it.

## GitHub Pages deployment contract

- The owner confirmed `Sebls/Sebls.github.io` on 11 November 2026 and authorized automatic publication. Production URL: `https://sebls.github.io/`; build base path: `/`.
- Configure Trunk's public URL for that location and use consistent base-aware asset URLs. Avoid root-absolute asset paths that fail under a repository subdirectory.
- Serve the generated release output, including the WASM and browser loader, as static files. Do not deploy the repository root or documentation/source files.
- Test root and repository-prefix hosting, routed deep links, refresh, back/forward, unknown routes, images, fonts, and article loading. Never assume the dev server reproduces GitHub Pages fallback behavior.
- The Pages workflow deploys automatically on pushes to `main` and supports manual reruns. Both triggers check and build the root-hosted site before deployment; GitHub Pages uses the Actions publishing source.
- CI uses the checksum-verified prebuilt Trunk 0.22.1 Linux x86-64 release and caches Cargo dependencies/compiled dependencies. Superseded branch builds are canceled; Pages publication is serialized without canceling an active deployment. See the development guide for cache and version-update details.
- Generate page-specific titles, descriptions, and Open Graph text in every document; confirm final absolute metadata URLs before deployment.

## Verification after implementation begins

The following are expected checks, not commands already available or run in this documentation-only repository:

```sh
cargo fmt --check
cargo check --target wasm32-unknown-unknown
cargo clippy --target wasm32-unknown-unknown -- -D warnings
trunk build --release
./scripts/build.sh /sebls-portfolio/
```

Document actual setup and commands in the [development guide](development.md), linked from the root README. Add targeted tests for meaningful logic such as draft exclusion or route parsing where needed; avoid a large test harness for static presentation.

Manually verify desktop and small-screen layouts, keyboard menu/focus handling, route and index links, empty gallery/blog, one real article when supplied, and the effect's entry/exit/resize/unmount behavior. Check reduced motion, touch fallback, no horizontal overflow, and no effect leakage onto siblings. Confirm a failed effect leaves the illustration and content usable. Inspect the release bundle and loading behavior rather than claiming performance from architecture alone.

## Official references

- [Yew deployment](https://yew.rs/docs/next/more/deployment) explains client routing and host fallback requirements. Verify the documentation for the stable version chosen at implementation time.
- [Trunk public URLs and paths](https://trunk-rs.github.io/trunk/guide/advanced/paths.html) describes asset URL prefixes.
- [GitHub Pages site setup](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-github-pages-site) describes static publishing.
