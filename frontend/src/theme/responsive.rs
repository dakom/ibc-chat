use dominator::clone;
use futures_signals::{map_ref, signal::{always, Mutable, Signal, SignalExt}};
use gloo_events::EventListener;
use wasm_bindgen::JsCast;

thread_local! {
    static WINDOW_SIZE: WindowSize = {
        let window = web_sys::window().unwrap();
        let width_value = window.inner_width().unwrap().as_f64().unwrap();
        let width = Mutable::new(width_value);

        let listener = {
            EventListener::new(&window, "resize", clone!(window, width => move |event| {
                let width_value = window.inner_width().unwrap().as_f64().unwrap();
                width.set_neq(width_value);
            }))
        };

        WindowSize {
            width, 
            listener
        }
    };
}

pub struct WindowSize {
    width: Mutable<f64>,
    listener: EventListener,
}

impl WindowSize {
    pub fn width_signal() -> impl Signal<Item = f64> {
        WINDOW_SIZE.with(|s| s.width.signal())
    }
}