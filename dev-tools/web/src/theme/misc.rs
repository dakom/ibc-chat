use dominator::class;
use once_cell::sync::Lazy;

pub static USER_SELECT_NONE:Lazy<String> = Lazy::new(|| {
    class! {
        .style(["user-select", "-moz-user-select", "-webkit-user-select"], "none")
    }
});

pub static CURSOR_POINTER:Lazy<String> = Lazy::new(|| {
    class! {
        .style("cursor", "pointer")
    }
});

pub static WORD_WRAP_PRE:Lazy<String> = Lazy::new(|| {
    class! {
        .style("white-space", "pre-wrap")
    }
});