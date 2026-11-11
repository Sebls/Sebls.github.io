#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use portfolio::{
        app::{App, AppProps},
        content, pages,
        routes::{href, Route},
    };
    use std::{fs, path::PathBuf};
    use yew_router::Routable;
    let args: Vec<String> = std::env::args().collect();
    let out = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("dist"));
    let base = args.get(2).cloned().unwrap_or("/".into());
    assert!(
        base.starts_with('/')
            && base.ends_with('/')
            && !base.contains("..")
            && !base.contains(['"', '<', '>', '?', '#']),
        "base must be a slash-delimited URL path"
    );
    let template = fs::read_to_string(out.join("index.html"))?;
    let mut routes = vec![
        Route::Home,
        Route::About,
        Route::Experience,
        Route::Work,
        Route::Hobbies,
        Route::Gallery,
        Route::Blog,
        Route::NotFound,
    ];
    routes.extend(
        content::posts()
            .into_iter()
            .map(|p| Route::Article { slug: p.slug }),
    );
    for route in routes {
        let path = route.to_path();
        let props = AppProps {
            path: path.clone(),
            base: base.clone(),
        };
        let body = yew::ServerRenderer::<App>::with_props(move || props)
            .render()
            .await;
        let title = format!("{} — Sebastian Lopez Silva", pages::title(&route));
        let desc = pages::description(&route);
        let html=template.replace("<title>SEBLS portfolio</title>",&format!("<title>{}</title>",escape(&title)))
          .replace("content=\"Engineering, applied AI, and research by Sebastian Lopez Silva.\"",&format!("content=\"{}\"",escape(&desc)))
          .replace("</head>",&format!("<meta property=\"og:title\" content=\"{}\"><meta property=\"og:description\" content=\"{}\"><meta property=\"og:type\" content=\"{}\"></head>",escape(&title),escape(&desc),if matches!(route,Route::Article{..}){"article"}else{"website"}))
          .replace("<div id=\"app\"></div>",&format!("<div id=\"app\" data-base=\"{}\" data-route=\"{}\">{body}</div>",escape(&base),escape(&path)));
        assert!(html.contains("<main"), "Trunk template insertion failed");
        let file = if route == Route::NotFound {
            out.join("404.html")
        } else {
            out.join(path.trim_matches('/')).join("index.html")
        };
        fs::create_dir_all(file.parent().unwrap())?;
        fs::write(&file, html)?;
        println!("Rendered {} -> {}", href(&base, &route), file.display());
    }
    for photo in content::photos() {
        assert!(
            photo.file.starts_with("assets/images/") && !photo.file.contains(".."),
            "gallery file must be inside assets/images"
        );
        assert!(
            !photo.alt.trim().is_empty() && photo.width > 0 && photo.height > 0,
            "photo needs alt text and dimensions"
        );
        let dest = out.join(&photo.file);
        fs::create_dir_all(dest.parent().unwrap())?;
        fs::copy(&photo.file, dest)?;
    }
    for post in content::posts() {
        for asset in post.assets {
            let path = std::path::Path::new(&asset);
            assert!(
                asset.starts_with("assets/images/")
                    && path
                        .components()
                        .all(|part| matches!(part, std::path::Component::Normal(_))),
                "article assets must be inside assets/images"
            );
            let dest = out.join(&asset);
            fs::create_dir_all(dest.parent().unwrap())?;
            fs::copy(&asset, dest)?;
        }
    }
    fs::write(out.join(".nojekyll"), "")?;
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
#[cfg(target_arch = "wasm32")]
fn main() {}
