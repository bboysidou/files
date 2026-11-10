use std::path::PathBuf;

use crate::features::compare::domain::entities::diff_row::DiffRow;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CompareResult {
    Text {
        rows: Vec<DiffRow>,
        changes: usize,
    },
    Images {
        left: PathBuf,
        right: PathBuf,
    },
    Binary {
        left_size: u64,
        right_size: u64,
        identical: bool,
    },
    Unreadable,
}
