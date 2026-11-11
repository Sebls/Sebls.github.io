#[cfg(target_arch = "wasm32")]
fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("app").unwrap();
    let base = root.get_attribute("data-base").unwrap_or("/".into());
    let path = root.get_attribute("data-route").unwrap_or("/".into());
    yew::Renderer::<portfolio::app::App>::with_root_and_props(
        root,
        portfolio::app::AppProps { path, base },
    )
    .hydrate();
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
