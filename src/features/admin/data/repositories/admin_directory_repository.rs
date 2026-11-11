use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::core::helpers::files::is_read_denied::is_read_denied;
use crate::features::admin::data::datasources::pkexec_session::PkexecSession;
use crate::features::admin::data::models::admin_reply::AdminReply;
use crate::features::admin::data::models::admin_request::AdminRequest;
use crate::features::admin::domain::repositories::admin_session::AdminSession;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::places::domain::entities::place::PlaceTarget;

pub struct AdminDirectoryRepository {
    local: Arc<dyn DirectoryRepository>,
    session: Arc<PkexecSession>,
}

impl AdminDirectoryRepository {
    pub fn new(local: Arc<dyn DirectoryRepository>, session: Arc<PkexecSession>) -> Self {
        Self { local, session }
    }
}

impl DirectoryRepository for AdminDirectoryRepository {
    fn list(&self, directory: &Path) -> Result<Vec<FileEntry>, ListError> {
        match self.local.list(directory) {
            Err(ListError::PermissionDenied) if self.session.is_active() => {
                match self.session.call(&AdminRequest::List {
                    path: directory.into(),
                }) {
                    Some(AdminReply::Entries { entries }) => Ok(entries),
                    _ => Err(ListError::PermissionDenied),
                }
            }
            outcome => outcome,
        }
    }

    fn entry(&self, path: &Path) -> Option<FileEntry> {
        if !(is_read_denied(path) && self.session.is_active()) {
            return self.local.entry(path);
        }
        match self
            .session
            .call(&AdminRequest::Entry { path: path.into() })
        {
            Some(AdminReply::Entry { entry }) => entry,
            _ => None,
        }
    }

    fn resolve(&self, target: &PlaceTarget) -> Option<PathBuf> {
        self.local.resolve(target)
    }
}
