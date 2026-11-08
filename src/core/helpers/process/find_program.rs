use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::core::constants::applications::{EXECUTE_BITS, PATH_VARIABLE};

pub fn find_program(name: &str) -> Option<PathBuf> {
    let paths = env::var_os(PATH_VARIABLE)?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|candidate| {
            fs::metadata(candidate)
                .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & EXECUTE_BITS != 0)
        })
}
