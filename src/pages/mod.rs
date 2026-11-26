use crate::{
    components::{arrow, entries, social_links, ArticleNavigation, PostCover},
    content,
    effects::Cat,
    routes::{href, Route},
};
use yew::prelude::*;
pub fn title(route: &Route) -> String {
    match route {
        Route::Home => "Home".into(),
        Route::About => "About".into(),
        Route::Experience => "Experience".into(),
        Route::Work => "Work".into(),
        Route::Hobbies => "Hobbies".into(),
        Route::Gallery => "Gallery".into(),
        Route::Blog => "Blog".into(),
        Route::Article { slug } => content::posts()
            .into_iter()
            .find(|p| p.slug == *slug)
            .map(|p| p.title)
            .unwrap_or("Article not found".into()),
        Route::NotFound => "Page not found".into(),
    }
}
pub fn description(route: &Route) -> String {
    match route {
        Route::Article { slug } => content::posts()
            .into_iter()
            .find(|p| p.slug == *slug)
            .map(|p| p.summary)
            .unwrap_or_default(),
        _ => format!(
            "{} — Sebastian Lopez Silva. Engineering, applied AI, software, and research.",
            title(route)
        ),
    }
}
pub fn sections(route: &Route) -> Vec<(String, String)> {
    match route {
        Route::About => [
            ("background", "Background"),
            ("education", "Education"),
            ("extra-courses", "Extra courses"),
            ("achievements", "Achievements"),
            ("skills", "Selected skills"),
            ("contact", "Contact"),
        ]
        .into_iter()
        .map(|(a, b)| (a.into(), b.into()))
        .collect(),
        Route::Hobbies => vec![
            (
                "competitive-programming".into(),
                "Competitive programming".into(),
            ),
            ("volunteering".into(), "Volunteering".into()),
            ("interests".into(), "Interests".into()),
        ],
        Route::Experience => content::portfolio()
            .experience
            .into_iter()
            .map(|e| (e.id, e.title))
            .collect(),
        Route::Work => vec![
            ("research".into(), "Research".into()),
            ("projects".into(), "Projects".into()),
        ],
        Route::Article { slug } => content::posts()
            .into_iter()
            .find(|p| p.slug == *slug)
            .map(|p| p.headings)
            .filter(|h| h.len() > 1)
            .unwrap_or_default(),
        _ => vec![],
    }
}
fn heading(number: &str, title: &str, intro: &str) -> Html {
    html! {<div class="page-heading"><p class="eyebrow">{number}</p><h1>{title}</h1><p class="lead">{intro}</p></div>}
}
fn post_date(date: &str) -> Html {
    if date.len() == 4 {
        html! {<span class="post-date">{date}</span>}
    } else {
        html! {<time class="post-date" datetime={date.to_owned()}>{date}</time>}
    }
}
pub fn render(route: &Route, base: &str) -> Html {
    match route {
        Route::Home => html! {<>
          <section class="home-intro"><p class="eyebrow"><span class="accent">{"Hello, I'm Sebastian"}</span>{" / SEBLS"}</p>
          <Cat base={base.to_owned()}/>
          <h1>{"Curious by nature."}<br/><span>{"Engineer in the making."}</span></h1>
          <p class="lead">{"I'm an engineering student working across software, applied AI, and research. I enjoy building useful systems, exploring how they work, and connecting practical engineering with new ideas."}</p>
          <div class="home-links"><a class="primary-link" href={href(base,&Route::Work)}>{"Explore my work"}{arrow("up-right")}</a><a href={href(base,&Route::About)}>{"A little about me"}{arrow("right")}</a></div></section>
        </>},
        Route::About => {
            html! {<>{heading("02 / A little context","About me","An interest in useful applications — and the systems underneath them.")}
              <section id="background" tabindex="-1"><h2>{"Background"}</h2><p>{"I'm pursuing a double degree at Mines Paris – PSL and Universidad Nacional de Colombia. My work spans software engineering, applied AI, and research, with experience in backend systems, data processing, and machine learning."}</p><p>{"I'm especially interested in AI agents, retrieval systems, and the engineering needed to make them reliable. Alongside application development, I enjoy learning about computer architecture, compilers, and generative models."}</p><p>{"I speak Spanish, English, French, some Portuguese and i'm trying to learn Chinese (It's so hard)."}</p></section>
              <section id="education" tabindex="-1"><h2>{"Education"}</h2><div class="education"><p class="eyebrow">{"2025 – Expected 2027"}</p><h3>{"Mines Paris – PSL"}</h3><p class="location">{"Paris, France"}</p><p>{"Engineering degree (M.Sc.), in progress"}</p><p>{"Machine Learning · Deep Learning & Computer Vision · Generative Models · Natural Language Processing · Compilers · Databases · Functional Programming · Computer Architecture"}</p></div><div class="education"><p class="eyebrow">{"2021 – Expected 2027"}</p><h3>{"Universidad Nacional de Colombia"}</h3><p class="location">{"Bogotá, Colombia"}</p><p>{"Computer Engineering (B.Sc.), in progress"}</p><p>{"Algorithms & Data Structures · Operating Systems · Computer Architecture · Networks & Databases · Software Engineering · Compilers · Parallel & Distributed Computing · Theory of Computation · Information Theory · Numerical & Convex Optimization · Machine Learning & Computer Vision · Probability & Statistics · Linear Algebra & Discrete Mathematics · Numerical Methods & Simulation"}</p></div></section>
              <section id="extra-courses" tabindex="-1"><h2>{"Extra courses"}</h2><dl class="achievement-list"><div><dt>{"November 2026"}</dt><dd><p class="achievement-title">{"Télécom Paris, Polytechnic Institute of Paris"}<span>{", Palaiseau, France"}</span></p><p>{"Formal Methods: Formal Specification and Verification of Systems (35 hours)"}</p></dd></div><div><dt>{"November 2026"}</dt><dd><p class="achievement-title">{"École normale supérieure – PSL"}<span>{", Paris, France"}</span></p><p>{"Cryptography and Computer Security (35 hours)"}</p></dd></div><div><dt>{"November 2025"}</dt><dd><p class="achievement-title">{"Czech Technical University in Prague"}<span>{", Prague, Czech Republic"}</span></p><p>{"Probabilistic Methods in Combinatorics (35 hours)"}</p></dd></div></dl></section>
              <section id="skills" tabindex="-1"><h2>{"Selected skills"}</h2><dl class="skills"><dt>{"Build"}</dt><dd>{"Python, C/C++, TypeScript, Java, FastAPI, SQL"}</dd><dt>{"Learn & evaluate"}</dt><dd>{"PyTorch, TensorFlow, scikit-learn, experimental design"}</dd><dt>{"Connect"}</dt><dd>{"AI agents, retrieval systems, knowledge graphs, multimodal pipelines"}</dd><dt>{"Deliver"}</dt><dd>{"Docker, Git, GitHub Actions, cloud infrastructure"}</dd></dl></section>
              <section id="achievements" tabindex="-1"><h2>{"Achievements"}</h2><dl class="achievement-list">{for content::portfolio().achievements.into_iter().map(|e| html!{<div id={e.id}><dt>{e.dates}</dt><dd><p class="achievement-title">if e.url.is_empty(){{e.title}}else{<a href={e.url}>{e.title}</a>}<span>{format!(" · {}", e.organization)}</span></p><p>{e.summary}</p></dd></div>})}</dl></section>
              <section id="contact" tabindex="-1"><h2>{"Let's connect"}</h2><p>{"For professional conversations and more of my code:"}</p><div class="text-links">{social_links()}</div></section>
            </>}
        }
        Route::Experience => {
            html! {<>{heading("03 / Putting ideas to work","Experience","Selected roles in applied AI, software engineering, and academic development.")}{entries(content::portfolio().experience)}</>}
        }
        Route::Work => {
            html! {<>{heading("04 / Questions into practice","Selected work","Research and projects exploring learning systems, optimization, and the tools we use to understand them.")}<section id="research" tabindex="-1"><h2>{"Research"}</h2>{entries(content::portfolio().research)}</section><section id="projects" tabindex="-1"><h2>{"Projects"}</h2>{entries(content::portfolio().projects)}</section></>}
        }
        Route::Hobbies => {
            html! {<>{heading("05 / Beyond the usual","Hobbies","Interests, challenges, and ways to give back.")}
              <section id="competitive-programming" tabindex="-1"><h2>{"Competitive programming"}</h2>{entries(content::portfolio().hobbies)}</section>
              <section id="volunteering" tabindex="-1"><h2>{"Volunteering"}</h2>{entries(content::portfolio().volunteering)}</section>
              <section id="interests" tabindex="-1"><h2>{"Interests"}</h2>{entries(content::portfolio().interests)}</section>
            </>}
        }
        Route::Gallery => {
            let photos = content::photos();
            html! {<>{heading("06 / Through the lens","Gallery","A space for photographs and the details that catch my eye.")}
            if photos.is_empty(){<div class="empty-state"><span class="empty-mark" aria-hidden="true">{"[  +  ]"}</span><h2>{"A collection, in time."}</h2><p>{"There are no photographs here yet. I'll share a selection of my own images in this space."}</p></div>}
            else {<div class="photo-grid">{for photos.into_iter().map(|p|html!{<figure><img src={format!("{}{}",base,p.file)} alt={p.alt} width={p.width.to_string()} height={p.height.to_string()} loading="lazy" decoding="async"/>if !p.caption.is_empty(){<figcaption>{p.caption}</figcaption>}</figure>})}</div>}
            </>}
        }
        Route::Blog => {
            html! {<>
              <header class="blog-heading"><p class="eyebrow">{"Notes & observations"}</p><h1>{"Writing"}</h1><p class="lead">{"A place to think out loud about engineering, learning, and the things in between."}</p></header>
              if content::posts().is_empty(){<div class="empty-state"><span class="empty-mark" aria-hidden="true">{"{ … }"}</span><h2>{"Room for a first thought."}</h2><p>{"No articles published yet. Future notes and longer reads will appear here."}</p></div>}
              else{<div class="post-list">{for content::posts().into_iter().map(|p|html!{
                <article class={classes!(p.cover_image.is_some().then_some("has-cover"))}>
                  if let Some(image) = &p.cover_image {
                    <a class="post-thumbnail" href={href(base,&Route::Article{slug:p.slug.clone()})} tabindex="-1" aria-hidden="true"><img src={format!("{}{}",base,image.file)} alt="" width={image.width.to_string()} height={image.height.to_string()} loading="lazy" decoding="async"/></a>
                  }
                  <div class="post-preview">{post_date(&p.date)}<h2><a href={href(base,&Route::Article{slug:p.slug})}>{p.title}</a></h2>
                    if !p.summary.is_empty(){<p>{p.summary}</p>}
                    if !p.tags.is_empty(){<p class="eyebrow">{p.tags.join(" / ")}</p>}
                  </div>
                </article>
              })}</div>}
            </>}
        }
        Route::Article { slug } => match content::posts().into_iter().find(|p| p.slug == *slug) {
            Some(p) => {
                let sections = sections(route);
                html! {<article class="reading">
                  <header class="blog-heading article-heading"><h1>{p.title}</h1>
                    if !p.summary.is_empty(){<p class="lead">{p.summary}</p>}
                  </header>
                  <div class="article-layout">
                    if !sections.is_empty(){<ArticleNavigation sections={sections} base={base.to_owned()}/>}
                    <dl class="article-metadata" aria-label="Article details">
                      <div><dt>{"Published"}</dt><dd>{post_date(&p.date)}</dd></div>
                      if !p.tags.is_empty(){<div><dt>{"Topics"}</dt><dd><ul>{for p.tags.iter().map(|tag| html!{<li>{tag}</li>})}</ul></dd></div>}
                    </dl>
                    <div class="article-content">
                      <PostCover image={p.cover_image} base={base.to_owned()}/>
                      <div class="article-body"><div class="prose">{Html::from_html_unchecked(AttrValue::from(p.html))}</div>
                        <a class="all-writing" href={href(base,&Route::Blog)}>{"All writing"}</a>
                      </div>
                    </div>
                  </div>
                </article>}
            }
            None => not_found(base),
        },
        Route::NotFound => not_found(base),
    }
}
fn not_found(base: &str) -> Html {
    html! {<>{heading("404 / A little off track","Page not found","That page isn't here. It may have moved, or the address may be incomplete.")}<a class="primary-link" href={href(base,&Route::Home)}>{"Back to Home"}{arrow("right")}</a></>}
}
