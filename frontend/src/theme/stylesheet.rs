use dominator::stylesheet;
use crate::{prelude::*, theme::responsive::WindowSize};

pub fn init() {
    stylesheet!(":root", {
        .style("box-sizing", "border-box")
        .style_signal("font-size", WindowSize::width_signal().map(|width| { 
            let scale_ratio = width / 1920.0;
            format!("{}px", scale_ratio * 16.0)
        }))
    });

    stylesheet!("*, ::before, ::after", {
        .style("box-sizing", "inherit")
    });

    stylesheet!("html, body", {
        .style("margin", "0")
        .style("padding", "0")
        .style("width", "100%")
        .style("height", "100%")
    });

    stylesheet!("a", {
        .style("all", "unset")
        .style("cursor", "pointer")
    })
}

