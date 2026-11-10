use std::path::Path;

use crate::features::compare::domain::entities::compare_result::CompareResult;

pub trait FileComparer: Send + Sync {
    fn compare(&self, left: &Path, right: &Path) -> CompareResult;
}
