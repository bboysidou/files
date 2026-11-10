use std::path::Path;

use rustix::fs::statfs;

use crate::core::constants::files::{
    EXFAT_MAGIC, FUSE_MAGIC, FUSE_NETWORK_TYPES, FUSEBLK_TYPE, MSDOS_MAGIC, NETWORK_MAGICS,
    NTFS_MAGIC, NTFS3_MAGIC,
};
use crate::core::enums::files::filesystem_kind::FilesystemKind;
use crate::core::helpers::files::fuse_type_of::fuse_type_of;

pub fn filesystem_kind_of(path: &Path) -> FilesystemKind {
    let Some(folder) = path.ancestors().find(|ancestor| ancestor.exists()) else {
        return FilesystemKind::Other;
    };
    let Ok(info) = statfs(folder) else {
        return FilesystemKind::Other;
    };
    match info.f_type as u32 {
        MSDOS_MAGIC => FilesystemKind::Fat,
        EXFAT_MAGIC => FilesystemKind::Exfat,
        NTFS3_MAGIC | NTFS_MAGIC => FilesystemKind::Ntfs,
        magic if NETWORK_MAGICS.contains(&magic) => FilesystemKind::Network,
        FUSE_MAGIC => match fuse_type_of(folder).as_deref() {
            Some(FUSEBLK_TYPE) => FilesystemKind::Ntfs,
            Some(kind) if FUSE_NETWORK_TYPES.contains(&kind) => FilesystemKind::Network,
            _ => FilesystemKind::Other,
        },
        _ => FilesystemKind::Other,
    }
}
