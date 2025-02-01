//! `web` localization definition.
//!
//! Common web stuff such as status codes and browser errors translated.

use fluent_templates::Loader;

fluent_templates::static_loader! {
    pub static LOCALE = {
        locales: "i18n",
        fallback_language: "en-US",
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
}

impl i18n::LocalizedDisplay for Error {
    fn localize(&self, lang: &i18n::LanguageIdentifier) -> String {
        match self {
            Self::NotFound => LOCALE.lookup(lang, "web-code-not_found"),
        }
    }
}
