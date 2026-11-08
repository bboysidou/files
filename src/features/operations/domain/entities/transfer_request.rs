use std::path::PathBuf;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransferRequest {
    pub sources: Vec<PathBuf>,
    pub destination: PathBuf,
    pub mode: ClipboardMode,
    pub policy: CollisionPolicy,
    pub in_place: bool,
}
