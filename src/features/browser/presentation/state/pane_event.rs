use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::operations::domain::entities::file_operation::Transfer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PaneEvent {
    RenameRequested(Transfer),
    ViewOptionsChanged(ViewOptions),
    Message(String),
}
