use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::core::constants::preview::{
    FC_SCAN_COMMAND, FC_SCAN_FAMILY, FONT_SPECIMEN_BACKGROUND, FONT_SPECIMEN_INK,
    FONT_SPECIMEN_POINTS, FONT_SPECIMEN_TEXT, MAGICK_COMMAND, PNG_EXTENSION, PREVIEW_MEDIA_DIR,
};
use crate::core::helpers::files::cache_name::cache_name;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;

const LABEL_PREFIX: &str = "label:";

pub fn font_family(path: &Path) -> Option<String> {
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let output = sandboxed_command(FC_SCAN_COMMAND, &access)
        .args(FC_SCAN_FAMILY)
        .arg(path)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let family = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (!family.is_empty()).then_some(family)
}

pub fn font_specimen(path: &Path) -> Option<PathBuf> {
    let dir = dirs::cache_dir()?.join(PREVIEW_MEDIA_DIR);
    let image = dir.join(cache_name(path)?).with_extension(PNG_EXTENSION);
    if image.exists() {
        return Some(image);
    }
    fs::create_dir_all(&dir).ok()?;
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: vec![dir],
    };
    let status = sandboxed_command(MAGICK_COMMAND, &access)
        .args([
            "-background",
            FONT_SPECIMEN_BACKGROUND,
            "-fill",
            FONT_SPECIMEN_INK,
        ])
        .arg("-font")
        .arg(path)
        .args(["-pointsize", FONT_SPECIMEN_POINTS])
        .arg(format!("{LABEL_PREFIX}{FONT_SPECIMEN_TEXT}"))
        .arg(&image)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    (status.success() && image.exists()).then_some(image)
}
