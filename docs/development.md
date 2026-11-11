# Development guide

Sebastian Lopez Silva's portfolio, built with Rust, Yew 0.23, Yew Router 0.20, Cargo, Trunk 0.22.1, and plain CSS. **Automatic GitHub Pages deployment from `main`.**

The owner selected **separately generated HTML pages** on 11 November 2026. Every main page and published article has its own HTML document and metadata. A native Yew renderer runs at build time, and the browser hydrates the same components for interaction. No backend is deployed. Normal page navigation and section anchors work without JavaScript; the mobile navigation remains expanded when JavaScript is disabled. Yew Router handles route definitions and recognition with a memory router, rather than intercepting document navigation.

## Build and preview

Install stable Rust with `rustfmt` and `clippy`, then:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk --version 0.22.1
./scripts/build.sh /
python3 -m http.server 8080 --directory dist
```

Open [the local site](http://localhost:8080). `trunk build --release` alone bundles the browser assets; **the complete build requires `scripts/build.sh`** to add static documents. `trunk serve` is not the multi-document preview.

For a GitHub project URL, build with the repository prefix:

```sh
./scripts/build.sh /sebls-portfolio/
```

Serve the resulting `dist` at `/sebls-portfolio/` when previewing that variant. All links and generated assets include the chosen base. Use `/` for an account site or custom domain. The production site is `https://sebls.github.io/`, built with `/` as its base path. Do not deploy the repository root.

## Content

- `content/portfolio.json`: curated experience, projects, and research.
- `src/pages/mod.rs`: page introductions, About, and empty states.
- `content/gallery.json`: photo metadata; initially empty. Only listed images are copied.
- `content/posts.json` and `content/posts/<slug>.md`: local articles; initially empty. Only explicit `draft: false` records are compiled or generated. A missing published body fails the build.
- See [the content guide](../docs/content/README.md) for exact fields and authoring instructions.

Home's cat, five plants, Colombian flag, and sleep marks share the `PointArtwork` component in `src/effects/mod.rs`. Each keeps its accessible SVG fallback and uses the same local Rust Canvas particle renderer on mouse interaction. A 40 CSS-pixel influence radius repels points around a 14px core matching the panel background; points ease back on exit. Artwork dimensions and dot radius come from its matching JSON, with contain sizing and 2D CSS rotation accounted for in pointer coordinates. Frames run only until movement settles; resize, blur, visibility changes, and reduced-motion changes reset the effect. Coarse pointers, no-hover devices, reduced motion, unavailable Canvas, and no JavaScript retain the SVG. At widths up to 720px, only the cat is visible; the flag, plants, and sleep marks are hidden. Authoring JSON is compiled into WASM rather than copied as public files.

To add another point illustration, supply matching `NAME-points.svg` and `NAME-points.json` (width, height, radius, and `[x, y, r, g, b]` points), add an `image(...)` entry in `Cat` with a positioning class, and add a Trunk copy-file entry for the SVG in `index.html`. The shared renderer supplies interaction and cleanup without new animation code. Keep positioning wrappers untransformed; an artwork can use a centered 2D CSS transform. The 720px mobile rule automatically hides every point artwork except `.cat-art`.
To regenerate both point assets on macOS with Swift installed, run `swift scripts/generate-cat-points.swift` from the repository root, then rebuild normally. This optional authoring tool uses [AppKit's bitmap reader](https://developer.apple.com/documentation/appkit/nsbitmapimagerep/init%28data%3A%29); Swift is not needed for normal builds or in the browser. Sampling uses a 9px grid in the 1254px source, 6.4px circle radii, and omits pixels below 90% opacity. Each JSON point is `[x, y, r, g, b]`; array order matches SVG circles. Coordinates use a top-left origin.

Original gallery photographs with captions/alt text and articles are still needed. No CV download or email link is displayed because none was supplied. The full CV and design reference JSON never enter the public output.

Home also includes five point plants in a horizontal row at the lower left of the cat surface, ordered by filename (`plant-1` through `plant-5`). To resize them all, change `--plant-size:63px` in `.plant-cluster` in `styles/global.css`; the row shrinks together when space is limited. `assets/images/plant-1-points.svg` through `plant-5-points.svg` use silhouette-aware coarse sampling with the desktop cat’s 2.225 CSS-pixel nominal pitch and 3.164px dot diameter. Matching `plant-N-points.json` files preserve `[x, y, r, g, b]` coordinates and color used by the shared interaction. The supplied `plant-N.png` originals and background-removed `plant-N-cutout.png` files remain authoring assets. Only the five SVGs are copied as standalone artwork files. Regenerate SVG/JSON with `swift scripts/generate-plant-points.swift` on macOS; each cutout is trimmed and its longest edge normalized to 1,254 units including padding.

## Verification

```sh
cargo fmt --check
cargo check --locked --target wasm32-unknown-unknown
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
cargo clippy --locked --all-targets -- -D warnings
./scripts/build.sh /
./scripts/build.sh /sebls-portfolio/
```

The native generator emits a friendly `404.html`; GitHub Pages supplies the actual unknown-path HTTP status. Article metadata is static. Canonical URLs and a sitemap remain future improvements; social images require supplied assets. Yew hydration is experimental upstream; the generated content remains readable if WASM fails.

See [the verification record](../docs/verification.md) for checks actually run, browser findings, and remaining limitations.

## Deployment preparation

The owner authorized automatic publication on 11 November 2026. `.github/workflows/pages.yml` checks, builds, verifies, and deploys the site on every push to `main`; it can also be run manually. Production always uses the root base path `/` for [sebls.github.io](https://sebls.github.io/). GitHub Pages uses GitHub Actions as its publishing source. Only `dist/` is uploaded as the site artifact. Failed checks prevent deployment.

## Project documentation

Start with [AGENTS.md](../AGENTS.md), [brief](../docs/brief.md), [technical plan](../docs/technical.md), [content guide](../docs/content/README.md), and [design guide](../docs/design.md). The [source CV](../docs/content/source-cv.md) is preserved verbatim. [Archived specifications](../docs/archive/README.md) are historical context.

The source CV is local, private authoring material and is excluded from Git by `.gitignore` until repository visibility is decided. It will not be present in a fresh clone; the curated public content is sufficient to build the site. Keep `Cargo.lock` tracked; build output, local toolchains, environment files, and editor caches are ignored. The repository uses `main` and Commitizen-style commit messages (`type(scope): description`). Its `origin` remote is `https://github.com/Sebls/Sebls.github.io.git`. Pushing `main` triggers production deployment.

The Colombian flag above the left end of the plant row uses `assets/images/colombia-flag-points.svg` (363 circles), with matching authoring JSON and the supplied transparent `colombia-flag.png` preserved alongside it. Regenerate with `swift scripts/generate-flag-points.swift` on macOS. The SVG is copied into the public build and the matching JSON supplies the shared particle effect.

## Illustration density

Regenerate all five plants and the flag with `swift scripts/generate-decoration-points.swift` on macOS (no extra packages). The existing plant/flag generator commands delegate to this shared sampler. It retains each plant’s largest opaque source component, groups opaque pixels into cells, places a dot at each occupied cell’s centroid, and averages its opaque colors. Sparse cells preserve fine stems and edges. Centers are kept inside the viewBox by the dot radius. Source cutouts/originals, image boxes, cat, and sleep marks are preserved.

The 63px plant and 70px flag desktop sizes target the 310px cat’s nominal 2.225px spacing. Occupied-cell centroids make actual neighbor distances vary; the goal is comparable visual sampling, not identical point counts per rectangular area. Counts are 582, 504, 305, 540, 453 for plants and 363 for the flag. Update `targetPitch` and reference display sizes in the sampler if the desired density or CSS sizing changes. Regenerate SVG and JSON together, then rebuild.

Run `python3 scripts/analyze-point-density.py` for current values, or add `--viewport 390` for mobile sizing. See [the density report](../docs/point-density.md) for before/after figures and limitations.
