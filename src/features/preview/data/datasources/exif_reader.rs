use std::path::Path;
use std::process::Stdio;

use crate::core::constants::preview::{EXIF_FIELDS, EXIV2_COMMAND, EXIV2_KEY_FLAG, EXIV2_PRINT};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::preview::domain::entities::exif_fact::ExifFact;

pub fn read_exif(path: &Path) -> Vec<ExifFact> {
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let mut command = sandboxed_command(EXIV2_COMMAND, &access);
    command.args(EXIV2_PRINT);
    for (key, _) in EXIF_FIELDS {
        command.args([EXIV2_KEY_FLAG, key]);
    }
    let Ok(output) = command
        .arg(path)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
    else {
        return Vec::new();
    };
    let text = String::from_utf8_lossy(&output.stdout);
    EXIF_FIELDS
        .iter()
        .filter_map(|(key, label_key)| {
            let value = text.lines().find_map(|line| {
                let (found, value) = line.split_once(char::is_whitespace)?;
                (found == *key).then(|| value.trim().to_owned())
            })?;
            (!value.is_empty()).then_some(ExifFact { label_key, value })
        })
        .collect()
}
