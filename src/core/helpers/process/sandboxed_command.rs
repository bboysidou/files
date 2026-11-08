use std::env;
use std::path::Path;
use std::process::Command;

use crate::core::constants::sandbox::{
    SANDBOX_BASE_ARGUMENTS, SANDBOX_COMMAND, SANDBOX_COMMAND_SEPARATOR, SANDBOX_OPTIONAL_READS,
    SANDBOX_PASSED_VARIABLES, SANDBOX_PATH_VALUE, SANDBOX_READ_FLAG, SANDBOX_SETENV_FLAG,
    SANDBOX_SYSTEM_ROOT, SANDBOX_WRITE_FLAG,
};
use crate::core::helpers::process::find_program::find_program;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandbox_available::sandbox_available;

pub fn sandboxed_command(program: &str, access: &SandboxAccess) -> Command {
    let Some(resolved) = find_program(program).filter(|path| path.starts_with(SANDBOX_SYSTEM_ROOT))
    else {
        return Command::new(program);
    };
    if !sandbox_available() {
        return Command::new(program);
    }
    let mut command = Command::new(SANDBOX_COMMAND);
    command.args(SANDBOX_BASE_ARGUMENTS);
    let optional = SANDBOX_OPTIONAL_READS
        .iter()
        .map(Path::new)
        .filter(|path| path.exists());
    for path in optional.chain(access.read.iter().map(|path| path.as_path())) {
        command.arg(SANDBOX_READ_FLAG).arg(path).arg(path);
    }
    for path in &access.write {
        command.arg(SANDBOX_WRITE_FLAG).arg(path).arg(path);
    }
    command.args([SANDBOX_SETENV_FLAG, "PATH", SANDBOX_PATH_VALUE]);
    for name in SANDBOX_PASSED_VARIABLES {
        if let Ok(value) = env::var(name) {
            command.args([SANDBOX_SETENV_FLAG, name]).arg(value);
        }
    }
    command.arg(SANDBOX_COMMAND_SEPARATOR).arg(resolved);
    command
}
