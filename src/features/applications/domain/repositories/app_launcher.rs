use std::path::{Path, PathBuf};

use crate::features::applications::domain::entities::desktop_app::DesktopApp;
use crate::features::applications::domain::entities::launch_error::LaunchError;

pub trait AppLauncher {
    fn launch(&self, app: &DesktopApp, paths: &[PathBuf]) -> Result<(), LaunchError>;
    fn open_terminal(&self, directory: &Path) -> Result<(), LaunchError>;
    fn run_program(&self, program: &Path, in_terminal: bool) -> Result<(), LaunchError>;
}
