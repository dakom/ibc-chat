use crate::{config::CONFIG, page::landing::Landing, prelude::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Dashboard,
    Logs,
    Settings
}

impl Route {
    pub fn from_url(url: &str, root_path: &str) -> Self {

        let url = web_sys::Url::new(url).unwrap();
        let paths = url.pathname();
        let paths = paths
            .split('/')
            .into_iter()
            .skip(if CONFIG.root_path.is_empty() { 1 } else { 2 })
            // skip all the roots (1 for the domain, 1 for each part of root path)
            //.skip(root_path.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>();
        let paths = paths.as_slice();

        match paths {
            [""] => Self::Dashboard,
            ["/"] => Self::Dashboard,
            ["dashboard"] => Self::Dashboard,
            ["logs"] => Self::Logs,
            ["settings"] => Self::Settings,
            _ => {
                panic!("Invalid route: {}", url.to_string());
            },
        }
    }

    pub fn link(&self) -> String {
        let domain = "";

        if CONFIG.root_path.is_empty() {
            format!("{}/{}", domain, self.to_string())
        } else {
            format!("{}/{}/{}", domain, CONFIG.root_path, self.to_string())
        }
    }
    pub fn go_to_url(&self) {
        dominator::routing::go_to_url(&self.link());
    }

    pub fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_cloned()
            .map(|url| Route::from_url(&url, CONFIG.root_path))
    }
}
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Route::Dashboard => "dashboard".to_string(),
            Route::Logs => "logs".to_string(),
            Route::Settings => "settings".to_string(),
        };
        write!(f, "{}", s)
    }
}


pub fn render() -> Dom {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum TopLevelRoute {
        Landing
    }

    // Get a deduped signal of top-level route, to avoid full-page re-renders
    let sig = Route::signal().map(|route| {
        match route {
            Route::Dashboard
            | Route::Logs
            | Route::Settings => TopLevelRoute::Landing, 
        }
    }).dedupe();

    html!("div", {
        .style("width", "100%")
        .style("height", "100%")
        .child_signal(sig.map(|route| {
            Some(match route {
                TopLevelRoute::Landing => Landing::new().render(),
            })
        }))
    })
}