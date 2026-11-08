use std::path::PathBuf;

use crate::core::dependency_injection::applications::{app_catalog, app_launcher};
use crate::core::dependency_injection::archives::archive_extractor;
use crate::core::dependency_injection::browser::{
    directory_repository, directory_watcher, file_launcher,
};
use crate::core::dependency_injection::operations::{file_operations, system_clipboard, trash_bin};
use crate::core::dependency_injection::preview::preview_source;
use crate::core::dependency_injection::search::search_indexer;
use crate::core::dependency_injection::thumbnails::thumbnail_store;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};

pub fn browser_setup(
    places: Vec<Place>,
    home: Option<PathBuf>,
    home_target: PlaceTarget,
) -> BrowserSetup {
    let repository = directory_repository();
    BrowserSetup {
        preview: preview_source(repository.clone()),
        thumbnails: thumbnail_store(),
        indexer: search_indexer(),
        repository,
        launcher: file_launcher(),
        apps: app_catalog(),
        app_launcher: app_launcher(),
        extractor: archive_extractor(),
        watcher: directory_watcher(),
        operations: file_operations(),
        system_clipboard: system_clipboard(),
        trash_bin: trash_bin(),
        places,
        home,
        home_target,
    }
}
