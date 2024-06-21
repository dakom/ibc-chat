use crate::{prelude::*, route::Route, util::mixins::set_on_hover};
pub struct Sidebar {
    items: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn new<T, I>(items: T) -> Arc<Self> 
    where 
        T: IntoIterator<Item = I>,
        I: Into<SidebarItem>,
    {
        Arc::new(Self {
            items: items.into_iter().map(Into::into).collect(),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("height", "100vh")
                .style("background-color", ColorSemantic::Darkest.to_str())
            }
        });
        html!("div", {
            .class(&*CONTAINER)
            .children(self.items.iter().map(|item| {
                item.render()
            }))
        })
    }
}

pub struct SidebarItem {
    text: String,
    route: Option<Route>,
    hovering: Mutable<bool>,
}

impl SidebarItem {
    fn render(self: &Self) -> Dom {
        static CONTAINER:Lazy<String> = Lazy::new(|| {
            class! {
                .style("cursor", "pointer")
                .style("padding", "1rem 2rem")
                .style("user-select", "none")
                .style("width", "100%")
            }
        });

        let my_route = self.route.clone();
        let selected_signal = || Route::signal().map(clone!(my_route => move |current_route| {
            Some(current_route) == my_route
        }));

        let hovering_selected_signal = map_ref! {
            let hovering = self.hovering.signal(),
            let selected = selected_signal()
            => (*hovering, *selected)
        };

        html!("div", {
            .class(&*CONTAINER)
            .class(&*TEXT_SIZE_LG)
            .style_signal("color", hovering_selected_signal.map(|(hovering, selected)| {
                match (hovering, selected) {
                    (true, true) => ColorSemantic::Darkest.to_str(),
                    (true, false) => ColorSemantic::Accent.to_str(),
                    (false, true) => ColorSemantic::Whiteish.to_str(),
                    (false, false) => ColorSemantic::MidGrey.to_str(),
                }
            }))
            .style_signal("background-color", selected_signal().map(|selected| {
                if selected {
                    ColorSemantic::Accent.to_str()
                } else {
                    ColorSemantic::Darkest.to_str()
                }
            }))
            //.style("background-color", ColorSemantic::Warning.to_str())
            .apply_if(my_route.is_some(), set_on_hover(&self.hovering))
            .text(&self.text)
            .event(clone!(my_route => move |_: events::Click| {
                if let Some(my_route) = my_route.as_ref() {
                    my_route.go_to_url();
                }
            }))
        })
    }

}

impl From<(&str, Option<Route>)> for SidebarItem {
    fn from((text, route): (&str, Option<Route>)) -> Self {
        Self {
            text: text.to_string(),
            route,
            hovering: Mutable::new(false),
        }
    }
}