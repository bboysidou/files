use crate::core::constants::preview::ARCHIVE_INNER_EXTENSION;

pub fn archive_stem(name: &str) -> &str {
    let stem = strip_extension(name);
    match stem.rsplit_once('.') {
        Some((inner, extension))
            if !inner.is_empty() && extension.eq_ignore_ascii_case(ARCHIVE_INNER_EXTENSION) =>
        {
            inner
        }
        _ => stem,
    }
}

fn strip_extension(name: &str) -> &str {
    match name.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem,
        _ => name,
    }
}
