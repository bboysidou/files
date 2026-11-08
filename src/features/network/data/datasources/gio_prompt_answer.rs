use crate::core::constants::network::{PROMPT_DOMAIN, PROMPT_END, PROMPT_PASSWORD, PROMPT_USER};
use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;

pub fn gio_prompt_answer(
    line: &str,
    credentials: Option<&NetworkCredentials>,
    password_sent: &mut bool,
) -> Result<Option<String>, NetworkMountError> {
    if !line.ends_with(PROMPT_END) {
        return Ok(None);
    }
    let credentials = credentials.ok_or(NetworkMountError::NeedsCredentials);
    if line.starts_with(PROMPT_USER) {
        return credentials.map(|given| Some(given.user.clone()));
    }
    if line.starts_with(PROMPT_DOMAIN) {
        return Ok(Some(
            credentials.map_or(String::new(), |given| given.domain.clone()),
        ));
    }
    if line.starts_with(PROMPT_PASSWORD) {
        if *password_sent {
            return Err(NetworkMountError::Rejected);
        }
        *password_sent = true;
        return credentials.map(|given| Some(given.password.clone()));
    }
    Err(NetworkMountError::Failed(line.trim().to_owned()))
}
