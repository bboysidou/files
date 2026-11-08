use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_encode};

const FILE_SCHEME: &str = "file://";
const URI_RESERVED: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b'-')
    .remove(b'.')
    .remove(b'/')
    .remove(b':')
    .remove(b'=')
    .remove(b'@')
    .remove(b'_')
    .remove(b'~');

pub fn file_uri(path: &Path) -> String {
    format!(
        "{FILE_SCHEME}{}",
        percent_encode(path.as_os_str().as_bytes(), URI_RESERVED)
    )
}
