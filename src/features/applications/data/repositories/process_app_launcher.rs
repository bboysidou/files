use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

use crate::features::applications::data::datasources::terminal_resolver::resolve_terminal;
use crate::features::applications::domain::entities::desktop_app::DesktopApp;
use crate::features::applications::domain::entities::launch_error::LaunchError;
use crate::features::applications::domain::repositories::app_launcher::AppLauncher;
use crate::features::applications::domain::usecases::expand_exec::expand_exec;

pub struct ProcessAppLauncher;

impl AppLauncher for ProcessAppLauncher {
    fn launch(&self, app: &DesktopApp, paths: &[PathBuf]) -> Result<(), LaunchError> {
        let argv = expand_exec(&app.exec, paths).ok_or(LaunchError::BadCommand)?;
        let argv = if app.terminal {
            let terminal = resolve_terminal().ok_or(LaunchError::NoTerminal)?;
            terminal
                .command
                .into_iter()
                .chain(terminal.run)
                .chain(argv)
                .collect()
        } else {
            argv
        };
        spawn_detached(&argv, paths.first().and_then(|path| path.parent()))
    }

    fn open_terminal(&self, directory: &Path) -> Result<(), LaunchError> {
        let terminal = resolve_terminal().ok_or(LaunchError::NoTerminal)?;
        let mut argv = terminal.command;
        if let Some(flag) = terminal.directory_flag {
            argv.push(format!("{flag}{}", directory.display()));
        }
        spawn_detached(&argv, Some(directory))
    }

    fn run_program(&self, program: &Path, in_terminal: bool) -> Result<(), LaunchError> {
        let target = vec![program.to_string_lossy().into_owned()];
        let argv = if in_terminal {
            let terminal = resolve_terminal().ok_or(LaunchError::NoTerminal)?;
            terminal
                .command
                .into_iter()
                .chain(terminal.run)
                .chain(target)
                .collect()
        } else {
            target
        };
        spawn_detached(&argv, program.parent())
    }
}

fn spawn_detached(argv: &[String], directory: Option<&Path>) -> Result<(), LaunchError> {
    let (program, arguments) = argv.split_first().ok_or(LaunchError::BadCommand)?;
    let mut command = Command::new(program);
    command
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(directory) = directory {
        command.current_dir(directory);
    }
    let mut child = command
        .spawn()
        .map_err(|error| LaunchError::Spawn(error.to_string()))?;
    thread::spawn(move || child.wait());
    Ok(())
}
