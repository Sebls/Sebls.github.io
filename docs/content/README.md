# Content and editorial guide

## Source of truth

[source-cv.md](source-cv.md) is the original extended CV, preserved verbatim. It establishes personal facts, not the page layout or final wording. New photographs, captions, and articles supplied by the owner will become authoritative for their respective content.

Do not edit the source CV merely to shorten website copy. Maintain curated website content separately when implementation begins. Curated public content lives in `content/portfolio.json`; the source CV is private local material excluded from Git until repository visibility is decided. A fresh clone can build using the curated content without the source CV.

## Voice and length

Use natural, direct English initially. Keep the first-person voice for introductions and personal reflections. Use concrete contribution language for role and project summaries. Avoid inflated claims, keyword walls, and repeated explanations across pages.

| Content | Target |
| --- | --- |
| Home introduction | 30–50 words; identity, focus, one personal note if supported |
| About introduction | 80–140 words in one or two short paragraphs |
| Experience entry | Role, organization, dates; one short summary and at most 2–3 meaningful bullets |
| Project/research card | 40–80 words; problem, own contribution, supported result; a few relevant tools |
| Skills | A few readable groups of selected tools; no exhaustive inventory or percentage meters |
| Photo caption | Optional, 1–2 short lines based on supplied facts |
| Article summary | Optional, 1–2 sentences; article body follows the author's writing |

These are editing targets, not reasons to omit necessary factual qualifications. Never infer seniority, proficiency, degrees awarded, authorship, or publication status from aspirations.

## Selection rules

- Home introduces the person; detailed history belongs on other pages.
- Condense the source Profile into current focus and background. Omit extended career aspirations, job-application language, and speculative future plans by default.
- Internship availability is time-sensitive. Include it only if the owner confirms it is still relevant when publishing.
- Preserve expected graduation wording. Do not turn an ongoing degree into an awarded degree or an old role into a current one.
- Keep chronology and overlapping roles accurate. Never merge distinct titles into an unsupported promotion or longer employment period.
- Select a small initial set of projects and research items, based on explanatory value and available evidence. Additional CV entries may be omitted.
- Retain useful metrics only with their original scope and qualifiers. Do not turn a selected experiment result into a general production guarantee.
- A co-authored manuscript is not automatically a published paper. Omit publication links and formal citations unless supplied.
- Do not infer a technology for a project merely because it appears elsewhere in the skills list. In particular, the site's Rust stack is not proof of prior professional Rust experience.
- Missing URLs or media should be omitted, not replaced with fake links, invented screenshots, or disabled buttons.

## Starting copy — draft for later review

Home:

> I'm Sebastian, an engineering student working across software and applied AI. I enjoy building useful systems, exploring how they work, and connecting practical engineering with research.

About:

> I'm pursuing a double degree at Mines Paris – PSL and Universidad Nacional de Colombia. My work spans software engineering, applied AI, and research, with experience in backend systems, data processing, and machine learning.
>
> I'm especially interested in AI agents, retrieval systems, and the engineering needed to make them reliable. Alongside application development, I enjoy learning about computer architecture, compilers, and operating systems. Teaching and sharing knowledge also matter to me, shaped by university tutoring and volunteer work.

These drafts summarize the source; they are not an instruction to publish unreviewed time-sensitive details.

## Minimal future content records

Use the simplest local formats that fit Yew; see [the technical plan](../technical.md). Do not build a schema framework.

- Profile: name, short introduction, about paragraphs, verified profile/contact links.
- Experience: stable ID, role, organization, dates, summary, selected bullets.
- Project/research: stable ID, title, type, summary, contribution, optional outcome, technologies and real links.
- Photo: stable ID, local file, alt text, optional caption/date/location. Remove embedded location data when it should not be public.
- Article: stable slug, title, publication date, optional summary/tags, draft flag, Markdown body. Use a tiny explicit local manifest for metadata if simplest; elaborate front-matter tooling is unnecessary.

Adding a photo or post should require local content/asset edits and a rebuild, not changes to the page layout. Never ship the source CV, archive, or raw reference JSON just to make content loading easier.

## Adding content

- Edit `content/portfolio.json` for curated experience, projects, and research. Introductory copy is in `src/pages/mod.rs`.
- Add photographs under `assets/images/`, then records to `content/gallery.json`: `file` (for example `assets/images/photo.webp`), `alt`, `width`, `height`, and optional `caption`. Only declared gallery files are copied. Supply authorized images and remove private EXIF/location data before adding them.
- Add `content/posts/<slug>.md` and one record in `content/posts.json`: `slug`, `title`, `date` (`YYYY-MM-DD`), `summary`, optional `tags` array, and explicit `draft: false` to publish. Slugs use lowercase ASCII letters, digits, and hyphens. Drafts (including missing flags) never enter generated HTML or WASM. Published records without bodies fail the build. Dates sort newest first; future dates are not scheduled publishing, so keep future posts as drafts.
- Article headings should start at `##` below the page title. Markdown is rendered and sanitized at build time. Use local section links and relative article links; for assets use relative paths appropriate to `/blog/<slug>/`. Article headings receive stable numbered IDs for the page index.
- Rebuild after local edits. No example photographs or articles are published by default.
