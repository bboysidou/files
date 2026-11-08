use std::path::{Path, PathBuf};

use crate::core::constants::integration::{ARG_DEFAULT, ARG_OFF, ARG_SELECT};
use crate::core::helpers::files::path_from_uri::path_from_uri;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;

fn absolute(argument: &str, working: &Path) -> PathBuf {
    let path = path_from_uri(argument).unwrap_or_else(|| PathBuf::from(argument));
    if path.is_absolute() {
        path
    } else {
        working.join(path)
    }
}

pub fn parse_launch(arguments: &[String], working: &Path) -> LaunchRequest {
    match arguments {
        [] => LaunchRequest::Window,
        [flag] if flag == ARG_DEFAULT => LaunchRequest::SetDefault(true),
        [flag, value] if flag == ARG_DEFAULT => LaunchRequest::SetDefault(value != ARG_OFF),
        [flag, target, ..] if flag == ARG_SELECT => {
            LaunchRequest::Reveal(absolute(target, working))
        }
        [target, ..] => LaunchRequest::Open(absolute(target, working)),
    }
}
