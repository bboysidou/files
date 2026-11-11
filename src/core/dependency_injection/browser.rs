use std::rc::Rc;
use std::sync::Arc;

use crate::core::dependency_injection::admin::pkexec_session;
use crate::features::admin::data::repositories::admin_directory_repository::AdminDirectoryRepository;
use crate::features::browser::data::repositories::local_directory_repository::LocalDirectoryRepository;
use crate::features::browser::data::repositories::notify_directory_watcher::NotifyDirectoryWatcher;
use crate::features::browser::data::repositories::xdg_file_launcher::XdgFileLauncher;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::repositories::directory_watcher::DirectoryWatcher;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;

pub fn directory_repository() -> Arc<dyn DirectoryRepository> {
    Arc::new(AdminDirectoryRepository::new(
        Arc::new(LocalDirectoryRepository),
        pkexec_session(),
    ))
}

pub fn file_launcher() -> Rc<dyn FileLauncher> {
    Rc::new(XdgFileLauncher)
}

pub fn directory_watcher() -> Arc<dyn DirectoryWatcher> {
    Arc::new(NotifyDirectoryWatcher)
}
