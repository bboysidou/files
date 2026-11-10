use std::path::PathBuf;

use gpui::{Pixels, Point};

use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::drop_request::DropRequest;
use crate::features::browser::presentation::state::server_edit::PendingEdit;
use crate::features::operations::domain::entities::file_operation::Transfer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PaneEvent {
    RenameRequested(Transfer),
    ViewOptionsChanged(ViewOptions),
    Message(String),
    ContextMenu(Point<Pixels>),
    RestoreRequested(Vec<PathBuf>),
    Picked(Vec<PathBuf>),
    PickCancelled,
    DropRequested(DropRequest),
    BulkRenameRequested(Vec<PathBuf>),
    Navigated(PathBuf),
    OpenRequested(PathBuf),
    CredentialsRequested {
        uri: String,
        rejected: bool,
    },
    Connected {
        uri: String,
        edit: Option<PendingEdit>,
    },
}
