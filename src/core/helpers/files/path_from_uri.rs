use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use percent_encoding::percent_decode_str;

const FILE_SCHEME: &str = "file://";
const LOCAL_HOST: &str = "localhost";

pub fn path_from_uri(uri: &str) -> Option<PathBuf> {
    let rest = uri.trim().strip_prefix(FILE_SCHEME)?;
    let path = rest.strip_prefix(LOCAL_HOST).unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let bytes: Vec<u8> = percent_decode_str(path).collect();
    Some(PathBuf::from(OsString::from_vec(bytes)))
}
