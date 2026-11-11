use yew::prelude::*;

#[cfg(any(target_arch = "wasm32", test))]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod particles;

#[derive(Properties, PartialEq)]
pub struct CatProps {
    pub base: String,
}

/// Every point illustration uses the same fallback and animation lifecycle.
/// Keep the SVG and its matching authoring JSON together when adding artwork.
#[derive(Properties, PartialEq)]
struct PointArtworkProps {
    surface: NodeRef,
    base: String,
    name: &'static str,
    data: &'static str,
    class: &'static str,
    #[prop_or_default]
    alt: &'static str,
}

#[function_component(PointArtwork)]
fn point_artwork(props: &PointArtworkProps) -> Html {
    let canvas = use_node_ref();
    let artwork = use_node_ref();
    {
        let surface = props.surface.clone();
        let canvas = canvas.clone();
        let artwork = artwork.clone();
        use_effect_with(props.data, move |data| {
            #[cfg(target_arch = "wasm32")]
            let cleanup = particles::mount(surface, canvas, artwork, data);
            #[cfg(not(target_arch = "wasm32"))]
            let cleanup = {
                let _ = (surface, canvas, artwork, data);
                || {}
            };
            cleanup
        });
    }
    html! {
        <div class={classes!("point-artwork", props.class)} ref={artwork}>
            <img class="point-fallback" src={format!("{}{}-points.svg", props.base, props.name)} alt={props.alt}/>
            <canvas class="point-particles" ref={canvas} aria-hidden="true"/>
        </div>
    }
}

#[function_component(Cat)]
pub fn cat(props: &CatProps) -> Html {
    let surface = use_node_ref();
    // Listing an illustration here opts it into the shared renderer automatically.
    let image = |name, data, class, alt| {
        html! {
            <PointArtwork surface={surface.clone()} base={props.base.clone()} {name} {data} {class} {alt}/>
        }
    };
    html! {<figure class="cat-figure"><div class="cat-surface" ref={surface.clone()}>
        {image("colombia-flag", include_str!("../../assets/images/colombia-flag-points.json"), "surface-flag", "Colombian flag made of colored dots")}
        <div class="plant-cluster" aria-hidden="true">
            {image("plant-1", include_str!("../../assets/images/plant-1-points.json"), "plant", "")}
            {image("plant-2", include_str!("../../assets/images/plant-2-points.json"), "plant", "")}
            {image("plant-3", include_str!("../../assets/images/plant-3-points.json"), "plant", "")}
            {image("plant-4", include_str!("../../assets/images/plant-4-points.json"), "plant", "")}
            {image("plant-5", include_str!("../../assets/images/plant-5-points.json"), "plant", "")}
        </div>
        <div class="cat-orbit" aria-hidden="true"/>
        {image("sleeping-cat", include_str!("../../assets/images/sleeping-cat-points.json"), "cat-art", "A black and white sleeping cat made of colored dots")}
        {image("sleep-marks", include_str!("../../assets/images/sleep-marks-points.json"), "cat-zzz", "")}
    </div><figcaption><span aria-hidden="true">{"please let Aiden (my cat) sleep"}</span></figcaption></figure>}
}
