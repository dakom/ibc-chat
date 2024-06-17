use dominator::{class, styles};
use futures_signals::signal::SignalExt;
use once_cell::sync::Lazy;

use crate::theme::responsive::MediaQueryWidth;

const FONT_FAMILY:&str = r#""Noto Sans", sans-serif"#;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextDirection {
    Ltr,
    Rtl
}

impl TextDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }

    pub fn into_class(self) -> &'static str {
        static RTL:Lazy<String> = Lazy::new(|| {
            class! {
                .style("dir", "rtl")
            }
        });
        static LTR:Lazy<String> = Lazy::new(|| {
            class! {
                .style("dir", "ltr")
            }
        });

        match self {
            Self::Ltr => &*LTR,
            Self::Rtl => &*RTL,
        }
    }
}

pub static TEXT_SIZE_H1:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "5.125rem")
    }
});

pub static TEXT_SIZE_H2:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "3.1875rem")
        .style("line-height", "3.825rem")
    }
});

pub static TEXT_SIZE_H3:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "1.9375rem")
    }
});

pub static TEXT_SIZE_XLG:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "1.5rem")
    }
});

pub static TEXT_SIZE_LG:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "1.1875rem")
    }
});

pub static TEXT_SIZE_MD:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "0.875rem")
    }
});

pub static TEXT_SIZE_SM:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-family", FONT_FAMILY)
        .style("font-size", "0.75rem")
    }
});

pub static TEXT_WEIGHT_BOLD:Lazy<String> = Lazy::new(|| {
    class! {
        .style("font-weight", "700")
    }
});
