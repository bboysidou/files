use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;

pub fn run_operation(
    operation: FileOperation,
    files: &dyn FileOperations,
) -> Result<FileOperation, OperationError> {
    match &operation {
        FileOperation::Copy(transfers) => {
            for transfer in transfers {
                files.copy(&transfer.from, &transfer.to)?;
            }
        }
        FileOperation::Move(transfers) => {
            for transfer in transfers {
                files.move_path(&transfer.from, &transfer.to)?;
            }
        }
        FileOperation::Rename(transfer) => files.move_path(&transfer.from, &transfer.to)?,
        FileOperation::Trash(paths) => files.trash(paths)?,
        FileOperation::Restore(paths) => files.restore(paths)?,
        FileOperation::CreateFolder(path) => files.create_folder(path)?,
        FileOperation::RemoveFolder(path) => files.remove_folder(path)?,
    }
    Ok(operation)
}
