use std::path::PathBuf;

use crate::core::enums::operations::run_kind::RunKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RunPlan {
    pub path: PathBuf,
    pub kind: RunKind,
    pub interpreter: Option<String>,
    pub mode: u32,
    pub executable: bool,
    pub noexec_mount: bool,
    pub origin: Option<String>,
}
