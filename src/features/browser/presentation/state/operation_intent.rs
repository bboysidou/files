use crate::features::operations::domain::entities::file_operation::FileOperation;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OperationIntent {
    Fresh,
    FreshThenRename,
    Undo(FileOperation),
    Redo(FileOperation),
}
