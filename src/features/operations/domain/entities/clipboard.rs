use std::path::PathBuf;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Clipboard {
    pub mode: ClipboardMode,
    pub paths: Vec<PathBuf>,
}
