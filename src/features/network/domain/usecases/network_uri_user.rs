use crate::core::constants::network::{SCHEME_SEPARATOR, SCP_USER_SEPARATOR};

pub fn network_uri_user(uri: &str) -> Option<String> {
    let (_, rest) = uri.split_once(SCHEME_SEPARATOR)?;
    let authority = rest.split('/').next()?;
    let (user, _) = authority.split_once(SCP_USER_SEPARATOR)?;
    let user = user.split(':').next().unwrap_or(user);
    (!user.is_empty()).then(|| user.to_owned())
}
