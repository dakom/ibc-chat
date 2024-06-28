use web_sys::HtmlInputElement;

use crate::prelude::*;

pub struct ChatInput {
    input_elem: RwLock<Option<HtmlInputElement>>
}

impl ChatInput {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            input_elem: RwLock::new(None)
        })
    }

    pub fn render<F>(self: &Arc<Self>, on_submit: F) -> Dom 
    where
        F: Fn(String) + Clone + 'static
    {
        let state = self;
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("width", "100%")
                //.style("height", "100%")
            }
        });
        static INPUT:Lazy<String> = Lazy::new(|| {
            class! {
                .style("flex", "1")
                .style("font-size", "2rem")
                .style("padding", "1rem")
                .style("border", "none")
                .style("outline", "none")
            }
        });
        static BUTTON:Lazy<String> = Lazy::new(|| {
            class! {
                .style("background-color", "#4CAF50")
                .style("border", "none")
                .style("color", "white")
                .style("text-decoration", "none")
                .style("font-size", "2rem")
                .style("height", "100%")
                .style("display", "flex")
                .style("align-items", "center")
                .style("cursor", "pointer")
                .style("padding", "0 1rem")
            }
        });
        html!("div", {
            .class(&*CONTAINER)
            .child(html!("input" => HtmlInputElement, {
                .class(&*INPUT)
                .attribute("placeholder", "Type a message...")
                .attribute("type", "text")
                .attribute("autocomplete", "off")
                .after_inserted(clone!(state => move |elem| {
                    *state.input_elem.write().unwrap_ext() = Some(elem);
                }))
            }))
            .child(html!("div", {
                .class(&*BUTTON)
                .class(&*USER_SELECT_NONE)
                .text("Send")
                .event(clone!(state, on_submit => move |_:events::Click| {
                    if let Some(input_elem) = state.input_elem.read().unwrap_ext().as_ref() {
                        let value = input_elem.value();
                        on_submit(value);
                        input_elem.set_value("");
                    }
                }))
            }))
        })
    }
}