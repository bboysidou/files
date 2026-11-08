use std::sync::Arc;

use crate::features::operations::data::repositories::local_file_operations::LocalFileOperations;
use crate::features::operations::domain::repositories::file_operations::FileOperations;

pub fn file_operations() -> Arc<dyn FileOperations> {
    Arc::new(LocalFileOperations)
}
