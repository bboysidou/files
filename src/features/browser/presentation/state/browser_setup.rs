use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;
use crate::features::places::domain::entities::place::Place;

pub struct BrowserSetup {
    pub repository: Arc<dyn DirectoryRepository>,
    pub launcher: Rc<dyn FileLauncher>,
    pub places: Vec<Place>,
    pub home: Option<PathBuf>,
}
