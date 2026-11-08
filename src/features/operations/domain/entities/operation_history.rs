use super::file_operation::FileOperation;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct OperationHistory {
    undo: Vec<FileOperation>,
    redo: Vec<FileOperation>,
}

impl OperationHistory {
    pub fn record(&mut self, operation: FileOperation) {
        self.undo.push(operation);
        self.redo.clear();
    }

    pub fn take_undo(&mut self) -> Option<FileOperation> {
        self.undo.pop()
    }

    pub fn take_redo(&mut self) -> Option<FileOperation> {
        self.redo.pop()
    }

    pub fn push_undo(&mut self, operation: FileOperation) {
        self.undo.push(operation);
    }

    pub fn push_redo(&mut self, operation: FileOperation) {
        self.redo.push(operation);
    }
}
