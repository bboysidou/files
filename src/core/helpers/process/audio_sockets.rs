use std::env;
use std::path::PathBuf;

use crate::core::constants::preview::{AUDIO_SOCKETS, XDG_RUNTIME_DIR};

pub fn audio_sockets() -> Vec<PathBuf> {
    let Some(runtime) = env::var_os(XDG_RUNTIME_DIR).map(PathBuf::from) else {
        return Vec::new();
    };
    AUDIO_SOCKETS
        .iter()
        .map(|socket| runtime.join(socket))
        .filter(|socket| socket.exists())
        .collect()
}
