use dominator::class;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorSemantic {
    Darkest,
    Accent,
    Whiteish,
    Darkish,
    MidGrey,
    AccentAlt,
    Focus,
    Error,
    Warning,
    Success,
    AccentVeryLight,
    GreyAlt1,
    GreyAlt2,
    PureWhite,
}

impl ColorSemantic {
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Darkest => "#11131A",
            Self::AccentAlt => "#0084B2",
            Self::AccentVeryLight => "#C7F8FF",
            Self::Accent => "#28B9EA",
            Self::Whiteish => "#FAFAFA",
            Self::Darkish => "#45474F",
            Self::MidGrey => "#92949F",
            Self::Focus => "#73A2FF",
            Self::Error => "#E00C0C",
            Self::Warning => "#ED933F",
            Self::Success => "#3AD365",
            Self::GreyAlt1 => "#D9D9D9",
            Self::GreyAlt2 => "#EFEFEF",
            Self::PureWhite => "#FFFFFF",
        }
    }
}

pub static COLOR_BUTTON_PRIMARY_BG: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::Accent.to_str())
});

pub static COLOR_BUTTON_PRIMARY_TEXT: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::Whiteish.to_str())
});

pub static COLOR_HEADER: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::Darkest.to_str())
});

pub static COLOR_BYLINE: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::MidGrey.to_str())
});

pub static COLOR_PARAGRAPH: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::Darkish.to_str())
});

pub static COLOR_UNDERLINE_PRIMARY: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::Accent.to_str())
});

pub static COLOR_BORDER_UNDERLINE_PRIMARY: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::Accent.to_str())
});

pub static COLOR_UNDERLINE_SECONDARY: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::MidGrey.to_str())
});

pub static COLOR_BORDER_UNDERLINE_SECONDARY: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::MidGrey.to_str())
});

pub static COLOR_BUTTON_PRIMARY_BG_HOVER: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::AccentAlt.to_str())
});

pub static COLOR_INPUT_LABEL_DEFAULT: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::Darkest.to_str())
});

pub static COLOR_INPUT_BORDER_DEFAULT: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::MidGrey.to_str())
});

pub static COLOR_INPUT_TEXT_DEFAULT: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::Darkest.to_str())
});

pub static COLOR_INPUT_BORDER_WARNING: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::Warning.to_str())
});

pub static COLOR_INPUT_BORDER_ERROR: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::Error.to_str())
});

pub static COLOR_INPUT_BORDER_FOCUS: Lazy<String> = Lazy::new(|| class! {
    .style("border-color", ColorSemantic::Focus.to_str())
});

pub static COLOR_BUTTON_BG_DISABLED: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::AccentVeryLight.to_str())
});

pub static COLOR_SIDEBAR_SELECTED_BG: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::GreyAlt1.to_str())
});

pub static COLOR_SIDEBAR_BG: Lazy<String> = Lazy::new(|| class! {
    .style("background-color", ColorSemantic::GreyAlt2.to_str())
});

pub static COLOR_INPUT_TEXT_PLACEHOLDER: Lazy<String> = Lazy::new(|| class! {
    .style("color", ColorSemantic::MidGrey.to_str())
});