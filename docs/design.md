# Design references and interaction guidance

## Assigned responsibilities

| Reference | Inspect first | Use for | Do not inherit |
| --- | --- | --- | --- |
| [portfolio-design-system.json](references/portfolio-design-system.json) | `metadata`, `global`, relevant `designTokens` and `components` | Color, typography, spacing, surfaces, visual hierarchy | Personal text, page structure, third-party imagery |
| [portfolio-layout-navigation.json](references/portfolio-layout-navigation.json) | `layout`, `navigation`, `index`, `architecture`, `responsive` | Left rail, content proportions, page index, navigation grouping | Marketing pages, product copy, its competing visual theme |
| [black-hole-inspection.json](references/black-hole-inspection.json) | `viewport`, `canvases`, `events`, selected `snapshots` | Visual clues for the local illustration interaction | Global pointer scope, reference assets, assumed shaders/libraries |

JSON is inspection evidence, not production configuration. Inspect keys and targeted excerpts rather than loading every event and computed style into the agent's context.

## Observed evidence

- Visual reference: `jayveepvalenzuela.github.io`, inspected at 2056 × 718, dark color scheme. Color samples include near-white `rgb(242, 242, 242)`, muted gray `rgb(146, 154, 162)`, dark `rgb(17, 17, 17)`, and coral `rgb(255, 100, 105)`. Recorded families include Satoshi Variable and Cabinet Grotesk Variable. These are candidates, not a complete approved token palette.
- Layout reference: `supermemory.ai`, inspected at 2056 × 718. The captured `aside.rail.rail-toc.is-visible` is **sticky**, 208 px wide, with `top: 0`; the main content column is 640 px wide. Separate site and local navigation groups appear in the rail. These dimensions describe that viewport, not universal fixed sizes.
- Interaction reference: `mannan.io`. The export contains canvas observations, pointer events, and style snapshots, but no detected technologies or animation source code. Repeated canvas records are observations, not necessarily distinct live canvases. Do not claim the export proves a particular renderer or distortion formula.

## Synthesis

Use the visual reference's dark editorial character with the layout reference's quiet rail and readable content column. Do not import the layout reference's light theme as a second competing design system. Use a small shared set of CSS variables for color roles, font families, type sizes, spacing, rail width, and content width.

Reference dimensions, frequencies, selectors, and negative spacing values are not automatically good design tokens. Adapt the page at smaller widths and use an appropriate grid for photography. Favor readable typography and alignment over unnecessary cards or ornament.

Separate evidence from decisions: record the final palette, font choices, breakpoints, and renderer briefly here when selected during implementation. Responsive choices not demonstrated in the exports must be labeled as adaptations. Only one layout snapshot is present; do not claim exhaustive mobile or scroll-state coverage.

## Interaction constraints

[The brief](brief.md) defines the hard boundary: the black-hole effect exists only in Home's cat container. Start with the simplest rendering method that achieves the needed impression; do not default to a large animation or 3D dependency. Exact shader reconstruction is not required from incomplete evidence.

Use ordinary hover/focus feedback elsewhere, without pointer-following distortion. Native scrolling stays intact. A local page index can use visibility observation on long pages, but must not rewrite history on every scroll movement. With hash routing, implement section selection through a route-compatible mechanism rather than overwriting the route hash.

Use local original/authorized artwork and fonts. Asset URLs and font names in the export do not grant redistribution permission. Check availability and licensing before adopting fonts; use a suitable fallback when necessary. Do not hotlink reference-site assets or depend on those sites at runtime.

## Missing inputs

Original gallery images, captions, and blog articles have not been supplied. Preserve intentional gallery/blog empty states until they arrive. Home uses a point SVG derived from the supplied cat PNG; individual particle repulsion is now implemented.

## Implemented choices

The palette uses #111111 background, #f2f2f2 text, #a0a5ab secondary text, #2a2a2a rules, and #ff6469 coral accents. System sans-serif and monospace fonts avoid unlicensed reference font redistribution and external requests. The desktop rail is 208px, with a flexible editorial column. Adaptations use 1000px and 720px breakpoints; the latter switches to an expandable header menu. No reference establishes these mobile layouts.

The supplied local `sleeping-cat.png` is preserved as source artwork. Home now uses `sleeping-cat-points.svg`: 6,305 colored circles sampled on a 9px grid, with 6.4px radii in the original 1254px coordinate space. Circles overlap to keep the panel background from showing through the interior while preserving the point count and positions. Pixels below 90% opacity are excluded to remove faint cutout fringes. Matching JSON records positions and RGB colors, compiled into the interactive WASM renderer rather than copied as a public file. The SVG remains an external accessible fallback. Each illustration has a local Canvas layer that renders individual points at their original colors and positions. Within a 40 CSS-pixel radius, a smooth radial falloff pushes them outward around a 14px core matching the #FFAC00 panel. Velocity-based damped springs restore exact resting artwork after exit. Pointer listeners belong to the cat surface; resize, window blur, document visibility, and media preference changes reset the effect. Animation frames stop when settled or offscreen; unmount cancels the pending frame and releases listeners/closures. Reduced-motion, coarse-pointer, and no-hover media keep the SVG static. No animation or rendering dependency was added. Browser Canvas and animation frame APIs were checked against official web-sys documentation.

Home’s yellow (#FFAC00) cat panel sits before the introduction. The cat is anchored at the bottom right using the existing responsive offsets; its lower silhouette slightly overhangs the panel. Sleep marks sit above it in an 80 × 16px box rotated −15°. Reduced motion preserves these positions.

The cat caption sits 16px below the banner, giving the text breathing room at all screen widths.

Link arrows use inline SVGs with a 1.4-unit stroke and rounded ends, shared across navigation, calls to action, and article links. GitHub and LinkedIn links pair small monochrome brand icons with visible text labels in the footer and About contact section. Decorative SVGs are hidden from assistive technology. Home no longer includes the “At the intersection” row.

The rail wordmark reads “Sebls” and Explore links omit numbering. A small coral outline location pin accompanies the Paris location. Work presents Research before Projects, with the same order in its local section index.

All pages start directly with their main content; the shared name/breadcrumb metadata header has been removed, including its divider and spacing.

Five supplied plant illustrations now form a horizontal row at the bottom left of Home’s cat surface, ordered by filename from plant-1 through plant-5. A single `--plant-size:63px` setting in `.plant-cluster` controls all five bounding boxes; equal grid columns shrink together to fit at most 48% of the panel width. Plants retain their proportions and share a bottom baseline. Transparent cutouts are sampled into separate colored-circle SVGs using the cat’s 9-unit grid, 6.4-unit radius, and 90% alpha cutoff, with each trimmed illustration normalized to a 1,254-unit longest edge. Matching point JSON supplies the shared particle renderer. The cluster scales with the panel width and ignores pointer events; the plants share the cat interaction. Only the SVGs are copied into public output; original PNGs, cutouts, and point JSON remain authoring inputs.

At widths up to 720px, only the cat is rendered visibly; the flag, plants, and sleep marks are hidden. At widths up to 480px, the cat scales to 140–180px. Desktop cat sizing is unchanged. Disconnected cutout specks are excluded from the sampled point set.

The supplied transparent Colombian flag appears above the left end of the plant row, at the panel’s left edge and 110px above its bottom. Its width is capped at 70px and scales with the panel width. The SVG uses the same 9-unit grid, 6.4-unit circles, and 90% alpha cutoff as the cat. Matching flag point JSON supplies the shared renderer; only the SVG is copied as a public artwork file. The flag is hidden on mobile.

The sleep marks now use `sleep-marks-points.svg`, a simple 39-circle drawing of three small z glyphs, rather than CSS-rendered text. The external image keeps the same right/bottom anchoring and −15° rotation in an 80 × 16px box. The dots use #f2f2f2 with matching JSON coordinates/colors used by the shared renderer. The decorative SVG has empty alt text; its animation listens only on the shared illustration surface.

The dotted sleep marks follow a gentle upward arc: the middle z sits 3.6 CSS pixels above the two outer glyphs. The same 80 × 16px image box, CSS anchors, rotation, dot count, and colors are retained. SVG and JSON positions match.

All eight point illustrations now use the same `PointArtwork` component, each with its own SVG fallback, Canvas, and state. Canvas dimensions and particle radius come from the supplied point JSON rather than cat-specific constants. Pointer coordinates account for contain letterboxing, bottom alignment, and centered 2D CSS transforms (including the sleep marks’ rotation). Listeners stay on the Home illustration surface; only nearby artworks animate. Each layer clips its particles to its image bounds, and failure of one layer preserves its own SVG fallback. New illustrations need matching SVG/JSON assets, a component entry, CSS placement, and a Trunk SVG copy entry; no new animation implementation is required. This extends the existing Home illustration boundary to its decorative objects, without affecting other page content. The 720px mobile cutoff is a layout adaptation requested by the owner.

The pointer mapping uses the browser’s computed style and element dimensions, checked against the official [web-sys Window API](https://docs.rs/web-sys/latest/web_sys/struct.Window.html#method.get_computed_style) and [HtmlElement API](https://docs.rs/web-sys/latest/web_sys/struct.HtmlElement.html#method.client_width). Parent positioning wrappers remain untransformed; artwork transforms use the centered 2D CSS convention. Mobile CSS automatically hides all point artworks except `.cat-art`, including future additions.


The shared particle effect now retains a velocity and orbital phase for each point. Within the existing 40 CSS-pixel influence, the repelled target gently circulates around the pointer, with angular speed fading toward the influence edge. A damped spring follows that target, so holding the mouse still keeps nearby points moving and pointer changes preserve momentum. On exit, the orbital displacement fades and the spring coasts back to the exact source positions; both displacement and speed must settle before frames stop and the SVG returns. Integration uses seconds and substeps no longer than 1/120 second, with frame deltas capped at 50ms to bound recovery from stalls. Reset also clears velocities and phases. The 14px core, artwork clipping, local listeners, media fallbacks, and lifecycle boundaries remain. No dependency or new browser API was introduced; the existing [animation-frame API](https://docs.rs/web-sys/latest/web_sys/struct.Window.html#method.request_animation_frame) remains the scheduler.

## Plant and flag density reduction — 11 November 2026

This update supersedes the earlier 9-unit sampling and 6.4-unit radius for plants and flag only. Cat and sleep-mark assets, CSS sizing, placement, and shared interaction are unchanged. `scripts/generate-decoration-points.swift` targets the 310px desktop cat’s 2.225 CSS-pixel pitch at 63px plant boxes and a 70px-wide flag. Nominal source cell widths are about 44.286 units for plants and 40.684 for the flag; radius is cell width × 6.4/9, giving the cat’s 3.164px desktop dot diameter.

The sampler reads original transparent artwork, excludes alpha below 90%, and retains the largest 8-connected source component for plants before sampling. Each occupied coarse cell produces one dot positioned at the opaque-pixel centroid with averaged RGB; sparse cells preserve stems and leaf tips. Dot centers are clamped by radius inside unchanged viewBoxes. This slightly rounds fine edges while preserving silhouette and main color regions. Actual neighbor spacing varies with centroids; grid pitch is nominal. New counts: plants 582 / 504 / 305 / 540 / 453; flag 363. Combined reduction: 55,840 → 2,747 (95.1%). SVGs and animation JSON are regenerated together. The old plant and flag script entry points delegate to the shared sampler. No runtime code, dependencies, or new browser APIs were added.
