use yew_router::prelude::*;
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/experience")]
    Experience,
    #[at("/work")]
    Work,
    #[at("/gallery")]
    Gallery,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    Article { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub fn recognize(path: &str) -> Route {
    Route::recognize(if path == "/" {
        path
    } else {
        path.trim_end_matches('/')
    })
    .unwrap_or(Route::NotFound)
}
pub fn href(base: &str, route: &Route) -> String {
    let path = route.to_path();
    format!(
        "{}{}{}",
        base,
        path.trim_start_matches('/'),
        if path == "/" { "" } else { "/" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separate_document_paths_keep_hosting_prefix() {
        assert_eq!(
            href("/sebls-portfolio/", &Route::About),
            "/sebls-portfolio/about/"
        );
        assert_eq!(href("/", &Route::Home), "/");
        assert_eq!(recognize("/about/"), Route::About);
        assert_eq!(
            recognize("/blog/real-post/"),
            Route::Article {
                slug: "real-post".into()
            }
        );
        assert_eq!(recognize("/unknown/path"), Route::NotFound);
    }
}
