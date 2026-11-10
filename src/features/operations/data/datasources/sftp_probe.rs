use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};

use crate::core::constants::ssh::{
    SFTP_BATCH_FLAG, SFTP_COMMAND, SFTP_EMPTY_BATCH, SSH_BATCH_MODE, SSH_CONNECT_TIMEOUT,
    SSH_OPTION_FLAG, SSH_PORT_FLAG,
};

static REACHABLE: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();

pub fn sftp_reachable(destination: &str, port: Option<&str>) -> bool {
    let key = format!("{destination} {}", port.unwrap_or_default());
    let cache = REACHABLE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(known) = cache.lock().ok().and_then(|known| known.get(&key).copied()) {
        return known;
    }
    let reachable = probe(destination, port);
    if let Ok(mut known) = cache.lock() {
        known.insert(key, reachable);
    }
    reachable
}

fn probe(destination: &str, port: Option<&str>) -> bool {
    let mut command = Command::new(SFTP_COMMAND);
    command.args([
        SSH_OPTION_FLAG,
        SSH_BATCH_MODE,
        SSH_OPTION_FLAG,
        SSH_CONNECT_TIMEOUT,
        SFTP_BATCH_FLAG,
        SFTP_EMPTY_BATCH,
    ]);
    if let Some(port) = port {
        command.args([SSH_PORT_FLAG, port]);
    }
    command
        .arg(destination)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
