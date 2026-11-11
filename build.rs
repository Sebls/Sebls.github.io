use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=content");
    let mut posts: Vec<serde_json::Value> =
        serde_json::from_str(&fs::read_to_string("content/posts.json").unwrap()).unwrap();
    let mut slugs = std::collections::HashSet::new();
    posts.retain(|p| p["draft"].as_bool() == Some(false));
    for p in &mut posts {
        let slug = p["slug"].as_str().expect("post slug");
        assert!(
            !slug.is_empty()
                && slug
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-'),
            "unsafe article slug"
        );
        assert!(slugs.insert(slug.to_string()), "duplicate slug");
        let path = PathBuf::from("content/posts").join(format!("{slug}.md"));
        let body = fs::read_to_string(&path).expect("published article body missing");
        let date = p["date"].as_str().expect("article date YYYY or YYYY-MM-DD");
        assert!(
            (date.len() == 4 && date.bytes().all(|c| c.is_ascii_digit()) && date != "0000")
                || (date.len() == 10 && date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-'),
            "date must be YYYY or YYYY-MM-DD"
        );
        let mut headings: Vec<(String, String)> = Vec::new();
        let mut current: Option<(String, String)> = None;
        let events = pulldown_cmark::Parser::new_ext(&body, pulldown_cmark::Options::ENABLE_TABLES)
            .map(|event| {
                use pulldown_cmark::{Event, Tag, TagEnd};
                match event {
                    Event::Start(Tag::Heading {
                        level,
                        classes,
                        attrs,
                        ..
                    }) => {
                        let id = format!("article-section-{}", headings.len() + 1);
                        current = Some((id.clone(), String::new()));
                        Event::Start(Tag::Heading {
                            level,
                            id: Some(id.into()),
                            classes,
                            attrs,
                        })
                    }
                    Event::Text(ref text) | Event::Code(ref text) => {
                        if let Some((_, label)) = &mut current {
                            label.push_str(text);
                        }
                        event
                    }
                    Event::End(TagEnd::Heading(_)) => {
                        if let Some(h) = current.take() {
                            headings.push(h);
                        }
                        event
                    }
                    _ => event,
                }
            });
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, events);
        p["html"] = ammonia::Builder::default()
            .add_tag_attributes("h1", &["id"])
            .add_tag_attributes("h2", &["id"])
            .add_tag_attributes("h3", &["id"])
            .add_tag_attributes("h4", &["id"])
            .add_tag_attributes("h5", &["id"])
            .add_tag_attributes("h6", &["id"])
            .clean(&html)
            .to_string()
            .into();
        let rendered = p["html"].as_str().unwrap().to_owned();
        p["html"] = (1..=6)
            .fold(rendered, |html, level| {
                html.replace(
                    &format!("<h{level} id="),
                    &format!("<h{level} tabindex=\"-1\" id="),
                )
            })
            .into();
        p["headings"] = serde_json::to_value(headings).unwrap();
    }
    posts.sort_by(|a, b| b["date"].as_str().cmp(&a["date"].as_str()));
    let json = serde_json::to_string(&posts).unwrap();
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("posts.rs"),
        format!("pub const POSTS_JSON: &str = {json:?};"),
    )
    .unwrap();
}
