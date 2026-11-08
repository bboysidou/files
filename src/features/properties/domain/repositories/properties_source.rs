use std::path::Path;

use crate::features::properties::domain::entities::file_properties::FileProperties;

pub trait PropertiesSource: Send + Sync {
    fn read(&self, path: &Path) -> Option<FileProperties>;
}
