use std::path::Path;

use rustix::fs::{Access, access};
use rustix::io::Errno;

pub fn is_write_denied(path: &Path) -> bool {
    path.parent()
        .is_some_and(|parent| access(parent, Access::WRITE_OK) == Err(Errno::ACCESS))
}
