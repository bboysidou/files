use std::path::Path;

use crate::core::helpers::files::write_xdg_tags::write_xdg_tags;
use crate::core::helpers::files::xdg_tags::xdg_tags;
use crate::features::tags::domain::repositories::tag_store::TagStore;

pub struct XattrTagStore;

impl TagStore for XattrTagStore {
    fn names(&self, path: &Path) -> Vec<String> {
        xdg_tags(path)
    }

    fn set_names(&self, path: &Path, names: &[String]) -> Result<(), String> {
        write_xdg_tags(path, names).map_err(|error| error.to_string())
    }
}
