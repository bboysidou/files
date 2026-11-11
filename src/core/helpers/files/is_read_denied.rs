use std::path::Path;

use rustix::fs::{Access, access};
use rustix::io::Errno;

pub fn is_read_denied(path: &Path) -> bool {
    access(path, Access::READ_OK) == Err(Errno::ACCESS)
}
