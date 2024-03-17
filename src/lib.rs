//! `web` localization definition.
//!
//! Common web stuff such as status codes and browser errors translated.

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "i18n",
        fallback_language: "en-US",
    };
}

