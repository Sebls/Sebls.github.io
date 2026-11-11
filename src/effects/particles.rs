//! Local, on-demand particle rendering. No page-level pointer handling.
use std::{cell::RefCell, f64::consts::TAU, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, PointerEvent};
use yew::NodeRef;

const STATIC_MEDIA: &str = "(prefers-reduced-motion: reduce), (pointer: coarse), (hover: none)";
// CSS-pixel radii, independent of the artwork's responsive size.
const INFLUENCE: f64 = 40.0;
const CORE: f64 = 14.0;
type Frame = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

#[derive(serde::Deserialize)]
struct Artwork {
    points: Vec<[f64; 5]>,
    width: f64,
    height: f64,
    radius: f64,
}
struct Point {
    home: (f64, f64),
    pos: (f64, f64),
    velocity: (f64, f64),
    phase: f64,
    color: String,
}
impl Point {
    /// A driven orbit keeps a stationary field alive; velocity and damping make
    /// pointer changes and the return home continuous rather than position snaps.
    fn advance(
        &mut self,
        pointer: (f64, f64),
        scale: f64,
        strength: f64,
        dot_radius: f64,
        dt_ms: f64,
    ) -> bool {
        let dx = self.home.0 - pointer.0;
        let dy = self.home.1 - pointer.1;
        let distance = dx.hypot(dy);
        let influence = INFLUENCE * scale;
        let falloff = (1.0 - distance / influence).max(0.0).powi(2);
        let driven = falloff * strength > 0.001;
        // Bounded substeps keep the spring stable across display refresh rates.
        let seconds = dt_ms.clamp(0.0, 50.0) / 1000.0;
        let steps = (seconds * 120.0).ceil().max(1.0) as u32;
        let h = seconds / f64::from(steps);
        for _ in 0..steps {
            let mut target = self.home;
            if driven {
                self.phase = (self.phase + 2.8 * falloff * strength * h) % TAU;
                let angle = if distance < 0.01 {
                    self.home.0 * 2.399
                } else {
                    dy.atan2(dx)
                } + self.phase;
                let displaced = (distance + CORE * 2.5 * scale * falloff * strength)
                    .max(CORE * scale * strength + dot_radius);
                // Fade orbital displacement back to the exact artwork on exit.
                target = (
                    self.home.0 + (pointer.0 + angle.cos() * displaced - self.home.0) * strength,
                    self.home.1 + (pointer.1 + angle.sin() * displaced - self.home.1) * strength,
                );
            }
            self.velocity.0 += ((target.0 - self.pos.0) * 90.0 - self.velocity.0 * 10.0) * h;
            self.velocity.1 += ((target.1 - self.pos.1) * 90.0 - self.velocity.1 * 10.0) * h;
            self.pos.0 += self.velocity.0 * h;
            self.pos.1 += self.velocity.1 * h;
        }
        let moving = (self.pos.0 - self.home.0).hypot(self.pos.1 - self.home.1) > 0.03 * scale
            || self.velocity.0.hypot(self.velocity.1) > 0.1 * scale;
        if !driven && !moving {
            self.pos = self.home;
            self.velocity = (0.0, 0.0);
            self.phase = 0.0;
        }
        driven || moving
    }
}
struct Field {
    points: Vec<Point>,
    pointer: (f64, f64),
    active: bool,
    strength: f64,
    scale: f64,
    last: f64,
    pending: Option<i32>,
    width: f64,
    height: f64,
    radius: f64,
}
impl Field {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, dt: f64) -> bool {
        let blend = 1.0 - (-dt / 65.0).exp();
        self.strength += (f64::from(self.active) - self.strength) * blend;
        let core = CORE * self.scale * self.strength;
        let mut unsettled = (f64::from(self.active) - self.strength).abs() > 0.001;
        ctx.clear_rect(0.0, 0.0, self.width, self.height);
        for point in &mut self.points {
            unsettled |= point.advance(self.pointer, self.scale, self.strength, self.radius, dt);
            ctx.set_fill_style_str(&point.color);
            ctx.begin_path();
            let _ = ctx.arc(point.pos.0, point.pos.1, self.radius, 0.0, TAU);
            ctx.fill();
        }
        if core > 0.1 {
            ctx.set_fill_style_str("#FFAC00");
            ctx.begin_path();
            let _ = ctx.arc(self.pointer.0, self.pointer.1, core, 0.0, TAU);
            ctx.fill();
        }
        unsettled
    }
    fn reset(&mut self) {
        self.active = false;
        self.strength = 0.0;
        self.last = 0.0;
        for p in &mut self.points {
            p.pos = p.home;
            p.velocity = (0.0, 0.0);
            p.phase = 0.0;
        }
        if let Some(id) = self.pending.take() {
            let _ = web_sys::window().unwrap().cancel_animation_frame(id);
        }
    }
}
fn schedule(field: &Rc<RefCell<Field>>, frame: &Frame) {
    let mut field = field.borrow_mut();
    if field.pending.is_none() {
        if let Some(callback) = frame.borrow().as_ref() {
            field.pending = web_sys::window()
                .unwrap()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .ok();
        }
    }
}

pub fn mount(surface: NodeRef, canvas: NodeRef, artwork: NodeRef, data: &str) -> Box<dyn FnOnce()> {
    let Some(window) = web_sys::window() else {
        return Box::new(|| {});
    };
    let Some(surface) = surface.cast::<HtmlElement>() else {
        return Box::new(|| {});
    };
    let Some(artwork) = artwork.cast::<HtmlElement>() else {
        return Box::new(|| {});
    };
    let Some(canvas) = canvas.cast::<HtmlCanvasElement>() else {
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
    let Ok(art) = serde_json::from_str::<Artwork>(data) else {
        return Box::new(|| {});
    };
    canvas.set_width(art.width as u32);
    canvas.set_height(art.height as u32);
    let field = Rc::new(RefCell::new(Field {
        points: art
            .points
            .into_iter()
            .map(|p| Point {
                home: (p[0], p[1]),
                pos: (p[0], p[1]),
                velocity: (0.0, 0.0),
                phase: 0.0,
                color: format!("rgb({},{},{})", p[2], p[3], p[4]),
            })
            .collect(),
        pointer: (0.0, 0.0),
        active: false,
        strength: 0.0,
        scale: 1.0,
        last: 0.0,
        pending: None,
        width: art.width,
        height: art.height,
        radius: art.radius,
    }));
    let frame: Frame = Rc::new(RefCell::new(None));
    let document = window.document().unwrap();
    *frame.borrow_mut() = Some(Closure::wrap(Box::new({
        let (field, frame, surface, artwork, ctx, media, document) = (
            field.clone(),
            frame.clone(),
            surface.clone(),
            artwork.clone(),
            ctx.clone(),
            media.clone(),
            document.clone(),
        );
        move |now: f64| {
            let rect = surface.get_bounding_client_rect();
            let height = web_sys::window()
                .unwrap()
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let mut state = field.borrow_mut();
            state.pending = None;
            if media.matches() || document.hidden() || rect.bottom() <= 0.0 || rect.top() >= height
            {
                state.reset();
                let _ = artwork.remove_attribute("data-particles");
                return;
            }
            let dt = if state.last == 0.0 {
                16.0
            } else {
                (now - state.last).clamp(1.0, 50.0)
            };
            state.last = now;
            let more = state.draw(&ctx, dt);
            if !more {
                state.last = 0.0;
                if !state.active {
                    state.reset();
                    let _ = artwork.remove_attribute("data-particles");
                }
            }
            drop(state);
            if more {
                schedule(&field, &frame);
            }
        }
    }) as Box<dyn FnMut(f64)>));
    let movement = Closure::wrap(Box::new({
        let (field, frame, canvas, artwork, media) = (
            field.clone(),
            frame.clone(),
            canvas.clone(),
            artwork.clone(),
            media.clone(),
        );
        move |event: PointerEvent| {
            if media.matches() || event.pointer_type() != "mouse" {
                return;
            }
            let Some(window) = web_sys::window() else {
                return;
            };
            let Ok(Some(style)) = window.get_computed_style(&artwork) else {
                return;
            };
            if style.get_property_value("display").unwrap_or_default() == "none" {
                return;
            }
            let rect = canvas.get_bounding_client_rect();
            // The canvas uses contain sizing inside the artwork box. Convert the
            // pointer back through its CSS rotation, then remove letterboxing.
            let width = f64::from(canvas.client_width());
            let height = f64::from(canvas.client_height());
            if width <= 0.0 || height <= 0.0 {
                return;
            }
            let transform = style.get_property_value("transform").unwrap_or_default();
            let matrix = transform
                .strip_prefix("matrix(")
                .and_then(|v| v.strip_suffix(')'))
                .map(|v| {
                    v.split(',')
                        .filter_map(|n| n.trim().parse::<f64>().ok())
                        .collect::<Vec<_>>()
                })
                .filter(|v| v.len() == 6)
                .unwrap_or_else(|| vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
            let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
            if determinant.abs() < 0.0001 {
                return;
            }
            let dx = f64::from(event.client_x()) - (rect.left() + rect.width() / 2.0);
            let dy = f64::from(event.client_y()) - (rect.top() + rect.height() / 2.0);
            let x = (matrix[3] * dx - matrix[2] * dy) / determinant + width / 2.0;
            let y = (-matrix[1] * dx + matrix[0] * dy) / determinant + height / 2.0;
            let mut state = field.borrow_mut();
            let fit = (width / state.width).min(height / state.height);
            state.scale = 1.0 / fit;
            state.pointer = (
                (x - (width - state.width * fit) / 2.0) / fit,
                (y - (height - state.height * fit)) / fit,
            );
            // Points near the edge also react when the pointer is just outside
            // their image box, avoiding seams between adjacent illustrations.
            let margin = INFLUENCE * state.scale;
            state.active = state.pointer.0 >= -margin
                && state.pointer.1 >= -margin
                && state.pointer.0 <= state.width + margin
                && state.pointer.1 <= state.height + margin;
            if !state.active && state.strength == 0.0 {
                return;
            }
            let _ = artwork.set_attribute("data-particles", "true");
            drop(state);
            schedule(&field, &frame);
        }
    }) as Box<dyn FnMut(PointerEvent)>);
    let leave = Closure::wrap(Box::new({
        let (field, frame) = (field.clone(), frame.clone());
        move |_: web_sys::Event| {
            field.borrow_mut().active = false;
            schedule(&field, &frame);
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    let reset = Closure::wrap(Box::new({
        let (field, artwork) = (field.clone(), artwork.clone());
        move |_: web_sys::Event| {
            field.borrow_mut().reset();
            let _ = artwork.remove_attribute("data-particles");
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    let _ =
        surface.add_event_listener_with_callback("pointermove", movement.as_ref().unchecked_ref());
    for name in ["pointerleave", "pointercancel"] {
        let _ = surface.add_event_listener_with_callback(name, leave.as_ref().unchecked_ref());
    }
    let _ = media.add_event_listener_with_callback("change", reset.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("resize", reset.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("blur", reset.as_ref().unchecked_ref());
    let _ = document
        .add_event_listener_with_callback("visibilitychange", reset.as_ref().unchecked_ref());
    Box::new(move || {
        field.borrow_mut().reset();
        frame.borrow_mut().take(); // Break the animation closure cycle on unmount.
        let _ = surface
            .remove_event_listener_with_callback("pointermove", movement.as_ref().unchecked_ref());
        for name in ["pointerleave", "pointercancel"] {
            let _ =
                surface.remove_event_listener_with_callback(name, leave.as_ref().unchecked_ref());
        }
        let _ = media.remove_event_listener_with_callback("change", reset.as_ref().unchecked_ref());
        let _ =
            window.remove_event_listener_with_callback("resize", reset.as_ref().unchecked_ref());
        let _ = window.remove_event_listener_with_callback("blur", reset.as_ref().unchecked_ref());
        let _ = document.remove_event_listener_with_callback(
            "visibilitychange",
            reset.as_ref().unchecked_ref(),
        );
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point() -> Point {
        Point {
            home: (12.0, 4.0),
            pos: (12.0, 4.0),
            velocity: (0.0, 0.0),
            phase: 0.0,
            color: String::new(),
        }
    }

    #[test]
    fn stationary_pointer_keeps_particles_moving() {
        let mut p = point();
        for _ in 0..300 {
            assert!(p.advance((0.0, 0.0), 1.0, 1.0, 1.0, 16.0));
        }
        let before = p.pos;
        p.advance((0.0, 0.0), 1.0, 1.0, 1.0, 16.0);
        assert!((p.pos.0 - before.0).hypot(p.pos.1 - before.1) > 0.05);
        assert!(p.velocity.0.hypot(p.velocity.1) > 1.0);
    }

    #[test]
    fn exit_preserves_momentum_then_settles_exactly() {
        let mut p = point();
        for _ in 0..100 {
            p.advance((0.0, 0.0), 1.0, 1.0, 1.0, 16.0);
        }
        let before = p.pos;
        assert!(p.advance((0.0, 0.0), 1.0, 0.0, 1.0, 16.0));
        assert_ne!(p.pos, before);
        assert_ne!(p.pos, p.home);
        for _ in 0..300 {
            p.advance((0.0, 0.0), 1.0, 0.0, 1.0, 16.0);
        }
        assert_eq!(p.pos, p.home);
        assert_eq!(p.velocity, (0.0, 0.0));
        assert!(!p.advance((0.0, 0.0), 1.0, 0.0, 1.0, 16.0));
    }

    #[test]
    fn distant_points_stay_at_rest() {
        assert!(!point().advance((200.0, 200.0), 1.0, 1.0, 1.0, 16.0));
    }

    #[test]
    fn motion_is_consistent_across_refresh_rates() {
        let simulate = |hz| {
            let mut p = point();
            for _ in 0..hz * 4 {
                p.advance((0.0, 0.0), 1.0, 1.0, 1.0, 1000.0 / f64::from(hz));
            }
            p.pos
        };
        let reference = simulate(120);
        for hz in [30, 60, 144] {
            let pos = simulate(hz);
            assert!((pos.0 - reference.0).hypot(pos.1 - reference.1) < 3.0);
        }
    }
}
