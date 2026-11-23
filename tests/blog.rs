#![cfg(not(target_arch = "wasm32"))]

use portfolio::{
    components::{PostCover, PostCoverProps},
    content::PostImage,
};

#[tokio::test]
async fn optional_cover_reserves_dimensions_and_escapes_supplied_text() {
    let props = PostCoverProps {
        image: Some(PostImage {
            file: "assets/images/cover.webp".into(),
            alt: "A <supplied> image".into(),
            width: 1600,
            height: 900,
            caption: "Owner's caption".into(),
        }),
        base: "/portfolio/".into(),
    };
    let html = yew::ServerRenderer::<PostCover>::with_props(move || props)
        .render()
        .await;
    assert!(html.contains("src=\"/portfolio/assets/images/cover.webp\""));
    assert!(html.contains("width=\"1600\""));
    assert!(html.contains("height=\"900\""));
    assert!(html.contains("alt=\"A &lt;supplied&gt; image\""));
    assert!(html.contains("<figcaption>"));
    let props = PostCoverProps {
        image: None,
        base: "/".into(),
    };
    let html = yew::ServerRenderer::<PostCover>::with_props(move || props)
        .render()
        .await;
    assert!(!html.contains("<figure"));
    assert!(!html.contains("<img"));
}
