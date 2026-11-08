use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crate::core::constants::files::OPENER_COMMAND;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;

pub struct XdgFileLauncher;

impl FileLauncher for XdgFileLauncher {
    fn launch(&self, path: &Path) {
        let spawned = Command::new(OPENER_COMMAND)
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if let Ok(mut child) = spawned {
            thread::spawn(move || child.wait());
        }
    }
}
