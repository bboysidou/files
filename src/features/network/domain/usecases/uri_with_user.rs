use crate::core::constants::network::{SCHEME_SEPARATOR, SCP_USER_SEPARATOR};

pub fn uri_with_user(uri: &str, user: &str) -> String {
    let Some((scheme, rest)) = uri.split_once(SCHEME_SEPARATOR) else {
        return uri.to_owned();
    };
    let authority = rest.split('/').next().unwrap_or_default();
    if user.is_empty() || authority.contains(SCP_USER_SEPARATOR) {
        return uri.to_owned();
    }
    format!("{scheme}{SCHEME_SEPARATOR}{user}{SCP_USER_SEPARATOR}{rest}")
}
