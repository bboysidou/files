use std::path::PathBuf;

use crate::features::compare::domain::entities::compare_result::CompareResult;

pub struct CompareState {
    pub left: PathBuf,
    pub right: PathBuf,
    pub result: Option<CompareResult>,
}
