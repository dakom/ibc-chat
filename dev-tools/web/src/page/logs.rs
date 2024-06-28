use std::fmt::Display;

use dominator::EventOptions;
use dominator_helpers::futures::spawn_future;
use js_sys::Date;
use shared_dev_tools::process::{self, ProcessId};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::{atoms::{buttons::{ButtonSize, Squareish1Button}, dynamic_svg::color_circle::ColorCircle, image::render_app_img}, config::CONFIG, prelude::*, route::Route};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct LogId(pub usize);
impl Display for LogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}


pub struct Logs {
    pub items: MutableVec<LogItem>
}

pub static LOGS:Lazy<Logs> = Lazy::new(|| {
    Logs {
        items: MutableVec::new()
    }
});

impl Logs {
    pub fn add_item(&self, options: LogItemOptions) -> LogId {
        let log_id = LogId(self.items.lock_ref().len());

        let process_id = options.process_id.clone();

        self.items.lock_mut().push_cloned(LogItem::new(
            log_id,
            options
        ));

        if let Some(process_id) = process_id {
            self.add_line(log_id, format!("Starting process {}", process_id)).unwrap_ext();
        }

        log_id
    }

    pub fn add_line(&self, item_id: LogId, message:String) -> Result<()> {
        let lock = self.items.lock_ref();
        let item = lock.get(item_id.0).context(format!("could not find item at {}", item_id))?;
        item.lines.lock_mut().push_cloned(LogLine::new(message));
        Ok(())
    }

    pub fn set_color(&self, item_id: usize, color: ColorSemantic) -> Result<()> {
        let lock = self.items.lock_ref();
        let item = lock.get(item_id).context(format!("could not find item at {}", item_id))?;
        let item_color = item.color.as_ref().context(format!("item at {} has no color", item_id))?;
        item_color.set(color);

        Ok(())
    }
}

#[derive(Clone)]
pub struct LogItem {
    pub id: LogId,
    pub title: Arc<String>,
    pub timestamp: f64,
    pub color: Option<Mutable<ColorSemantic>>,
    pub lines: MutableVec<LogLine>,
    pub process_id: Option<ProcessId>
}

pub struct LogItemOptions {
    pub title: String,
    pub color: Option<ColorSemantic>,
    pub process_id: Option<ProcessId>
}

impl LogItem {
    pub fn new(id: LogId, options: LogItemOptions) -> Self {
        Self {
            id,
            title: Arc::new(options.title.to_string()),
            timestamp: Date::now(),
            color: options.color.map(Mutable::new),
            lines: MutableVec::new(),
            process_id: options.process_id
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogLine {
    pub message: Arc<String>,
}

impl LogLine {
    pub fn new(message:String) -> Self {
        Self {
            message: Arc::new(message)
        }
    }
}

pub struct LogsUi {
}

impl LogsUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("padding", "1rem")
            }
        });

        html!("div", {
            .class(&*CLASS)
            .children_signal_vec(LOGS.items.signal_vec_cloned().map(|item| {
                LogItemUi::new(item).render()
            }))

        })
    }
}

pub struct LogItemUi {
    item: LogItem,
    showing_lines: Mutable<bool>
}

impl LogItemUi {
    pub fn new(item:LogItem) -> Arc<Self> {
        Arc::new(Self {
            item,
            showing_lines: Mutable::new(false)
        })
    }

    fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static CONTAINER_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("border", "1px solid black")
                .style("margin", "1rem")
                .style("padding", "1rem")
                .style("border-radius", "0.5rem")
                .style("background-color", "white")
                .style("box-shadow", "0 0 0.5rem 0.25rem rgba(0,0,0,0.1)")
            }
        });
        static LINES_CONTAINER_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("margin-top", "1rem")
                .style("padding", "1rem")
                .style("font-family", "Inconsolata, monospace")
                .style("font-size", "2rem")
                .style("background-color", ColorSemantic::Darkish.to_str())
                .style("color", ColorSemantic::Whiteish.to_str())
            }
        });

        static HEADER_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("justify-content", "space-between")
                .style("align-items", "center")
                .style("cursor", "pointer")
            }
        });

        static TITLE_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("width", "100%")
            }
        });

        static LEFT_SIDE_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("align-items", "center")
                .style("justify-content", "center")
            }
        });
        static RIGHT_SIDE_CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("align-items", "center")
                .style("justify-content", "center")
            }
        });

        html!("div", {
            .class(&*CONTAINER_CLASS)
            .child(html!("div", {
                .class(&*HEADER_CLASS)
                .class(&*USER_SELECT_NONE)
                .child(html!("div", {
                    .class(&*LEFT_SIDE_CLASS)
                    .apply_if(state.item.color.is_some(), |dom| {
                        dom.child(html!("div", {
                            .child(ColorCircle::render(ButtonSize::Lg, state.item.color.clone().unwrap_ext().signal()))
                        }))
                    })
                    .child(html!("div", {
                        .class(&*TEXT_SIZE_MD)
                        .class(&*TITLE_CLASS)
                        .text({
                            let date = Date::new(&JsValue::from_f64(state.item.timestamp));
                            &format!("{} - {}", state.item.title, date.to_iso_string())
                        })
                    }))
                }))
                .child(html!("div", {
                    .class(&*RIGHT_SIDE_CLASS)
                    .apply_if(state.item.process_id.is_some(), |dom| {
                        dom.fragment(&render_app_img("kill_process.png".to_string(), clone!(state => move |dom| {
                            dom
                                // so we can detect whether it's a kill process button
                                // when the header is clicked
                                .attribute("data-kill-process", "true")
                                .style("width", "3rem")
                                .style("height", "3rem")
                                .event(clone!(state => move |evt:events::Click| {
                                    let process_id = state.item.process_id.unwrap_ext();
                                    spawn_local(async move {
                                        tauri::kill_process(process_id).await;
                                    })
                                }))
                        })))
                    })
                    .child(html!("div", {
                        .class(&*TEXT_SIZE_MD)
                        .text_signal(state.showing_lines.signal().map(|showing| {
                            if showing {
                                "▲"
                            } else {
                                "▼"
                            }
                        }))
                    }))
                }))
                .event(clone!(state => move |evt:events::Click| {
                    if let Some(target) = evt.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                        if target.has_attribute("data-kill-process") {
                            return;
                        }
                    }
                    state.showing_lines.set_neq(!state.showing_lines.get());
                }))
            }))
            .child(html!("div", {
                .child_signal(state.showing_lines.signal().map(clone!(state => move |showing| {
                    if showing {
                        Some(html!("div", {
                            .class([&*LINES_CONTAINER_CLASS])
                            .children_signal_vec(state.item.lines.signal_vec_cloned().map(|line| {
                                html!("div", {
                                    .text(&*line.message)
                                })
                            }))
                        }))
                    } else {
                        None
                    }
                })))
            }))
        })
    }
}