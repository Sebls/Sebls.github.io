# Portfolio brief

## Intent

Build a restrained, personal, editorial portfolio for Sebastian Lopez Silva. Visitors should quickly understand his engineering background, applied AI work, projects, and research, then explore photography and writing as personal interests.

The site should feel distinctive through typography, a clear left navigation rail, thoughtful spacing, and one playful Home illustration. The supplied design references should guide an original, coherent design.

## Requirements

- A website with separate main pages and shared navigation, hosted as static files on GitHub Pages without a backend.
- Rust/Yew is the selected stack. Separate page views are required; the exact URL/rendering architecture must respect the distinction in [the technical plan](technical.md).
- A sleeping-cat vector on Home, with a black-hole-inspired pointer effect confined to the illustration's own container.
- Short, selective writing derived from the extended CV; no motivation-letter transcription.
- A gallery for original photographs and a blog for future articles or other writing.
- Responsive layouts, semantic structure, visible keyboard focus, readable contrast, and reduced-motion support.

## Proposed page map

This grouping is a practical default; it avoids a separate page for every CV heading.

| Page | Purpose and initial content | Local index |
| --- | --- | --- |
| Home | Name, short professional introduction, sleeping cat, links to selected work and contact | Usually unnecessary |
| About | Short background, education, achievements, selected skills, professional profile links | Background, Education, Achievements, Skills, Contact |
| Experience | Curated professional roles; concise contribution summaries | Role headings |
| Work | Selected projects and research, clearly labeled; publications only with supported details | Projects, Research |
| Hobbies | Competitive programming, volunteering, and future owner-supplied interests | Competitive programming, Volunteering |
| Gallery | Original photographs with optional factual captions | Unnecessary initially |
| Blog | Published article list, followed by a separate reading view for each article | Article headings on long articles |

Contact can live on About and in a compact shared footer. Use only supplied contact/profile links. Add a CV download only when a public CV file is provided. Do not create empty Publications, Awards, or individual project-detail pages just to fill navigation.

## Navigation

The left rail has two clearly labeled groups when needed: site pages and the current page's section index. The active page and active section are different states. Section links never act as substitutes for page routes. Generate index entries from the sections actually rendered. Short pages do not need an index.

Use a sticky rail on desktop, with sufficient space for long menus. On small screens use a compact header and an accessible expandable menu. Preserve normal scrolling, browser navigation, and touch behavior. On route changes, give the new page an appropriate title and move focus to the main heading; local section navigation must also be keyboard usable.

## Home illustration contract

The supplied local PNG has been converted into a colored point SVG at the owner's request. This preserves the existing cat design and prepares positions and colors for later local gravitational interaction. A shared local Canvas renderer repels and swirls individual points in the cat, plants, flag, and sleep marks around the mouse, preserving momentum and restoring their positions on exit, with SVGs retained as static fallbacks. At widths up to 720px, show only the cat. If no local artwork is available, use an unobtrusive static placeholder. Do not invent a final cat design or reuse the reference website's imagery.

The local black-hole effect may visually warp the illustration or a decorative layer inside that area, depending on what the reference evidence supports. Exact appearance will be refined when artwork is supplied. It must satisfy all of these boundaries:

- Its rendering surface, coordinates, clipping, and pointer listeners belong to the illustration container.
- Entry activates the effect, movement updates it locally, and exit restores a resting state.
- No global cursor replacement, whole-page distortion, scroll hijacking, or effects on sibling content.
- No continuous offscreen work; release listeners and graphical resources when unmounted.
- Reduced motion and coarse/touch pointers receive a static illustration. No information or action depends on the effect.
- Text, links, the navigation rail, and other pages remain visually and functionally independent.

## Gallery and blog

The Gallery supports a simple responsive image grid, dimensions reserved to avoid layout shifts, useful alt text, and efficient image loading. A lightbox, filters, albums, and infinite scrolling are outside the initial scope. With no photographs, show a short intentional empty state and no stock-image substitutes.

The Blog supports titles, publication dates, optional summaries/tags, and a readable article view. Sort published posts newest first; exclude drafts and missing articles from the public list. No fabricated posts, comments system, CMS, search service, newsletter, or runtime publishing API. With no posts, show a short empty state. Articles are written locally and deployed with the site.

## Acceptance criteria

- Every main page is reachable through shared navigation; direct links and refreshes work on the selected GitHub Pages URL.
- Home has a short introduction and a reserved cat area; its effect is contained during pointer entry, exit, scrolling, resizing, and page navigation.
- No extended CV paragraphs are dumped into the UI; qualifications, dates, and selected outcomes remain accurate.
- Gallery and Blog work with zero items and with real supplied items. A published article has its own reading view; drafts are excluded.
- The rail and page index remain distinguishable and usable on desktop, mobile, keyboard, and touch.
- Reduced motion disables the decorative effect. Missing artwork does not break layout or block reading.
- A release build runs from static hosting at the chosen root or repository subpath, with no runtime backend or reference-site dependency.
- Document the actual SEO and no-JavaScript limitations of the selected rendering approach rather than claiming static pre-rendering.

## Non-goals

No authentication, database, admin dashboard, complex design-system package, generic plugin architecture, full CV transcription, mandatory animation library, or custom content platform. Add features only when the owner requests them.
