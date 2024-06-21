use crate::{atoms::dynamic_svg::more_arrow::MoreArrow, prelude::*, util::mixins::{handle_on_click, set_on_hover}};
use dominator::{animation::{easing, MutableAnimation, Percentage}, DomBuilder};
use dominator_helpers::signals::arc_signal_fn;
use unic_langid::CharacterDirection;
use web_sys::HtmlElement;

pub struct UnderlineButton {
    pub inner: ButtonInner,
}

impl UnderlineButton {
    pub fn new() -> Self {
        Self {
            inner: ButtonInner::new(),
        }
    }
    pub fn render<F, S>(&self, text: String, selected_signal_fn: F, on_click: impl FnMut() + 'static) -> Dom 
        where
            F: Fn() -> S + 'static,
            S: Signal<Item = bool> + 'static
    {
        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
            }
        });
        self.inner.render(|hover| clone!(hover => move |dom| {
            #[derive(Clone, Copy, PartialEq, Debug)]
            enum State {
                Hover,
                Selected,
                Default
            }

            let state_signal_fn = || map_ref! {
                let hover = hover.signal(),
                let selected = selected_signal_fn() => {
                    if *selected {
                        State::Selected
                    } else if *hover {
                        State::Hover
                    } else {
                        State::Default
                    }
                }
            }.dedupe();

            apply_methods!(dom, {
                .class([&*CLASS, &*TEXT_SIZE_LG])
                .style_signal("color", state_signal_fn().map(|state| {
                    match state {
                        State::Hover | State::Selected => ColorSemantic::Darkish.to_str(),
                        State::Default => ColorSemantic::MidGrey.to_str(),
                    }
                }))
                .apply(handle_on_click(on_click))
                .child(html!("div", {
                    .text(&text)
                }))
                .child_signal(state_signal_fn().map(|state| {
                    if state == State::Hover || state == State::Selected {
                        let animation = MutableAnimation::new(600.0);
                        animation.animate_to(Percentage::END);
                        let animation_signal = animation
                            .signal()
                            .map(|t| easing::out(t, easing::cubic))
                            .map(|t| t.range_inclusive(0.0, 1.0));
                        Some(html!("div", {
                            .style("height", "0.125rem")
                            .style("width", "100%")
                            .apply_if(state == State::Selected, |dom| {
                                dom.class(&*COLOR_UNDERLINE_PRIMARY)
                            })
                            .apply_if(state != State::Selected, |dom| {
                                dom.class(&*COLOR_UNDERLINE_SECONDARY)
                            })
                            .style("transform", "scaleX(0)")
                            .style_signal("transform", animation_signal.map(|t| {
                                format!("scaleX({})", t)
                            }))
                        }))
                    } else {
                        None
                    }
                }))
            })
        }))
    }
}

pub struct Squareish1Button {
    pub inner: ButtonInner
}

impl Squareish1Button {
    pub fn new() -> Self {
        Self {
            inner: ButtonInner::new(),
        }
    }
    pub fn render(&self, text: String, on_click: impl FnMut() + 'static) -> Dom {
        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-flex")
                .style("padding", "0.625rem 1.875rem")
                .style("justify-content", "center")
                .style("align-items", "center")
                .style("gap", "0.625rem")
                .style("border-radius", "0.25rem")
            }
        });
        self.inner.render(|hover| clone!(hover => move |dom| {
            apply_methods!(dom, {
                .class([&*CLASS, &*TEXT_SIZE_LG, &*COLOR_BUTTON_PRIMARY_TEXT])
                .class_signal(&*COLOR_BUTTON_PRIMARY_BG, hover.signal().map(|x| !x))
                .class_signal(&*COLOR_BUTTON_PRIMARY_BG_HOVER, hover.signal())
                .apply(handle_on_click(on_click))
                .children([
                    html!("div", {
                        .text(&text)
                    }),
                ])
            })
        }))
    }
}

pub struct OutlineButton {
    pub accent: bool,
    pub inner: ButtonInner,
}

impl OutlineButton {
    pub fn new(accent: bool) -> Self {
        Self {
            accent,
            inner: ButtonInner::new(),
        }
    }

    pub fn hovering(&self) -> &Mutable<bool> {
        &self.inner.hovering
    }

    pub fn render(&self, image: Option<Dom>, text: String, on_click: impl FnMut() + 'static) -> Dom {
        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("align-items", "center")
                .style("gap", "0.625rem")
                .style("padding", "0.625rem 1.875rem")
                .style("border-radius", "0.25rem")
                .style("border-width", "1px")
                .style("border-style", "solid")
            }
        });

        let accent = self.accent;
        let color = move |hover: bool| -> &'static str {
            if hover {
                if accent {
                    ColorSemantic::AccentAlt.to_str()
                } else {
                    ColorSemantic::Darkish.to_str()
                }
            } else {
                if accent {
                    ColorSemantic::Accent.to_str()
                } else {
                    ColorSemantic::MidGrey.to_str()
                }
            }
        };


        self.inner.render(|hover| clone!(hover => move |dom| {
            apply_methods!(dom, {
                .class(&*CLASS)
                .style_signal("border-color", hover.signal().map(move |hover| color(hover)))
                .style_signal("color", hover.signal().map(move |hover| color(hover)))
                .apply(handle_on_click(on_click))
                .apply_if(image.is_none(), |dom| {
                    dom
                        .style("justify-content", "center")
                        .style("align-items", "center")
                })
                .apply_if(image.is_some(), |dom| {
                    dom.child(image.unwrap())
                })
                .child(html!("div", {
                    .class(&*TEXT_SIZE_LG)
                    .text(&text)
                }))
            })
        }))
    }
}

struct ButtonInner {
    pub hovering: Mutable<bool>,
    pub prevent_hover: bool,
}

impl ButtonInner
{
    fn new() -> Self {
        Self {
            hovering: Mutable::new(false),
            prevent_hover: false,
        }
    }

    fn render<F, F_INNER>(&self, mixin: F) -> Dom
    where
        F: FnOnce(Mutable<bool>) -> F_INNER,
        F_INNER: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static
    {

        html!("div", {
            .apply(set_on_hover(&self.hovering))
            .class_signal(&*CURSOR_POINTER, self.hovering.signal())
            .class(&*USER_SELECT_NONE)
            .apply(mixin(self.hovering.clone()))
        })
    }
}