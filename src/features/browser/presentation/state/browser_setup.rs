use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use crate::features::applications::domain::repositories::app_catalog::AppCatalog;
use crate::features::applications::domain::repositories::app_launcher::AppLauncher;
use crate::features::archives::domain::repositories::archive_tool::ArchiveTool;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::repositories::directory_watcher::DirectoryWatcher;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::system_clipboard::SystemClipboard;
use crate::features::operations::domain::repositories::trash_bin::TrashBin;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};
use crate::features::preview::domain::repositories::preview_source::PreviewSource;
use crate::features::properties::domain::repositories::properties_source::PropertiesSource;
use crate::features::search::domain::repositories::content_searcher::ContentSearcher;
use crate::features::search::domain::repositories::search_indexer::SearchIndexer;
use crate::features::thumbnails::domain::repositories::thumbnail_store::ThumbnailStore;

pub struct BrowserSetup {
    pub repository: Arc<dyn DirectoryRepository>,
    pub launcher: Rc<dyn FileLauncher>,
    pub apps: Arc<dyn AppCatalog>,
    pub app_launcher: Rc<dyn AppLauncher>,
    pub archives: Arc<dyn ArchiveTool>,
    pub watcher: Arc<dyn DirectoryWatcher>,
    pub operations: Arc<dyn FileOperations>,
    pub system_clipboard: Arc<dyn SystemClipboard>,
    pub trash_bin: Arc<dyn TrashBin>,
    pub preview: Arc<dyn PreviewSource>,
    pub thumbnails: Arc<dyn ThumbnailStore>,
    pub indexer: Arc<dyn SearchIndexer>,
    pub content: Arc<dyn ContentSearcher>,
    pub properties: Arc<dyn PropertiesSource>,
    pub network: Arc<dyn NetworkMounter>,
    pub places: Vec<Place>,
    pub home: Option<PathBuf>,
    pub home_target: PlaceTarget,
}
