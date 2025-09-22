//! `web` localization definition.
//!
//! Common web stuff such as status codes and browser errors translated.

i18n::load!("./i18n", fallback_lang = "en-US");

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
}

impl i18n::LocalizedDisplay for Error {
    fn localize(&self, lang: &i18n::LanguageIdentifier) -> i18n::Message {
        let id = match self {
            Self::NotFound => "http-code-not_found",
        };

        LOCALES
            .query(lang, &i18n::Query::new(id).with_fallback(true))
            .unwrap()
    }
}
