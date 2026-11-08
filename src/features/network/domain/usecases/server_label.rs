use crate::core::constants::network::{SCHEME_SEPARATOR, SMB_SCHEME};

pub fn server_label(uri: &str) -> Option<String> {
    let (scheme, rest) = uri.split_once(SCHEME_SEPARATOR)?;
    let mut parts = rest.split('/').filter(|part| !part.is_empty());
    let authority = parts.next()?;
    let share = parts
        .next()
        .filter(|_| scheme.eq_ignore_ascii_case(SMB_SCHEME));
    Some(match share {
        Some(share) => format!("{authority}/{share}"),
        None => authority.to_owned(),
    })
}
