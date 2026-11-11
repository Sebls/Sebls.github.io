use crate::{
    components::{arrow, entries, social_links},
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
            ("skills", "Selected skills"),
            ("contact", "Contact"),
        ]
        .into_iter()
        .map(|(a, b)| (a.into(), b.into()))
        .collect(),
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
              <section id="education" tabindex="-1"><h2>{"Education"}</h2><div class="education"><p class="eyebrow">{"2025 – Expected 2027"}</p><h3>{"Mines Paris – PSL"}</h3><p>{"Engineering degree (M.Sc.), in progress"}</p><p>{"Machine learning, generative models, natural language processing, and engineering systems."}</p></div><div class="education"><p class="eyebrow">{"2021 – Expected 2027"}</p><h3>{"Universidad Nacional de Colombia"}</h3><p>{"Computer Engineering (B.Sc.), in progress"}</p><p>{"Algorithms, computer systems, optimization, and machine learning."}</p></div></section>
              <section id="skills" tabindex="-1"><h2>{"Selected skills"}</h2><dl class="skills"><dt>{"Build"}</dt><dd>{"Python, C/C++, TypeScript, Java, FastAPI, SQL"}</dd><dt>{"Learn & evaluate"}</dt><dd>{"PyTorch, TensorFlow, scikit-learn, experimental design"}</dd><dt>{"Connect"}</dt><dd>{"AI agents, retrieval systems, knowledge graphs, multimodal pipelines"}</dd><dt>{"Deliver"}</dt><dd>{"Docker, Git, GitHub Actions, cloud infrastructure"}</dd></dl></section>
              <section id="contact" tabindex="-1"><h2>{"Let's connect"}</h2><p>{"For professional conversations and more of my code:"}</p><div class="text-links">{social_links()}</div></section>
            </>}
        }
        Route::Experience => {
            html! {<>{heading("03 / Putting ideas to work","Experience","Selected roles in applied AI, software engineering, and academic development.")}{entries(content::portfolio().experience)}</>}
        }
        Route::Work => {
            html! {<>{heading("04 / Questions into practice","Selected work","Research and projects exploring learning systems, optimization, and the tools we use to understand them.")}<section id="research" tabindex="-1"><h2>{"Research"}</h2>{entries(content::portfolio().research)}</section><section id="projects" tabindex="-1"><h2>{"Projects"}</h2>{entries(content::portfolio().projects)}</section></>}
        }
        Route::Gallery => {
            let photos = content::photos();
            html! {<>{heading("05 / Through the lens","Gallery","A space for photographs and the details that catch my eye.")}
            if photos.is_empty(){<div class="empty-state"><span class="empty-mark" aria-hidden="true">{"[  +  ]"}</span><h2>{"A collection, in time."}</h2><p>{"There are no photographs here yet. I'll share a selection of my own images in this space."}</p></div>}
            else {<div class="photo-grid">{for photos.into_iter().map(|p|html!{<figure><img src={format!("{}{}",base,p.file)} alt={p.alt} width={p.width.to_string()} height={p.height.to_string()} loading="lazy" decoding="async"/>if !p.caption.is_empty(){<figcaption>{p.caption}</figcaption>}</figure>})}</div>}
            </>}
        }
        Route::Blog => {
            html! {<>{heading("06 / Notes & observations","Writing","A place to think out loud about engineering, learning, and the things in between.")}
              if content::posts().is_empty(){<div class="empty-state"><span class="empty-mark" aria-hidden="true">{"{ … }"}</span><h2>{"Room for a first thought."}</h2><p>{"No articles published yet. Future notes and longer reads will appear here."}</p></div>}
              else{<div class="post-list">{for content::posts().into_iter().map(|p|html!{<article><time datetime={p.date.clone()}>{p.date}</time><h2><a href={href(base,&Route::Article{slug:p.slug})}>{p.title}{arrow("up-right")}</a></h2><p>{p.summary}</p><p class="eyebrow">{p.tags.join(" / ")}</p></article>})}</div>}
            </>}
        }
        Route::Article { slug } => match content::posts().into_iter().find(|p| p.slug == *slug) {
            Some(p) => {
                html! {<article class="reading"><a class="back-link" href={href(base,&Route::Blog)}>{arrow("left")}{"All writing"}</a><header class="page-heading"><p class="eyebrow"><time datetime={p.date.clone()}>{p.date}</time></p><h1>{p.title}</h1><p class="lead">{p.summary}</p></header><div class="prose">{Html::from_html_unchecked(AttrValue::from(p.html))}</div></article>}
            }
            None => not_found(base),
        },
        Route::NotFound => not_found(base),
    }
}
fn not_found(base: &str) -> Html {
    html! {<>{heading("404 / A little off track","Page not found","That page isn't here. It may have moved, or the address may be incomplete.")}<a class="primary-link" href={href(base,&Route::Home)}>{"Back to Home"}{arrow("right")}</a></>}
}
