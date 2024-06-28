use crate::{atoms::buttons::{ButtonSize, Squareish1Button, UnderlineButton}, config::CONFIG, page::{dashboard::DashboardUi, logs::LogsUi, settings::SettingsUi}, prelude::*, route::Route};


pub struct Landing {
}

impl Landing {
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
            .child(state.render_header())
            .child_signal(Route::signal().map(clone!(state => move |route| {
                match route {
                    Route::Dashboard => {
                        Some(DashboardUi::new().render())
                    },
                    Route::Logs => {
                        Some(LogsUi::new().render())
                    },
                    Route::Settings => {
                        Some(SettingsUi::new().render())
                    },
                    _ => {
                        None
                    }
                }
            })))
        })
    }

    fn render_header(self: &Arc<Self>) -> Dom {
        let state = self;

        static CLASS:Lazy<String> = Lazy::new(|| {
            class! {
                .style("width", "100%")
                .style("justify-content", "center")
                .style("padding", "1rem")
                .style("display", "flex")
                .style("gap", "3rem")
            }
        });

        html!("div", {
            .class(&*CLASS)
            .child(state.render_header_button(Route::Dashboard))
            .child(state.render_header_button(Route::Logs))
            .child(state.render_header_button(Route::Settings))
        })
    }

    fn render_header_button(self: &Arc<Self>, route: Route) -> Dom {
        let is_selected = clone!(route => move || Route::signal().map(clone!(route => move |current_route| {
            current_route == route
        })));

        UnderlineButton::new()
            .set_size(ButtonSize::Md)
            .render(
                route.to_string(), 
                is_selected,
                clone!(route => move || route.go_to_url())
            )
    }
}