use std::process::{Command, Stdio};
use std::sync::OnceLock;

use crate::core::constants::sandbox::{
    SANDBOX_BASE_ARGUMENTS, SANDBOX_COMMAND, SANDBOX_COMMAND_SEPARATOR, SANDBOX_PROBE_PROGRAM,
};
use crate::core::helpers::process::find_program::find_program;

static AVAILABLE: OnceLock<bool> = OnceLock::new();

pub fn sandbox_available() -> bool {
    *AVAILABLE.get_or_init(|| {
        let Some(probe) = find_program(SANDBOX_PROBE_PROGRAM) else {
            return false;
        };
        Command::new(SANDBOX_COMMAND)
            .args(SANDBOX_BASE_ARGUMENTS)
            .arg(SANDBOX_COMMAND_SEPARATOR)
            .arg(probe)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    })
}
