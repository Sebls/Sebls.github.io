# Future implementation prompt

Use this prompt only when ready to authorize development. It does not authorize implementation during the current documentation task.

---

Implement my personal portfolio using the documentation in this repository. Begin with `AGENTS.md`, then read the active brief, technical plan, content guide, source CV, and relevant design reference sections.

I want a simple, polished portfolio with separate Home, About, Experience, Work, Gallery, and Blog views, plus article reading views. Use the selected Rust/Yew/Trunk stack and prepare it for static GitHub Pages hosting. Follow the technical plan's proposed structure without adding unnecessary layers or dependencies.

Before scaffolding, resolve the documented choice between Yew hash-routed page views and separately generated HTML pages. Explain that tradeoff briefly and ask me which I mean if I have not already chosen it. Keep the selected framework unless I explicitly authorize a change. Make routine reversible implementation decisions yourself.

Use the design-system JSON for visual identity and the layout JSON for the left rail and page index. Integrate them into one original, responsive editorial design. Separate site-page navigation from current-page section links. Interpret reference evidence; do not copy its markup, text, or third-party assets.

Write concise portfolio copy from the extended CV. Select relevant details, shorten motivation-letter paragraphs, preserve factual qualifications and dates, and avoid fabricated achievements or links. Keep detailed source material out of the public build.

On Home, reserve a sleeping-cat vector illustration area. I will supply the artwork/reference later if it is still missing. The black-hole-inspired pointer animation must run exclusively inside that illustration's div. It must never distort text, navigation, neighboring content, or other pages. Provide a static reduced-motion/touch fallback and clean up rendering resources. Until artwork arrives, keep the area a restrained static placeholder; do not invent the final illustration.

Create a Gallery ready for my original photographs and a Blog ready for locally authored Markdown articles. If I have supplied none, show intentional empty states. Do not populate either with fake material. Exclude article drafts and make adding content straightforward without changing layout code.

After resolving the rendering choice, summarize the chosen routes, visual direction, and any material assumptions, then proceed with implementation without requesting routine approvals. Build the shared layout and static content first, then gallery/blog support and the contained effect where assets permit. Verify the release build, hosting base path, deep links, mobile layout, keyboard use, reduced motion, and effect isolation. Report actual checks and remaining owner inputs. Prepare deployment configuration but do not publish the site in this task.
