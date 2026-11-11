use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::core::enums::files::filesystem_kind::FilesystemKind;
use crate::core::helpers::files::is_read_denied::is_read_denied;
use crate::core::helpers::files::is_write_denied::is_write_denied;
use crate::features::admin::data::datasources::pkexec_session::PkexecSession;
use crate::features::admin::data::models::admin_reply::AdminReply;
use crate::features::admin::data::models::admin_request::AdminRequest;
use crate::features::admin::domain::repositories::admin_session::AdminSession;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub struct AdminFileOperations {
    local: Arc<dyn FileOperations>,
    session: Arc<PkexecSession>,
}

impl AdminFileOperations {
    pub fn new(local: Arc<dyn FileOperations>, session: Arc<PkexecSession>) -> Self {
        Self { local, session }
    }

    fn hidden(&self, path: &Path) -> bool {
        is_read_denied(path) && self.session.is_active()
    }

    fn guarded(&self, paths: &[&Path]) -> bool {
        paths
            .iter()
            .any(|path| is_read_denied(path) || is_write_denied(path))
            && self.session.is_active()
    }

    fn flag(&self, request: AdminRequest) -> bool {
        matches!(
            self.session.call(&request),
            Some(AdminReply::Flag { value: true })
        )
    }

    fn number(&self, request: AdminRequest) -> u64 {
        match self.session.call(&request) {
            Some(AdminReply::Number { value }) => value,
            _ => 0,
        }
    }

    fn remote(&self, request: AdminRequest) -> Result<(), OperationError> {
        match self.session.call(&request) {
            Some(AdminReply::Done) => Ok(()),
            Some(AdminReply::Fault { error }) => Err(error),
            _ => Err(OperationError::PermissionDenied),
        }
    }

    fn mutate(
        &self,
        paths: &[&Path],
        local: impl FnOnce() -> Result<(), OperationError>,
        request: impl FnOnce() -> AdminRequest,
    ) -> Result<(), OperationError> {
        if self.guarded(paths) {
            return self.remote(request());
        }
        match local() {
            Err(OperationError::PermissionDenied) if self.session.is_active() => {
                self.remote(request())
            }
            outcome => outcome,
        }
    }
}

impl FileOperations for AdminFileOperations {
    fn exists(&self, path: &Path) -> bool {
        if self.hidden(path) {
            return self.flag(AdminRequest::Exists { path: path.into() });
        }
        self.local.exists(path)
    }

    fn is_same_file(&self, first: &Path, second: &Path) -> bool {
        if self.hidden(first) || self.hidden(second) {
            return self.flag(AdminRequest::IsSameFile {
                first: first.into(),
                second: second.into(),
            });
        }
        self.local.is_same_file(first, second)
    }

    fn is_dir(&self, path: &Path) -> bool {
        if self.hidden(path) {
            return self.flag(AdminRequest::IsDir { path: path.into() });
        }
        self.local.is_dir(path)
    }

    fn children(&self, path: &Path) -> Vec<PathBuf> {
        if !self.hidden(path) {
            return self.local.children(path);
        }
        match self
            .session
            .call(&AdminRequest::Children { path: path.into() })
        {
            Some(AdminReply::Paths { paths }) => paths,
            _ => Vec::new(),
        }
    }

    fn copy(&self, from: &Path, to: &Path, sink: &dyn ProgressSink) -> Result<(), OperationError> {
        self.mutate(
            &[from, to],
            || self.local.copy(from, to, sink),
            || AdminRequest::Copy {
                from: from.into(),
                to: to.into(),
            },
        )
    }

    fn move_path(
        &self,
        from: &Path,
        to: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), OperationError> {
        self.mutate(
            &[from, to],
            || self.local.move_path(from, to, sink),
            || AdminRequest::Move {
                from: from.into(),
                to: to.into(),
            },
        )
    }

    fn size_of(&self, path: &Path) -> u64 {
        if self.hidden(path) {
            return self.number(AdminRequest::SizeOf { path: path.into() });
        }
        self.local.size_of(path)
    }

    fn filesystem_kind(&self, path: &Path) -> FilesystemKind {
        self.local.filesystem_kind(path)
    }

    fn has_file_over(&self, path: &Path, limit: u64) -> bool {
        if self.hidden(path) {
            return self.flag(AdminRequest::HasFileOver {
                path: path.into(),
                limit,
            });
        }
        self.local.has_file_over(path, limit)
    }

    fn transfer_bytes(&self, from: &Path, to: &Path) -> u64 {
        if self.hidden(from) {
            return self.number(AdminRequest::TransferBytes {
                from: from.into(),
                to: to.into(),
            });
        }
        self.local.transfer_bytes(from, to)
    }

    fn trash(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        let refs: Vec<&Path> = paths.iter().map(PathBuf::as_path).collect();
        if self.guarded(&refs) {
            return Err(OperationError::AdminNoTrash);
        }
        self.local.trash(paths)
    }

    fn delete(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        let refs: Vec<&Path> = paths.iter().map(PathBuf::as_path).collect();
        self.mutate(
            &refs,
            || self.local.delete(paths),
            || AdminRequest::Delete {
                paths: paths.to_vec(),
            },
        )
    }

    fn restore(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        self.local.restore(paths)
    }

    fn create_folder(&self, path: &Path) -> Result<(), OperationError> {
        self.mutate(
            &[path],
            || self.local.create_folder(path),
            || AdminRequest::CreateFolder { path: path.into() },
        )
    }

    fn remove_folder(&self, path: &Path) -> Result<(), OperationError> {
        self.mutate(
            &[path],
            || self.local.remove_folder(path),
            || AdminRequest::RemoveFolder { path: path.into() },
        )
    }

    fn create_file(&self, path: &Path) -> Result<(), OperationError> {
        self.mutate(
            &[path],
            || self.local.create_file(path),
            || AdminRequest::CreateFile { path: path.into() },
        )
    }

    fn link(&self, target: &Path, link: &Path) -> Result<(), OperationError> {
        self.mutate(
            &[link],
            || self.local.link(target, link),
            || AdminRequest::Link {
                target: target.into(),
                link: link.into(),
            },
        )
    }

    fn set_mode(&self, path: &Path, mode: u32) -> Result<(), OperationError> {
        self.mutate(
            &[path],
            || self.local.set_mode(path, mode),
            || AdminRequest::SetMode {
                path: path.into(),
                mode,
            },
        )
    }
}
