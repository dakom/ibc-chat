use dominator::clone;
use futures_signals::{map_ref, signal::{always, Mutable, Signal, SignalExt}};
use gloo_events::EventListener;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaQueryWidth {
    SmallPhone,
    Phone,
    Tablet,
    SmallDesktop,
    Desktop
}

impl MediaQueryWidth {
    pub fn signal() -> impl Signal<Item = MediaQueryWidth> {
        WINDOW_SIZE.with(|window_size| {
            window_size.query_width.signal()
        })
    }

    const fn cutoff_size(self) -> f64 {
        match self {
            MediaQueryWidth::SmallPhone => 600.0,
            MediaQueryWidth::Phone => 768.0,
            MediaQueryWidth::Tablet => 992.0,
            MediaQueryWidth::SmallDesktop => 1200.0,
            MediaQueryWidth::Desktop => panic!("Desktop has no cutoff size"),
        }
    }

    // based on a ratio to Desktop size
    pub fn scale_factor(&self) -> f64 {
        match self {
            MediaQueryWidth::Desktop => 1.0,
            _ => self.cutoff_size() / MediaQueryWidth::SmallDesktop.cutoff_size()
        }
    }
}

impl From<f64> for MediaQueryWidth {
    fn from(value: f64) -> Self {
        if value < MediaQueryWidth::SmallPhone.cutoff_size() {
            MediaQueryWidth::SmallPhone
        } else if value < MediaQueryWidth::Phone.cutoff_size() {
            MediaQueryWidth::Phone
        } else if value < MediaQueryWidth::Tablet.cutoff_size() {
            MediaQueryWidth::Tablet
        } else if value < MediaQueryWidth::SmallDesktop.cutoff_size() {
            MediaQueryWidth::SmallDesktop
        } else {
            MediaQueryWidth::Desktop
        }
    }
}

thread_local! {
    static WINDOW_SIZE: WindowSize = {
        let window = web_sys::window().unwrap();
        let width_value = window.inner_width().unwrap().as_f64().unwrap();
        let width = Mutable::new(width_value);
        let query_width = Mutable::new(width_value.into());

        let listener = {
            EventListener::new(&window, "resize", clone!(window, width, query_width => move |event| {
                let width_value = window.inner_width().unwrap().as_f64().unwrap();
                width.set_neq(width_value);
                query_width.set_neq(width_value.into());
            }))
        };

        WindowSize {
            width, 
            query_width, 
            listener
        }
    };
}

struct WindowSize {
    width: Mutable<f64>,
    query_width: Mutable<MediaQueryWidth>,
    listener: EventListener,
}