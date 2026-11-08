use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

fn item_name(transfer: &Transfer) -> String {
    transfer
        .from
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

pub fn run_operation(
    operation: FileOperation,
    files: &dyn FileOperations,
    sink: &dyn ProgressSink,
) -> Result<FileOperation, OperationError> {
    match &operation {
        FileOperation::Copy(transfers) => {
            sink.begin(
                transfers
                    .iter()
                    .map(|transfer| files.size_of(&transfer.from))
                    .sum(),
            );
            for transfer in transfers {
                sink.set_item(&item_name(transfer));
                files.copy(&transfer.from, &transfer.to, sink)?;
            }
        }
        FileOperation::Move(transfers) => {
            for transfer in transfers {
                sink.set_item(&item_name(transfer));
                files.move_path(&transfer.from, &transfer.to, sink)?;
            }
        }
        FileOperation::Rename(transfer) => files.move_path(&transfer.from, &transfer.to, sink)?,
        FileOperation::Trash(paths) => files.trash(paths)?,
        FileOperation::Restore(paths) => files.restore(paths)?,
        FileOperation::Extract(path) | FileOperation::Compress(path) => {
            files.restore(std::slice::from_ref(path))?
        }
        FileOperation::CreateFolder(path) => files.create_folder(path)?,
        FileOperation::CreateFile(path) => files.create_file(path)?,
        FileOperation::SetMode(change) => files.set_mode(&change.path, change.to)?,
        FileOperation::Link(transfers) => {
            for transfer in transfers {
                files.link(&transfer.from, &transfer.to)?;
            }
        }
        FileOperation::RemoveFolder(path) => files.remove_folder(path)?,
        FileOperation::Batch(steps) => {
            for step in steps {
                run_operation(step.clone(), files, sink)?;
            }
        }
    }
    Ok(operation)
}
