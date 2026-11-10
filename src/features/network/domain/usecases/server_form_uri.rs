use crate::core::constants::network::{
    SCHEME_SEPARATOR, SCP_HOST_SEPARATOR, SCP_USER_SEPARATOR, URI_PATH_SEPARATOR,
};
use crate::features::network::domain::entities::server_form::ServerForm;
use crate::features::network::domain::entities::server_form_error::ServerFormError;

pub fn server_form_uri(form: &ServerForm) -> Result<String, ServerFormError> {
    let host = form.host.trim();
    if host.is_empty() || host.contains(|found: char| found.is_whitespace()) {
        return Err(ServerFormError::MissingHost);
    }
    let port = form.port.trim();
    if !port.is_empty() && port.parse::<u16>().map_or(true, |number| number == 0) {
        return Err(ServerFormError::BadPort);
    }
    let user = form.user.trim();
    let user = if user.is_empty() {
        String::new()
    } else {
        format!("{user}{SCP_USER_SEPARATOR}")
    };
    let port = if port.is_empty() {
        String::new()
    } else {
        format!("{SCP_HOST_SEPARATOR}{port}")
    };
    let path = form.path.trim().trim_matches(URI_PATH_SEPARATOR);
    Ok(format!(
        "{}{SCHEME_SEPARATOR}{user}{host}{port}{URI_PATH_SEPARATOR}{path}",
        form.protocol.scheme()
    ))
}
