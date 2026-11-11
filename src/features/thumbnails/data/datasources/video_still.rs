use std::path::Path;
use std::process::Stdio;

use image::DynamicImage;

use crate::core::constants::preview::{FFMPEG_COMMAND, FFMPEG_INPUT_FLAG, FFMPEG_QUIET};
use crate::core::constants::thumbnails::{VIDEO_STILL_OUTPUT, VIDEO_STILL_TARGET};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;

pub fn video_still(path: &Path) -> Option<DynamicImage> {
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let output = sandboxed_command(FFMPEG_COMMAND, &access)
        .args(FFMPEG_QUIET)
        .arg(FFMPEG_INPUT_FLAG)
        .arg(path)
        .args(VIDEO_STILL_OUTPUT)
        .arg(VIDEO_STILL_TARGET)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success() && !output.stdout.is_empty())?;
    image::load_from_memory(&output.stdout).ok()
}
