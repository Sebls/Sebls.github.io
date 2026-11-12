#[path = "build_support/markdown.rs"]
mod markdown;
use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=build_support");
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
        let date = p["date"]
            .as_str()
            .expect("article date YYYY, YYYY-MM, or YYYY-MM-DD");
        assert!(
            (date.len() == 4 && date.bytes().all(|c| c.is_ascii_digit()) && date != "0000")
                || (date.len() == 7
                    && date.is_ascii()
                    && date.as_bytes()[4] == b'-'
                    && date[..4].bytes().all(|c| c.is_ascii_digit())
                    && &date[..4] != "0000"
                    && matches!(date[5..].parse::<u8>(), Ok(1..=12)))
                || (date.len() == 10 && date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-'),
            "date must be YYYY, YYYY-MM, or YYYY-MM-DD"
        );
        let (html, headings) =
            markdown::render(&body).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        p["html"] = html.into();
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
