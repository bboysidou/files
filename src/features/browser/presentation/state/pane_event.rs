use crate::features::operations::domain::entities::file_operation::Transfer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PaneEvent {
    RenameRequested(Transfer),
}
