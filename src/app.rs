use crate::{components::Shell, routes::Route};
use yew::prelude::*;
use yew_router::{
    history::{History, MemoryHistory},
    prelude::*,
};
#[derive(Properties, PartialEq, Clone)]
pub struct AppProps {
    pub path: String,
    pub base: String,
}
#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let history = use_state(|| {
        let h = MemoryHistory::new();
        h.push(props.path.clone());
        h
    });
    let base = props.base.clone();
    html! { <Router history={yew_router::history::AnyHistory::from((*history).clone())}><Switch<Route> render={move |route| html!{<Shell route={route} base={base.clone()}/>}} /></Router> }
}
