use dominator::stylesheet;
use crate::{
    prelude::*, 
    theme::responsive::MediaQueryWidth,
};

pub fn init() {
    stylesheet!(":root", {
        .style("box-sizing", "border-box")
        .style_signal("font-size", MediaQueryWidth::signal().map(|query_width| {
            match query_width {
                MediaQueryWidth::SmallPhone => "6px",
                MediaQueryWidth::Phone => "10px",
                MediaQueryWidth::Tablet => "12px",
                MediaQueryWidth::SmallDesktop => "14px",
                MediaQueryWidth::Desktop => "16px",
            }
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

