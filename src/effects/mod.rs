use yew::prelude::*;

#[cfg(any(target_arch = "wasm32", test))]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod particles;

#[derive(Properties, PartialEq)]
pub struct CatProps {
    pub base: String,
}

/// Artwork registration is separate from physics. Order is also drawing order.
struct Figure {
    name: &'static str,
    data: &'static str,
    class: &'static str,
    alt: &'static str,
}
macro_rules! figure {
    ($name:literal, $class:literal, $alt:literal) => {
        Figure {
            name: $name,
            data: include_str!(concat!("../../assets/images/", $name, "-points.json")),
            class: $class,
            alt: $alt,
        }
    };
}
const FIGURES: &[Figure] = &[
    figure!(
        "colombia-flag",
        "surface-flag",
        "Colombian flag made of colored dots"
    ),
    figure!("plant-1", "plant", ""),
    figure!("plant-2", "plant", ""),
    figure!("plant-3", "plant", ""),
    figure!("plant-4", "plant", ""),
    figure!("plant-5", "plant", ""),
    figure!(
        "sleeping-cat",
        "cat-art",
        "A black and white sleeping cat made of colored dots"
    ),
    figure!("sleep-marks", "cat-zzz", ""),
];

#[function_component(Cat)]
pub fn cat(props: &CatProps) -> Html {
    let surface = use_node_ref();
    let canvas = use_node_ref();
    let nodes = use_memo((), |_| {
        FIGURES
            .iter()
            .map(|_| NodeRef::default())
            .collect::<Vec<_>>()
    });
    {
        let (surface, canvas, nodes) = (surface.clone(), canvas.clone(), nodes.clone());
        use_effect_with((), move |_| {
            let figures = nodes
                .iter()
                .cloned()
                .zip(FIGURES.iter().map(|f| f.data))
                .collect();
            #[cfg(target_arch = "wasm32")]
            let cleanup = particles::mount(surface, canvas, figures);
            #[cfg(not(target_arch = "wasm32"))]
            let cleanup = {
                let _: Vec<(NodeRef, &'static str)> = figures;
                let _ = (surface, canvas);
                || {}
            };
            cleanup
        });
    }
    let image = |i: usize| {
        let f = &FIGURES[i];
        html! {<div class={classes!("point-artwork", f.class)} ref={nodes[i].clone()}>
            <img class="point-fallback" src={format!("{}{}-points.svg", props.base, f.name)} alt={f.alt}/>
        </div>}
    };
    html! {<figure class="cat-figure"><div class="cat-surface" ref={surface}>
        {for FIGURES.iter().enumerate().filter(|(_, f)| f.class != "plant").map(|(i, _)| image(i))}
        <div class="plant-cluster" aria-hidden="true">
            {for FIGURES.iter().enumerate().filter(|(_, f)| f.class == "plant").map(|(i, _)| image(i))}
        </div>
        <canvas class="scene-particles" ref={canvas} aria-hidden="true"/>
    </div><figcaption><span aria-hidden="true">{"please let Aiden (my cat) sleep"}</span></figcaption></figure>}
}
