use crate::prelude::*;

#[derive(Clone)]
pub struct TextInput {
    pub kind: TextInputKind,
    pub value: Mutable<Option<String>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TextInputKind {
    Email,
    Password,
    Text,
    Number,
}

impl TextInput {
    pub fn new(kind: TextInputKind) -> Self {
        Self {
            kind,
            value: Mutable::new(None)
        }
    }

    pub fn render(&self, placeholder: Option<&str>) -> Dom {
        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("padding", "0.625rem 1.875rem")
                .style("border-radius", "0.25rem")
                .style("border-width", "1px")
                .style("border-style", "solid")
                .style("font-size", "1.2rem")
            }
        });
        let value = self.value.clone();
        let show_password = Mutable::new(false);
        let kind = self.kind;

        html!("div", {
            .child(html!("input" => web_sys::HtmlInputElement, {
                .class(&*CLASS)
                .attr_signal("type", show_password.signal().map(move |show_password| {
                    match kind {
                        TextInputKind::Email => "email",
                        TextInputKind::Password => if show_password { "text" } else {"password"},
                        TextInputKind::Text => "text",
                        TextInputKind::Number => "number",
                    }
                }))
                .apply_if(placeholder.is_some(), |dom| {
                    dom.attr("placeholder", placeholder.unwrap())
                })
                .property_signal("value", self.value.signal_cloned().map(|x| x.unwrap_or_default()))
                .with_node!(elem => {
                    .event(clone!(value => move |e:events::Input| {
                        let text = elem.value();
                        let text = if text.is_empty() {
                            None
                        } else {
                            Some(text)
                        };
                        value.set_neq(text);
                    }))
                })
            }))

            .apply_if(self.kind == TextInputKind::Password, |dom| {
                dom.child(html!("div", {
                    .style("margin-top", "0.625rem")
                    .style("cursor", "pointer")
                    .style("user-select", "none")
                    .class(&*TEXT_SIZE_MD)
                    .text_signal(show_password.signal().map(|show_password| {
                        if show_password {
                            get_text!("landing-signin-hide-password")
                        } else {
                            get_text!("landing-signin-show-password")
                        }
                    }))
                    .event(clone!(show_password => move |_:events::Click| {
                        show_password.replace_with(|x| !*x);
                    }))
                }))
            })
        })
    }
}