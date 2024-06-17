/*
Just call LOCALE.set_lang("en") (where "en" is the language code)

then call LOCALE.get_text("some-id", args) or LOCALE.get_dir() to get the language string or direction

but it's simpler to use the macros

Text is retrieved like:

html!("div", {
    .text(get_text!("landing-header"))
})

or with fluent args:

html!("div", {
    .text(get_text!("greeting", {
        "name" => "bob"
    }))
}) 

*/

use std::{borrow::Cow, collections::HashMap, sync::{Arc, RwLock}};

use awsm_web::prelude::UnwrapExt;
use dominator::{events, html, Dom};
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;
use unic_langid::{langid, LanguageIdentifier};
use intl_memoizer::concurrent::IntlLangMemoizer;
use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;

use crate::{TextDirection, config::CONFIG, TEXT_SIZE_LG};

pub struct Locale {
    pub current: Mutable<Arc<LocaleInfo>>,
    lookup: HashMap<LanguageIdentifier, Arc<LocaleInfo>>,
    fallback: Arc<LocaleInfo>,
}

impl Locale {
    pub fn get_text<'a>(&'a self, id: &'static str, args: Option<FluentArgs<'a>>) -> String {
        let curr = self.current.lock_ref();
        match curr.try_get_string(id, args.as_ref()) {
            Ok(value) => value,
            Err(err) => {
                if curr.lang_id == self.fallback.lang_id {
                    panic!("Failed to get message for fluent id: {}: {id} - {err}", curr.lang_id.to_string());
                } else {
                    match self.fallback.try_get_string(id, args.as_ref()) {
                        Ok(value) => value,
                        Err(err) => panic!("Failed to get message for fluent id using fallback: {}: {id} - {err}", self.fallback.lang_id.to_string())
                    }
                }
            }
        }
    }

    pub fn get_dir(&self) -> TextDirection {
        self.current.lock_ref().dir()
    }

    pub fn set_lang(&self, lang_str: &str) {
        let mut lock = self.current.lock_mut();
        match lang_str.parse::<LanguageIdentifier>() {
            Ok(lang_id) => {
                // fallback is actually an *optimized* case
                // it's excluded from the lookup, just set it directly 
                if lang_id.matches(&self.fallback.lang_id, true, true) {
                    *lock = self.fallback.clone(); 
                    return;
                } else {
                    for id in self.lookup.keys() {
                        if lang_id.matches(id, true, true) {
                            let info = self.lookup.get(id).unwrap_ext();
                            *lock = info.clone();
                            return;
                        }
                    }
                }
            },
            Err(_) => {}
        }

        log::warn!("Failed to set current language: {lang_str}");
    }
}

pub struct LocaleInfo {
    pub lang_id: LanguageIdentifier,
    pub bundle: FluentBundle<FluentResource, IntlLangMemoizer>
}

impl LocaleInfo {
    pub fn try_get<'a, 'b>(&'a self, id: &str, args: Option<&'b FluentArgs>) -> Result<Cow<'b, str>> 
    where 'a: 'b
    {
        let msg = self.bundle.get_message(id).with_context(|| format!("failed to get message for fluent id: {id}"))?;
        let pattern = msg.value().with_context(|| format!("failed to get pattern for fluent id: {id}"))?;
        let mut errors = Vec::new();
        let value = self.bundle.format_pattern(&pattern, args, &mut errors);

        if errors.is_empty() {
            Ok(value)
        } else {
            Err(anyhow!("Failed to format pattern: {:?}", errors))
        }
    }

    pub fn get<'a, 'b>(&'a self, id: &str, args: Option<&'b FluentArgs>) -> Cow<'b, str>
    where 'a: 'b
    {
        self.try_get(id, args).unwrap_ext()
    }

    pub fn get_string(&self, id: &str, args: Option<&FluentArgs>) -> String 
    {
        self.get(id, args).to_string()
    }

    pub fn try_get_string(&self, id: &str, args: Option<&FluentArgs>) -> Result<String>
    {
        self.try_get(id, args).map(|x| x.to_string())
    }


    pub fn dir(&self) -> TextDirection {
        match self.lang_id.language.as_str() {
            "he" => TextDirection::Rtl,
            _ => TextDirection::Ltr 
        }
    }

    pub fn new(lang_str: &'static str) -> Self {
        let (lang_id, ftl_texts) = match lang_str {
            "en" => (langid!("en"), [
                include_str!("locale/landing/en.ftl"),
                include_str!("locale/error/en.ftl"),
                include_str!("locale/common/en.ftl"),
            ]),
            "he" => (langid!("he"), [
                include_str!("locale/landing/he.ftl"),
                include_str!("locale/error/he.ftl"),
                include_str!("locale/common/he.ftl"),
            ]),
            _ => panic!("Unsupported locale: {lang_str}")
        };

        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);

        for ftl_text in &ftl_texts {
            let resource = FluentResource::try_new(ftl_text.to_string()).expect(&format!("Failed to parse an FTL string for {lang_str}"));
            bundle.add_resource(resource).expect("Failed to add FTL resources to the bundle.");
        }

        Self {
            lang_id,
            bundle
        }
    }
}

pub static LOCALE:Lazy<Locale> = Lazy::new(|| {
    let mut lookup = HashMap::new();

    // Add more locales here
    let english = Arc::new(LocaleInfo::new("en"));
    let hebrew = Arc::new(LocaleInfo::new("he"));
    lookup.insert(hebrew.lang_id.clone(), hebrew);

    let ret = Locale {
        current: Mutable::new(english.clone()),
        lookup,
        fallback: english,
    };

    match CONFIG.default_lang {
        Some(default_lang) => {
            ret.set_lang(default_lang);
        },
        None => {
            if let Some(browser_lang) = web_sys::window()
                .and_then(|window| window.navigator().language()) {
                    ret.set_lang(&browser_lang);
                }
        }
    }

    ret
});


#[macro_export]
macro_rules! text_args {
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut args = ::fluent::FluentArgs::new();
        $(args.set($key, $value);)*

        args
    }};
}

#[macro_export]
macro_rules! get_text {
    ($id:expr) => {
        crate::locale::LOCALE.get_text($id, None)
    };

    ($id:expr, {$($key:expr => $value:expr),+ $(,)?}) => {{
        let mut args = ::fluent::FluentArgs::new();
        $(args.set($key, $value);)*

        LOCALE.get_text($id, Some(args))
    }};

    ($id:expr, $args:expr) => {{
        LOCALE.get_text($id, $args)
    }};
}


pub struct LanguageSelector {
}

impl LanguageSelector {
    pub fn render() -> Dom {

        html!("div", {
            .style("display", "flex")
            .style("justify-content", "center")
            .child(html!("select", {
                .class(&*TEXT_SIZE_LG)
                .event(|evt: events::Change| {
                    if let Some(value) = evt
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
                        .map(|select| select.value())
                    {
                        LOCALE.set_lang(&value);
                    }
                })
                .children([
                    html!("option", {
                        .text("English")
                        .property("value", "en")
                        .prop_signal("selected", LOCALE.current.signal_cloned().map(|lang| {
                            lang.lang_id.matches(&langid!("en"), true, true)
                        }))
                    }),
                    html!("option", {
                        .text("Hebrew / עִברִית")
                        .property("value", "he")
                        .prop_signal("selected", LOCALE.current.signal_cloned().map(|lang| {
                            lang.lang_id.matches(&langid!("he"), true, true)
                        }))
                    }),
                ])
            }))
        })
    }
}