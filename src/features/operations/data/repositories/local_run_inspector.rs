use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use rustix::fs::{StatVfsMountFlags, statvfs};

use crate::core::constants::operations::{
    APPIMAGE_MAGIC, APPIMAGE_MAGIC_OFFSET, ELF_MAGIC, EXEC_BITS, ORIGIN_XATTR, RUN_HEAD_BYTES,
    SHEBANG, SHEBANG_MAX_CHARS,
};
use crate::core::enums::operations::run_kind::RunKind;
use crate::features::operations::domain::entities::run_plan::RunPlan;
use crate::features::operations::domain::repositories::run_inspector::RunInspector;

pub struct LocalRunInspector;

fn head(path: &Path) -> Vec<u8> {
    let mut buffer = vec![0_u8; RUN_HEAD_BYTES];
    let read = File::open(path)
        .and_then(|mut file| file.read(&mut buffer))
        .unwrap_or(0);
    buffer.truncate(read);
    buffer
}

fn kind_of(bytes: &[u8]) -> RunKind {
    let appimage_end = APPIMAGE_MAGIC_OFFSET + APPIMAGE_MAGIC.len();
    if bytes.starts_with(&ELF_MAGIC) {
        if bytes.get(APPIMAGE_MAGIC_OFFSET..appimage_end) == Some(&APPIMAGE_MAGIC[..]) {
            return RunKind::AppImage;
        }
        return RunKind::Elf;
    }
    if bytes.starts_with(SHEBANG) {
        return RunKind::Script;
    }
    RunKind::Plain
}

fn interpreter(bytes: &[u8]) -> Option<String> {
    let line = bytes
        .strip_prefix(SHEBANG)?
        .split(|byte| *byte == b'\n')
        .next()?;
    let text: String = String::from_utf8_lossy(line)
        .trim()
        .chars()
        .filter(|character| !character.is_control())
        .take(SHEBANG_MAX_CHARS)
        .collect();
    (!text.is_empty()).then_some(text)
}

fn noexec(path: &Path) -> bool {
    let folder = path.parent().unwrap_or(path);
    statvfs(folder).is_ok_and(|info| info.f_flag.contains(StatVfsMountFlags::NOEXEC))
}

fn origin(path: &Path) -> Option<String> {
    let value = xattr::get(path, ORIGIN_XATTR).ok()??;
    let text = String::from_utf8_lossy(&value).trim().to_string();
    (!text.is_empty()).then_some(text)
}

impl RunInspector for LocalRunInspector {
    fn inspect(&self, path: &Path) -> RunPlan {
        let bytes = head(path);
        let mode = fs::metadata(path)
            .map(|meta| meta.permissions().mode())
            .unwrap_or(0);
        let kind = kind_of(&bytes);
        RunPlan {
            path: path.to_path_buf(),
            kind,
            interpreter: (kind == RunKind::Script)
                .then(|| interpreter(&bytes))
                .flatten(),
            mode,
            executable: mode & EXEC_BITS != 0,
            noexec_mount: noexec(path),
            origin: origin(path),
        }
    }
}
