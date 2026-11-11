use serde::Deserialize;
include!(concat!(env!("OUT_DIR"), "/posts.rs"));
#[derive(Deserialize, Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub assets: Vec<String>,
    pub html: String,
    pub headings: Vec<(String, String)>,
}
pub fn posts() -> Vec<Post> {
    serde_json::from_str(POSTS_JSON).expect("validated posts")
}
#[derive(Deserialize)]
pub struct Photo {
    pub file: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub caption: String,
}
pub fn photos() -> Vec<Photo> {
    serde_json::from_str(include_str!("../../content/gallery.json")).expect("gallery metadata")
}
#[derive(Deserialize)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub organization: String,
    pub dates: String,
    pub summary: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub url: String,
}
#[derive(Deserialize)]
pub struct Portfolio {
    pub experience: Vec<Entry>,
    pub projects: Vec<Entry>,
    pub research: Vec<Entry>,
    pub achievements: Vec<Entry>,
    pub volunteering: Vec<Entry>,
    pub hobbies: Vec<Entry>,
}
pub fn portfolio() -> Portfolio {
    serde_json::from_str(include_str!("../../content/portfolio.json")).expect("curated content")
}
