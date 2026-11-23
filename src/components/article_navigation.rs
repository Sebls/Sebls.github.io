use super::PageIndex;
use crate::routes::{href, Route};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleNavigationProps {
    pub sections: Vec<(String, String)>,
    pub base: String,
}

/// Keep the full index at the start, then expose the current section while reading.
#[function_component(ArticleNavigation)]
pub fn article_navigation(props: &ArticleNavigationProps) -> Html {
    let active = use_state_eq(|| None::<usize>);
    let expanded = use_state(|| false);
    let button = use_node_ref();
    {
        let active = active.clone();
        use_effect_with(props.sections.clone(), move |sections| {
            let window = web_sys::window().expect("browser effect");
            let headings: Vec<_> = window
                .document()
                .into_iter()
                .flat_map(|document| {
                    sections
                        .iter()
                        .filter_map(move |(id, _)| document.get_element_by_id(id))
                })
                .collect();
            let update = move || {
                let current = headings
                    .iter()
                    .enumerate()
                    .filter(|(_, heading)| heading.get_bounding_client_rect().top() <= 100.0)
                    .map(|(index, _)| index)
                    .next_back();
                active.set(current);
            };
            update();
            let listener = Closure::<dyn Fn(web_sys::Event)>::new(move |_| update());
            for event in ["scroll", "resize", "pageshow"] {
                let _ = window
                    .add_event_listener_with_callback(event, listener.as_ref().unchecked_ref());
            }
            move || {
                for event in ["scroll", "resize", "pageshow"] {
                    let _ = window.remove_event_listener_with_callback(
                        event,
                        listener.as_ref().unchecked_ref(),
                    );
                }
            }
        });
    }
    // Close a stale disclosure when the reader moves into a different section.
    {
        let expanded = expanded.clone();
        use_effect_with(*active, move |_| expanded.set(false));
    }
    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };
    let escape = {
        let expanded = expanded.clone();
        let button = button.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                expanded.set(false);
                if let Some(button) = button.cast::<web_sys::HtmlElement>() {
                    let _ = button.focus();
                }
            }
        })
    };
    let reading = active.is_some();
    let index = active.unwrap_or(0);
    html! {<>
        <aside class={classes!("article-index", reading.then_some("is-collapsed"))}
            aria-hidden={reading.then_some("true")} inert={reading}>
            <PageIndex sections={props.sections.clone()}/>
        </aside>
            <div class={classes!("article-section-bar", reading.then_some("is-visible"))}
                aria-hidden={(!reading).then_some("true")} inert={!reading} onkeydown={escape}>
                <div class="section-bar-inner">
                  <div class="section-bar-controls">
                    <button ref={button} id="article-section-toggle" class="section-bar-toggle"
                        aria-expanded={expanded.to_string()} aria-controls="article-section-menu"
                        aria-label={format!("Contents: {}", props.sections[index].1)} onclick={toggle}>
                        <span class="section-number" aria-hidden="true">{format!("{:02}", index + 1)}</span>
                        <span>{&props.sections[index].1}</span>
                        <svg class={classes!("section-chevron", (*expanded).then_some("is-open"))} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
                    </button>
                    <a class="section-bar-back" href={href(&props.base, &Route::Blog)}>{"All writing"}</a>
                  </div>
                    <nav id="article-section-menu" class="section-bar-menu" aria-label="Article sections" hidden={!*expanded}>
                        {for props.sections.iter().enumerate().map(|(i, (id, label))| {
                            let expanded = expanded.clone();
                            html!{<a href={format!("#{id}")} aria-current={if i == index {Some("location")} else {None}}
                                onclick={Callback::from(move |_| expanded.set(false))}>
                                <span class="section-number" aria-hidden="true">{format!("{:02}", i + 1)}</span>{label}
                            </a>}
                        })}
                    </nav>
                </div>
            </div>
    </>}
}
