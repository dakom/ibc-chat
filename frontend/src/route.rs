use crate::{config::CONFIG, page::{chat::ChatPage, landing::Landing, notfound::NotFound}, prelude::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Landing,
    Chat,
    NotFound
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

        let route = match paths {
            [""] => Self::Landing,
            ["/"] => Self::Landing,
            ["chat"] => Self::Chat,
            _ => Self::NotFound,
        };

        match route {
            Self::Landing | Self::NotFound => route,
            _ => {
                if Wallet::get_connected() {
                    route
                } else {
                    Route::Landing.go_to_url();
                    Route::Landing
                }
            }
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
            Route::Landing => "".to_string(),
            Route::Chat => "chat".to_string(), 
            Route::NotFound => "404".to_string(), 
        };
        write!(f, "{}", s)
    }
}


pub fn render() -> Dom {
    html!("div", {
        .style("width", "100%")
        .style("height", "100%")
        .child_signal(Route::signal().map(|route| {
            Some(match route {
                Route::Landing => Landing::new().render(),
                Route::Chat => ChatPage::new().render(),
                Route::NotFound => NotFound::new().render(),
            })
        }))
    })
}