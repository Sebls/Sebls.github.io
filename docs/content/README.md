# Content and editorial guide

## Source of truth

[source-cv.md](source-cv.md) is the original extended CV, preserved verbatim. It establishes personal facts, not the page layout or final wording. New photographs, captions, and articles supplied by the owner will become authoritative for their respective content.

Do not edit the source CV merely to shorten website copy. Maintain curated website content separately when implementation begins. Curated public content lives in `content/portfolio.json`; the source CV is private local material excluded from Git until repository visibility is decided. A fresh clone can build using the curated content without the source CV.

## Old portfolio references

The `old-portfolio/` reference archive is local authoring material excluded from Git. It will not be present in a fresh clone; the curated public article, selected figures, and portfolio records are tracked separately and are sufficient to build the site.

The [old portfolio archive](old-portfolio/README.md) organizes content recovered from the supplied local `Sebls.github.io` checkout on 11 November 2026: Home/profile copy, education, experience, projects, skills, achievements, volunteering, and English/French/Spanish variants. It also preserves the complete autoencoder article and inventories its 13 figures, the portrait, and the old CV PDFs.

These are historical reference materials, with selected items now imported at the owner's request. The archive records conflicts with the current CV and details that need review. All 20 local images (13 figures, one portrait, and six CV icons) are copied into `old-portfolio/assets/`, and image links use those archived copies. PDF links still point to the old checkout. Keep this archive out of the public build along with the source CV and reference JSON; only curated copies belong in public content.

The [additional-facts comparison](old-portfolio/additional-facts.md) extracts old claims missing from the new website, identifies which are also absent from the extended source CV, and flags conflicting or historical details before reuse.

On 11 November 2026, the owner selected the three additional projects (PhraseCrack, smart irrigation, and Gaussian Splatting), achievements, volunteering, and the autoencoder article for the current site. Projects, achievements, hobbies, and volunteering live in `content/portfolio.json`. About renders compact, neutral achievement rows; Hobbies renders ICPC participation under Competitive programming and both volunteer roles under Volunteering, with local index links. Future hobby entries require owner-supplied facts. Award copy preserves the archived Top 1% scope and Mines exemption, identifies ICPC as participation, and uses the current CV's explicit UNAL waiver periods rather than expanding them to all semesters. The irrigation project uses the dated English/Spanish account. The complete article is in `content/posts/autoencoders-image-similarity.md`; its 13 selected figures are copied to `assets/images/autoencoders/` and declared in the post's `assets` list. Its source supplies only 2025, so the site displays a year without inventing a day. Gallery remains empty; portrait, icons, and PDFs are not imported into the public site.

## Manuscript and experience update

On 11 November 2026, the owner supplied the title and author order for the STIM manuscript: “Lipschitz morphological networks and their training as feature extractors,” by Mihaela Dimitrova, Mikhail Kataevskii, Sebastian Lopez Silva, Samy Blusseau, and Santiago Velasco-Forero. The submission-system acknowledgement identifies DGMM2027 submission 44 and receipt of updated files. Public copy says “Submitted to DGMM2027; decision pending”; this is not evidence of acceptance or publication. No paper URL, DOI, acceptance date, or publication date was supplied. The existing February–May 2026 dates describe the research work. LISI research-group membership is presented under Experience rather than as a paper in Work.

The owner subsequently supplied a separate STIM Research Intern role for March–May 2026, on-site in Fontainebleau. The owner then placed it immediately after STMicroelectronics and shortened its summary and detail, using the existing entry format. These supplied internship dates supersede the older CV dates for the role; the Work manuscript entry retains its broader February–May research period. LinkedIn edit-form links and the remote logo were not imported into the site.

The latest owner edits also expand the About coursework, add a November 2025 Probabilistic Methods in Combinatorics entry at Czech Technical University in Prague, place Selected skills before Achievements, and record full Mines tuition exemptions for both 2025–2026 and 2026–2027. These newer owner-supplied facts update the curated copy without rewriting the preserved source CV.

## TransMilenio project

The owner supplied a Universidad Nacional de Colombia Neural Networks course project in Bogotá, November 2024–February 2025: a TransMilenio station-congestion prediction system using temporal graph neural networks. Work presents it as “Predicting congestion at TransMilenio stations,” in the existing project-entry format. No implementation library, dataset, repository link, evaluation metric, or deployment outcome was supplied; the copy does not infer those details.

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

- Edit `content/portfolio.json` for curated experience, projects, research, achievements, hobbies, and volunteering. Entries can include an optional `url` for a linked title. Introductory copy is in `src/pages/mod.rs`.
- Add photographs under `assets/images/`, then records to `content/gallery.json`: `file` (for example `assets/images/photo.webp`), `alt`, `width`, `height`, and optional `caption`. Only declared gallery files are copied. Supply authorized images and remove private EXIF/location data before adding them.
- Add `content/posts/<slug>.md` and one record in `content/posts.json`: `slug`, `title`, `date` (`YYYY-MM-DD`, or `YYYY` when the source supplies only a year), `summary`, optional `tags` and `assets` arrays, and explicit `draft: false` to include it in the site. Slugs use lowercase ASCII letters, digits, and hyphens. Drafts (including missing flags) never enter generated HTML or WASM. Published records without bodies fail the build. Dates sort newest first; a year-only entry follows fully dated entries from that year. Future dates are not scheduled publishing, so keep future posts as drafts.
- Declare each selected article asset with its repository path under `assets/images/` in the post's `assets` array. Only assets belonging to published posts are copied by the native renderer. Use `../../assets/images/<folder>/<file>` in Markdown for images at `/blog/<slug>/`; this also works under repository-prefix hosting. Do not reference `docs/` or the old checkout from public article bodies.
- Article headings should start at `##` below the page title. Markdown is rendered and sanitized at build time. Use local section links and relative article links; for assets use relative paths appropriate to `/blog/<slug>/`. Article headings receive stable numbered IDs for the page index.
- Rebuild after local edits. The autoencoder article is the first real post; no example articles or photographs are published.
