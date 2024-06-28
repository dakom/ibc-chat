use crate::{prelude::*, util::mixins::set_on_hover};

pub struct Dropdown<T> {
    pub label: String,
    pub options: Vec<Arc<DropdownOption<T>>>,
    pub selected: Mutable<Option<Arc<DropdownOption<T>>>>,
    pub showing: Mutable<bool>
}

pub struct DropdownOption<T> {
    pub label: String,
    pub value: T,
}

impl <T> Dropdown<T> 
where T: PartialEq + 'static
{
    pub fn new(label: String, initial_selected: Option<T>, options: impl IntoIterator<Item = (String, T)>) -> Arc<Self> {

        let options:Vec<Arc<DropdownOption<T>>> = options.into_iter().map(|(label, value)| {
            Arc::new(DropdownOption {
                label,
                value,
            })
        }).collect();

        let initial_selected = initial_selected.map(|initial_selected| {
            options.iter().find(|option| option.value == initial_selected).cloned()
        }).flatten();

        Arc::new(Self {
            label,
            options,
            selected: Mutable::new(initial_selected),
            showing: Mutable::new(false)
        })
    }

    pub fn render(self: &Arc<Self>, on_change: impl Fn(&T) + 'static) -> Dom {
        let state = self;

        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-flex")
                .style("flex-direction", "column")
                .style("gap", "1rem")
            }
        });

        html!("div", {
            .class(&*CONTAINER)
            .class(&*USER_SELECT_NONE)
            .child(html!("div", {
                .class(&*TEXT_SIZE_MD)
                .text(&state.label)
            }))
            .child(state.render_inner(on_change))
        })
    }

    fn render_inner(self: &Arc<Self>, on_change: impl Fn(&T) + 'static) -> Dom {
        let state = self;

        static CONTAINER_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-block")
                .style("position", "relative")
                .style("border", "1px solid black")
                .style("border-radius", "4px")
                .style("padding", "0.5rem")
                .style("cursor", "pointer")
            }
        });

        static LABEL_CONTAINER_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("justify-content", "space-between")
                .style("padding", "1rem")
            }
        });

        static OPTIONS_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("position", "absolute")
                .style("padding", "1rem")
                .style("top", "100%")
                .style("left", "0")
                .style("width", "100%")
                .style("border", "1px solid black")
                .style("border-radius", "4px")
                .style("background-color", "white")
                .style("z-index", "100")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("gap", "1rem")
            }
        });

        let selected_label = state.selected.signal_cloned().map(|selected| {
            selected.map(|selected| selected.label.clone()).unwrap_or_else(|| "Select...".to_string())
        });

        let on_change = Arc::new(on_change);

        html!("div", {
            .class(&*CONTAINER_CLASS)
            .child(html!("div", {
                .class(&*LABEL_CONTAINER_CLASS)
                .child(html!("div", {
                    .class(&*TEXT_SIZE_MD)
                    .text_signal(selected_label)
                }))
                .child(html!("div", {
                    .class(&*TEXT_SIZE_MD)
                    .text_signal(state.showing.signal().map(|showing| {
                        if showing {
                            "▲"
                        } else {
                            "▼"
                        }
                    }))
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.showing.set(!state.showing.get());
                }))
            }))
            .child_signal(state.showing.signal().map(clone!(state, on_change => move |showing| {
                if showing {
                    Some(html!("div", {
                        .class(&*OPTIONS_CLASS)
                        .children(state.options.iter().map(clone!(state, on_change => move |option| {
                            let hovering = Mutable::new(false);
                            html!("div", {
                                .class(&*TEXT_SIZE_MD)
                                .text(&option.label)
                                .style_signal("color", hovering.signal().map(|hovering| {
                                    if hovering {
                                        ColorSemantic::Accent.to_str()
                                    } else {
                                        ColorSemantic::Darkish.to_str()
                                    }
                                }))
                                .event({
                                    clone!(state, option, on_change => move |_: events::Click| {
                                        state.selected.set(Some(option.clone()));
                                        state.showing.set_neq(false);
                                        on_change(&option.value);
                                    })
                                })
                                .apply(set_on_hover(&hovering))
                            })
                        })))
                    }))
                } else {
                    None
                }
            })))
            .with_node!(el => {
                .global_event(clone!(state => move |evt: events::Click| {
                    if let Some(target) = evt.target() {
                        if !el.contains(Some(target.unchecked_ref())) {
                            state.showing.set_neq(false);
                        }
                    }
                }))
            })
        })
    }
}

