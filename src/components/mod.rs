use crate::{
    content, pages,
    routes::{href, Route},
};
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct ShellProps {
    pub route: Route,
    pub base: String,
}
#[function_component(Shell)]
pub fn shell(props: &ShellProps) -> Html {
    let open = use_state(|| false);
    use_effect_with((), |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if window.location().hash().unwrap_or_default().is_empty() {
                    if let Ok(Some(heading)) = document.query_selector("main h1") {
                        use wasm_bindgen::JsCast;
                        let _ = heading.set_attribute("tabindex", "-1");
                        if let Ok(heading) = heading.dyn_into::<web_sys::HtmlElement>() {
                            let _ = heading.focus();
                        }
                    }
                }
            }
        }
        || ()
    });
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Escape" {
                open.set(false);
                if let Some(el) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id("menu-toggle"))
                {
                    use wasm_bindgen::JsCast;
                    if let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() {
                        let _ = el.focus();
                    }
                }
            }
        })
    };
    let sections = pages::sections(&props.route);
    html! {
      <div class="site" onkeydown={close}>
        <a class="skip" href="#main">{"Skip to content"}</a>
        <aside class="rail">
          <div class="rail-top"><a class="brand" href={href(&props.base, &Route::Home)} aria-label="SEBLS — Home">{"Sebls"}</a>
          <span class="mobile-name">{"Sebastian Lopez Silva"}</span>
          <button id="menu-toggle" class="menu-toggle" aria-expanded={open.to_string()} aria-controls="navigation" onclick={toggle}>{if *open {"Close menu"} else {"Menu"}}</button></div>
          <div id="navigation" class={classes!("navigation", (*open).then_some("is-open"))}>
            <nav aria-label="Site pages"><p class="nav-label">{"Explore"}</p>
              {for [(Route::Home,"Home"),(Route::About,"About"),(Route::Experience,"Experience"),(Route::Work,"Work"),(Route::Hobbies,"Hobbies"),(Route::Gallery,"Gallery"),(Route::Blog,"Blog")].into_iter().map(|(route,label)| {
                let active = props.route == route || (route == Route::Blog && matches!(props.route,Route::Article{..}));
                html!{<a class={classes!("page-link",active.then_some("active"))} aria-current={if active {Some("page")} else {None}} href={href(&props.base,&route)}>{label}<span class="nav-arrow" aria-hidden="true">{arrow("up-right")}</span></a>}
              })}
            </nav>
            if !sections.is_empty() { <PageIndex sections={sections}/> }
            <div class="rail-note"><span class="location"><svg class="location-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false"><path d="M20 10c0 6-8 12-8 12S4 16 4 10a8 8 0 1 1 16 0Z"/><circle cx="12" cy="10" r="2.5"/></svg>{"Based in Paris, France"}</span><p>{"Engineering & curiosity."}</p></div>
          </div>
        </aside>
        <div class="content-column">
          <main id="main" tabindex="-1">{pages::render(&props.route,&props.base)}</main>
          <footer><span>{"© Sebastian Lopez Silva"}</span><div>{social_links()}</div></footer>
        </div>
      </div>
    }
}
#[derive(Properties, PartialEq)]
pub struct IndexProps {
    pub sections: Vec<(String, String)>,
}
#[function_component(PageIndex)]
fn page_index(props: &IndexProps) -> Html {
    let selected = use_state(String::new);
    {
        let selected = selected.clone();
        use_effect_with((), move |_| {
            use wasm_bindgen::{closure::Closure, JsCast};
            let window = web_sys::window().expect("browser effect");
            let update = move || {
                if let Some(w) = web_sys::window() {
                    selected.set(
                        w.location()
                            .hash()
                            .unwrap_or_default()
                            .trim_start_matches('#')
                            .to_owned(),
                    );
                }
            };
            update();
            let listener = Closure::<dyn Fn(web_sys::Event)>::new(move |_| update());
            let _ = window
                .add_event_listener_with_callback("hashchange", listener.as_ref().unchecked_ref());
            move || {
                let _ = window.remove_event_listener_with_callback(
                    "hashchange",
                    listener.as_ref().unchecked_ref(),
                );
            }
        });
    }
    html! {<nav class="page-index" aria-label="On this page"><p class="nav-label">{"On this page"}</p>{for props.sections.iter().map(|(id,label)| {
      let selected=selected.clone(); let clicked=id.clone();
      html!{<a href={format!("#{id}")} aria-current={if *selected==*id {Some("location")}else{None}} onclick={Callback::from(move |_| selected.set(clicked.clone()))}>{label}</a>}
    })}</nav>}
}
pub fn entries(entries: Vec<content::Entry>) -> Html {
    html! {<div class="entries">{for entries.into_iter().map(|e|html!{<section class="entry" id={e.id} tabindex="-1"><p class="eyebrow">{e.dates}</p><h3>if e.url.is_empty(){{e.title}}else{<a href={e.url}>{e.title}{arrow("up-right")}</a>}</h3><p class="organization">if e.organization_url.is_empty(){{e.organization}}else{<a href={e.organization_url}>{e.organization}</a>}</p><p>{e.summary}</p>if !e.detail.is_empty(){<p>{e.detail}</p>}</section>})}</div>}
}

/// Decorative arrows share a light stroke and rounded ends at every link size.
pub fn arrow(direction: &str) -> Html {
    let path = match direction {
        "left" => "M19 12H5m6-6-6 6 6 6",
        "right" => "M5 12h14m-6-6 6 6-6 6",
        _ => "M6 18 18 6M6 6h12v12",
    };
    html! {<svg class="link-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false"><path d={path}/></svg>}
}

pub fn social_links() -> Html {
    html! {<>
        <a class="social-link" href="https://github.com/Sebls">
            <svg class="social-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" focusable="false"><path d="M12 .75a11.25 11.25 0 0 0-3.56 21.92c.56.1.77-.24.77-.54v-2.1c-3.13.68-3.79-1.33-3.79-1.33-.51-1.3-1.25-1.65-1.25-1.65-1.02-.7.08-.68.08-.68 1.13.08 1.73 1.16 1.73 1.16 1 1.72 2.63 1.22 3.27.93.1-.73.39-1.22.71-1.5-2.5-.28-5.13-1.25-5.13-5.56 0-1.23.44-2.23 1.16-3.02-.12-.28-.5-1.43.11-2.98 0 0 .95-.3 3.1 1.15a10.8 10.8 0 0 1 5.64 0c2.15-1.45 3.1-1.15 3.1-1.15.61 1.55.23 2.7.11 2.98.72.79 1.16 1.79 1.16 3.02 0 4.32-2.63 5.28-5.14 5.56.4.35.76 1.03.76 2.08v3.09c0 .3.2.65.78.54A11.25 11.25 0 0 0 12 .75Z"/></svg>
            <span>{"GitHub"}</span>
        </a>
        <a class="social-link" href="https://linkedin.com/in/slopezsi">
            <svg class="social-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" focusable="false"><path d="M20.45 2H3.55C2.69 2 2 2.68 2 3.53v16.94c0 .85.69 1.53 1.55 1.53h16.9c.86 0 1.55-.68 1.55-1.53V3.53c0-.85-.69-1.53-1.55-1.53ZM7.93 18.75H4.96V9.2h2.97v9.55ZM6.45 7.9a1.72 1.72 0 1 1 0-3.44 1.72 1.72 0 0 1 0 3.44Zm12.3 10.85h-2.96V14.1c0-1.11-.02-2.54-1.55-2.54-1.55 0-1.79 1.21-1.79 2.46v4.73H9.48V9.2h2.85v1.3h.04c.4-.75 1.37-1.55 2.82-1.55 3.02 0 3.56 1.99 3.56 4.57v5.23Z"/></svg>
            <span>{"LinkedIn"}</span>
        </a>
    </>}
}
