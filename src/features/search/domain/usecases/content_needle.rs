use crate::core::constants::search::CONTENT_PREFIX;

pub fn content_needle(query: &str) -> Option<&str> {
    query
        .trim_start()
        .strip_prefix(CONTENT_PREFIX)
        .map(str::trim)
        .filter(|needle| !needle.is_empty())
}
