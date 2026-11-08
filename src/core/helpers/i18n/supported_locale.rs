use crate::core::constants::i18n::{DEFAULT_LOCALE, LOCALE_REGION_SEPARATOR};

pub fn supported_locale(tag: &str, available: &[&str]) -> String {
    if available.contains(&tag) {
        return tag.to_owned();
    }
    let language = tag.split(LOCALE_REGION_SEPARATOR).next().unwrap_or(tag);
    if available.contains(&language) {
        return language.to_owned();
    }
    DEFAULT_LOCALE.to_owned()
}
