use crate::core::constants::files::{BADGE_FALLBACK, BADGE_MAX_CHARS};

pub fn extension_badge(extension: Option<&str>) -> String {
    match extension.filter(|extension| !extension.is_empty()) {
        Some(extension) => extension
            .chars()
            .take(BADGE_MAX_CHARS)
            .collect::<String>()
            .to_uppercase(),
        None => BADGE_FALLBACK.to_string(),
    }
}
