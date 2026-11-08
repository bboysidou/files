use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::repositories::directory_watcher::DirectoryWatcher;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

pub struct BrowserSetup {
    pub repository: Arc<dyn DirectoryRepository>,
    pub launcher: Rc<dyn FileLauncher>,
    pub watcher: Arc<dyn DirectoryWatcher>,
    pub operations: Arc<dyn FileOperations>,
    pub preview: Arc<dyn PreviewSource>,
    pub places: Vec<Place>,
    pub home: Option<PathBuf>,
    pub home_target: PlaceTarget,
}
