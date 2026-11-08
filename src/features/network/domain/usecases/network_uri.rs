use crate::core::constants::network::{
    NETWORK_SCHEMES, SCHEME_SEPARATOR, SCP_HOST_SEPARATOR, SCP_USER_SEPARATOR, SFTP_PREFIX,
};

pub fn network_uri(typed: &str) -> Option<String> {
    let typed = typed.trim();
    if let Some((scheme, _)) = typed.split_once(SCHEME_SEPARATOR) {
        return NETWORK_SCHEMES
            .iter()
            .any(|known| known.eq_ignore_ascii_case(scheme))
            .then(|| typed.to_owned());
    }
    let (host, path) = typed.split_once(SCP_HOST_SEPARATOR)?;
    let plausible = !host.is_empty()
        && !host.contains('/')
        && (host.contains(SCP_USER_SEPARATOR) || path.starts_with('/'));
    plausible.then(|| {
        let path = path.trim_start_matches('/');
        format!("{SFTP_PREFIX}{host}/{path}")
    })
}
