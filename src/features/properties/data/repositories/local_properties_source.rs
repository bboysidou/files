use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

use crate::features::properties::data::datasources::account_names::{group_name, user_name};
use crate::features::properties::domain::entities::file_properties::FileProperties;
use crate::features::properties::domain::repositories::properties_source::PropertiesSource;

pub struct LocalPropertiesSource;

impl PropertiesSource for LocalPropertiesSource {
    fn read(&self, path: &Path) -> Option<FileProperties> {
        let link = fs::symlink_metadata(path).ok()?;
        let meta = fs::metadata(path).unwrap_or_else(|_| link.clone());
        Some(FileProperties {
            path: path.to_path_buf(),
            is_dir: meta.is_dir(),
            size: meta.len(),
            modified: meta.modified().ok(),
            accessed: meta.accessed().ok(),
            created: meta.created().ok(),
            owner: user_name(meta.uid()),
            group: group_name(meta.gid()),
            mode: meta.permissions().mode(),
            link_target: link
                .file_type()
                .is_symlink()
                .then(|| fs::read_link(path).ok())
                .flatten(),
        })
    }
}
