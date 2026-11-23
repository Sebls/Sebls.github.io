#![cfg(not(target_arch = "wasm32"))]

use portfolio::{
    app::{App, AppProps},
    components::{PostCover, PostCoverProps},
    content::PostImage,
};

#[tokio::test]
async fn blog_shell_is_scoped_and_keeps_native_links() {
    for path in ["/blog/", "/blog/gcd-lcm-prime-exponents/"] {
        let props = AppProps {
            path: path.into(),
            base: "/portfolio/".into(),
        };
        let html = yew::ServerRenderer::<App>::with_props(move || props)
            .render()
            .await;
        assert!(html.contains("class=\"blog-site\""));
        assert!(html.contains("href=\"/portfolio/\">Portfolio</a>"));
        assert!(!html.contains("class=\"rail\""));
        assert!(!html.contains("menu-toggle"));
        assert!(!html.contains("post-cover"));
        if path != "/blog/" {
            assert!(html.contains("href=\"/portfolio/blog/\""));
            assert!(html.contains(">All writing</a>"));
            assert!(html.contains("aria-label=\"On this page\""));
        }
    }
    let props = AppProps {
        path: "/about/".into(),
        base: "/".into(),
    };
    let html = yew::ServerRenderer::<App>::with_props(move || props)
        .render()
        .await;
    assert!(html.contains("class=\"rail\""));
    assert!(!html.contains("blog-site"));
}

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
