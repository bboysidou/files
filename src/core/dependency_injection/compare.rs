use std::sync::Arc;

use crate::features::compare::data::repositories::local_file_comparer::LocalFileComparer;
use crate::features::compare::domain::repositories::file_comparer::FileComparer;

pub fn file_comparer() -> Arc<dyn FileComparer> {
    Arc::new(LocalFileComparer)
}
