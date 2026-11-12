//! One CSS-coordinate scene, one scheduler, and local pointer input.
use std::{cell::RefCell, f64::consts::TAU, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, PointerEvent};
use yew::NodeRef;

const STATIC_MEDIA: &str = "(prefers-reduced-motion: reduce), (pointer: coarse), (hover: none)";
const INFLUENCE: f64 = 40.0;
const MAX_SPEED: f64 = 1100.0;
type Frame = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
type Vec2 = (f64, f64);

#[derive(serde::Deserialize)]
struct Artwork {
    points: Vec<[f64; 5]>,
    width: f64,
    height: f64,
    radius: f64,
}
struct Point {
    home: Vec2,
    pos: Vec2,
    velocity: Vec2,
    radius: f64,
    color: String,
}
fn limited(v: Vec2, max: f64) -> Vec2 {
    let factor = (max / v.0.hypot(v.1).max(0.0001)).min(1.0);
    (v.0 * factor, v.1 * factor)
}
impl Point {
    fn advance(&mut self, pointer: Option<Vec2>, dt: f64) -> bool {
        // Sleeping points need no integration until the pointer reaches them.
        if self.pos == self.home
            && self.velocity == (0.0, 0.0)
            && pointer.is_none_or(|p| (self.home.0 - p.0).hypot(self.home.1 - p.1) >= INFLUENCE)
        {
            return false;
        }
        let steps = (dt * 120.0).ceil().max(1.0) as u32;
        let h = dt / f64::from(steps);
        for _ in 0..steps {
            let spring = if pointer.is_some() { 38.0 } else { 75.0 };
            let damping = if pointer.is_some() { 7.0 } else { 15.0 };
            let mut force = (
                (self.home.0 - self.pos.0) * spring,
                (self.home.1 - self.pos.1) * spring,
            );
            if let Some(mouse) = pointer {
                let dx = self.pos.0 - mouse.0;
                let dy = self.pos.1 - mouse.1;
                let distance = dx.hypot(dy);
                let falloff = (1.0 - distance / INFLUENCE).max(0.0).powi(2);
                if falloff > 0.0 {
                    let angle = if distance > 0.1 {
                        dy.atan2(dx)
                    } else {
                        self.home.0 * 2.399
                    };
                    let (nx, ny) = (angle.cos(), angle.sin());
                    force.0 += falloff * (nx * 2800.0 - ny * 2200.0);
                    force.1 += falloff * (ny * 2800.0 + nx * 2200.0);
                }
            }
            self.velocity = limited(
                (
                    self.velocity.0 + (force.0 - damping * self.velocity.0) * h,
                    self.velocity.1 + (force.1 - damping * self.velocity.1) * h,
                ),
                MAX_SPEED,
            );
            self.pos.0 += self.velocity.0 * h;
            self.pos.1 += self.velocity.1 * h;
        }
        let near_mouse = pointer.is_some_and(|p| {
            (self.home.0 - p.0).hypot(self.home.1 - p.1) < INFLUENCE
                || (self.pos.0 - p.0).hypot(self.pos.1 - p.1) < INFLUENCE
        });
        let moving = (self.pos.0 - self.home.0).hypot(self.pos.1 - self.home.1) > 0.04
            || self.velocity.0.hypot(self.velocity.1) > 0.15;
        if !moving && !near_mouse {
            self.pos = self.home;
            self.velocity = (0.0, 0.0);
        }
        moving || near_mouse
    }
}
struct Scene {
    points: Vec<Point>,
    pointer: Option<Vec2>,
    last: f64,
    pending: Option<i32>,
    dirty: bool,
    visible: bool,
    width: f64,
    height: f64,
}
impl Scene {
    fn reset(&mut self, surface: &HtmlElement) {
        self.pointer = None;
        self.last = 0.0;
        for p in &mut self.points {
            p.pos = p.home;
            p.velocity = (0.0, 0.0);
        }
        if let Some(id) = self.pending.take() {
            let _ = web_sys::window().unwrap().cancel_animation_frame(id);
        }
        let _ = surface.remove_attribute("data-particles");
    }
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dt: f64) -> bool {
        ctx.clear_rect(0.0, 0.0, self.width, self.height);
        let mut moving = false;
        for p in &mut self.points {
            moving |= p.advance(self.pointer, dt);
            ctx.set_fill_style_str(&p.color);
            ctx.begin_path();
            let _ = ctx.arc(p.pos.0, p.pos.1, p.radius, 0.0, TAU);
            ctx.fill();
        }
        moving
    }
}
fn schedule(scene: &Rc<RefCell<Scene>>, frame: &Frame) {
    let mut scene = scene.borrow_mut();
    if scene.pending.is_none() {
        if let Some(callback) = frame.borrow().as_ref() {
            scene.pending = web_sys::window()
                .unwrap()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .ok();
        }
    }
}

/// Resolve CSS geometry only when layout changes. Hidden figures are not parsed.
fn layout(
    surface: &HtmlElement,
    canvas: &HtmlCanvasElement,
    figures: &[(NodeRef, &'static str)],
) -> Option<Vec<Point>> {
    let window = web_sys::window()?;
    let rect = surface.get_bounding_client_rect();
    let width = f64::from(surface.client_width());
    let height = f64::from(surface.client_height());
    if width <= 0.0 || height <= 0.0 {
        return None;
    }
    let mut points = Vec::new();
    for (node, data) in figures {
        let node = node.cast::<HtmlElement>()?;
        let style = window.get_computed_style(&node).ok()??;
        if style.get_property_value("display").ok()? == "none" {
            continue;
        }
        let w = f64::from(node.client_width());
        let h = f64::from(node.client_height());
        if w == 0.0 || h == 0.0 {
            continue;
        }
        let art: Artwork = serde_json::from_str(data).ok()?;
        let fit = (w / art.width).min(h / art.height);
        let box_rect = node.get_bounding_client_rect();
        let transform = style.get_property_value("transform").ok()?;
        let m = transform
            .strip_prefix("matrix(")
            .and_then(|s| s.strip_suffix(')'))
            .map(|s| {
                s.split(',')
                    .filter_map(|v| v.trim().parse::<f64>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|m| m.len() == 6)
            .unwrap_or_else(|| vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let center = (
            box_rect.left() + box_rect.width() / 2.0 - rect.left(),
            box_rect.top() + box_rect.height() / 2.0 - rect.top(),
        );
        for p in art.points {
            let x = p[0] * fit - art.width * fit / 2.0;
            let y = p[1] * fit + h / 2.0 - art.height * fit;
            let home = (
                center.0 + m[0] * x + m[2] * y,
                center.1 + m[1] * x + m[3] * y,
            );
            points.push(Point {
                home,
                pos: home,
                velocity: (0.0, 0.0),
                radius: art.radius * fit,
                color: format!("rgb({},{},{})", p[2], p[3], p[4]),
            });
        }
    }
    let density = window.device_pixel_ratio().clamp(1.0, 2.0);
    canvas.set_width((width * density).ceil() as u32);
    canvas.set_height((height * density).ceil() as u32);
    let ctx = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;
    ctx.set_transform(density, 0.0, 0.0, density, 0.0, 0.0)
        .ok()?;
    Some(points)
}

pub fn mount(
    surface: NodeRef,
    canvas: NodeRef,
    figures: Vec<(NodeRef, &'static str)>,
) -> Box<dyn FnOnce()> {
    let Some(window) = web_sys::window() else {
        return Box::new(|| {});
    };
    let (Some(surface), Some(canvas)) = (
        surface.cast::<HtmlElement>(),
        canvas.cast::<HtmlCanvasElement>(),
    ) else {
        return Box::new(|| {});
    };
    let Some(ctx) = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|v| v.dyn_into::<CanvasRenderingContext2d>().ok())
    else {
        return Box::new(|| {});
    };
    let Ok(Some(media)) = window.match_media(STATIC_MEDIA) else {
        return Box::new(|| {});
    };
    let document = window.document().unwrap();
    let scene = Rc::new(RefCell::new(Scene {
        points: Vec::new(),
        pointer: None,
        last: 0.0,
        pending: None,
        dirty: true,
        visible: true,
        width: 0.0,
        height: 0.0,
    }));
    let frame: Frame = Rc::new(RefCell::new(None));
    *frame.borrow_mut() = Some(Closure::wrap(Box::new({
        let (scene, frame, surface, canvas, ctx, media, document) = (
            scene.clone(),
            frame.clone(),
            surface.clone(),
            canvas.clone(),
            ctx.clone(),
            media.clone(),
            document.clone(),
        );
        move |now: f64| {
            let mut s = scene.borrow_mut();
            s.pending = None;
            if media.matches() || document.hidden() || !s.visible {
                s.reset(&surface);
                return;
            }
            if s.dirty {
                let pointer = s.pointer;
                s.reset(&surface);
                s.pointer = pointer;
                let Some(points) = layout(&surface, &canvas, &figures) else {
                    return;
                };
                s.points = points;
                s.width = f64::from(surface.client_width());
                s.height = f64::from(surface.client_height());
                s.dirty = false;
            }
            let dt = if s.last == 0.0 {
                0.016
            } else {
                ((now - s.last) / 1000.0).clamp(0.001, 0.05)
            };
            s.last = now;
            let more = s.draw(&ctx, dt);
            if more {
                let _ = surface.set_attribute("data-particles", "true");
            } else {
                s.last = 0.0;
                let _ = surface.remove_attribute("data-particles");
            }
            drop(s);
            if more {
                schedule(&scene, &frame);
            }
        }
    }) as Box<dyn FnMut(f64)>));
    let movement = Closure::wrap(Box::new({
        let (scene, frame, surface, media) =
            (scene.clone(), frame.clone(), surface.clone(), media.clone());
        move |event: PointerEvent| {
            if media.matches() || event.pointer_type() != "mouse" {
                return;
            }
            let rect = surface.get_bounding_client_rect();
            let p = (
                f64::from(event.client_x()) - rect.left(),
                f64::from(event.client_y()) - rect.top(),
            );
            let mut s = scene.borrow_mut();
            if !s.visible {
                return;
            }
            s.pointer = Some(p);
            drop(s);
            schedule(&scene, &frame);
        }
    }) as Box<dyn FnMut(PointerEvent)>);
    let leave = Closure::wrap(Box::new({
        let (scene, frame) = (scene.clone(), frame.clone());
        move |_: web_sys::Event| {
            let mut s = scene.borrow_mut();
            s.pointer = None;
            drop(s);
            schedule(&scene, &frame);
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    let reset = Closure::wrap(Box::new({
        let (scene, surface) = (scene.clone(), surface.clone());
        move |_: web_sys::Event| {
            let mut s = scene.borrow_mut();
            s.reset(&surface);
            s.dirty = true;
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    let resized = Closure::wrap(Box::new({
        let (scene, surface) = (scene.clone(), surface.clone());
        move |_: js_sys::Array, _: web_sys::ResizeObserver| {
            let mut s = scene.borrow_mut();
            s.reset(&surface);
            s.dirty = true;
        }
    })
        as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);
    let resize_observer = web_sys::ResizeObserver::new(resized.as_ref().unchecked_ref()).ok();
    if let Some(observer) = &resize_observer {
        observer.observe(&surface);
    }
    let visibility = Closure::wrap(Box::new({
        let (scene, surface) = (scene.clone(), surface.clone());
        move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
            if let Some(entry) = entries
                .get(0)
                .dyn_ref::<web_sys::IntersectionObserverEntry>()
            {
                let mut s = scene.borrow_mut();
                s.visible = entry.is_intersecting();
                if !s.visible {
                    s.reset(&surface);
                }
            }
        }
    })
        as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);
    let observer = web_sys::IntersectionObserver::new(visibility.as_ref().unchecked_ref()).ok();
    if let Some(observer) = &observer {
        observer.observe(&surface);
    }
    let _ =
        surface.add_event_listener_with_callback("pointermove", movement.as_ref().unchecked_ref());
    for event in ["pointerleave", "pointercancel"] {
        let _ = surface.add_event_listener_with_callback(event, leave.as_ref().unchecked_ref());
    }
    for event in ["resize", "blur"] {
        let _ = window.add_event_listener_with_callback(event, reset.as_ref().unchecked_ref());
    }
    let _ = media.add_event_listener_with_callback("change", reset.as_ref().unchecked_ref());
    let _ = document
        .add_event_listener_with_callback("visibilitychange", reset.as_ref().unchecked_ref());
    Box::new(move || {
        scene.borrow_mut().reset(&surface);
        frame.borrow_mut().take();
        if let Some(observer) = resize_observer {
            observer.disconnect();
        }
        if let Some(observer) = observer {
            observer.disconnect();
        }
        let _ = surface
            .remove_event_listener_with_callback("pointermove", movement.as_ref().unchecked_ref());
        for event in ["pointerleave", "pointercancel"] {
            let _ =
                surface.remove_event_listener_with_callback(event, leave.as_ref().unchecked_ref());
        }
        for event in ["resize", "blur"] {
            let _ =
                window.remove_event_listener_with_callback(event, reset.as_ref().unchecked_ref());
        }
        let _ = media.remove_event_listener_with_callback("change", reset.as_ref().unchecked_ref());
        let _ = document.remove_event_listener_with_callback(
            "visibilitychange",
            reset.as_ref().unchecked_ref(),
        );
        drop((resized, visibility));
    })
}
