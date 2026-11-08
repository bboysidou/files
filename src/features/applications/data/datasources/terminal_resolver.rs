use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::core::constants::applications::{
    DEFAULT_RUN_ARGUMENTS, EXECUTE_BITS, KNOWN_TERMINALS, KnownTerminal, PATH_SEPARATOR,
    PATH_VARIABLE, TERMINAL_VARIABLE,
};
use crate::features::applications::data::models::terminal_profile::TerminalProfile;

pub fn resolve_terminal() -> Option<TerminalProfile> {
    env_terminal().or_else(|| {
        KNOWN_TERMINALS
            .iter()
            .find(|(command, _, _)| on_path(command[0]))
            .map(profile)
    })
}

fn env_terminal() -> Option<TerminalProfile> {
    let value = env::var(TERMINAL_VARIABLE).ok()?;
    let command: Vec<String> = value.split_whitespace().map(str::to_owned).collect();
    let program = command.first()?;
    if !on_path(program) {
        return None;
    }
    let name = Path::new(program).file_name()?.to_str()?;
    let known = KNOWN_TERMINALS
        .iter()
        .find(|(known, _, _)| known.len() == 1 && known[0] == name);
    Some(match known {
        Some(entry) if command.len() == 1 => profile(entry),
        _ => TerminalProfile {
            command,
            run: owned(DEFAULT_RUN_ARGUMENTS),
            directory_flag: None,
        },
    })
}

fn profile(entry: &KnownTerminal) -> TerminalProfile {
    let (command, run, directory_flag) = entry;
    TerminalProfile {
        command: owned(command),
        run: owned(run),
        directory_flag: directory_flag.map(str::to_owned),
    }
}

fn owned(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

fn on_path(program: &str) -> bool {
    if program.contains(PATH_SEPARATOR) {
        return is_executable(Path::new(program));
    }
    env::var_os(PATH_VARIABLE)
        .is_some_and(|paths| env::split_paths(&paths).any(|dir| is_executable(&dir.join(program))))
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & EXECUTE_BITS != 0)
}
