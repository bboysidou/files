use std::env;

use crate::core::constants::i18n::{DEFAULT_LOCALE, LOCALE_ENV_VARS, NEUTRAL_LOCALES};

pub fn system_locale() -> String {
    LOCALE_ENV_VARS
        .iter()
        .filter_map(|name| env::var(name).ok())
        .find(|value| !value.is_empty() && !NEUTRAL_LOCALES.contains(&value.as_str()))
        .and_then(|value| {
            value
                .split(['.', '@'])
                .next()
                .map(|tag| tag.replace('_', "-"))
        })
        .unwrap_or_else(|| DEFAULT_LOCALE.to_string())
}
