use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

use crate::core::enums::assets::icon::Icon;

pub struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Icon::ALL
            .iter()
            .find(|icon| icon.path() == path)
            .map(|icon| Cow::Borrowed(icon.bytes())))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Icon::ALL
            .iter()
            .map(|icon| icon.path())
            .filter(|asset| asset.starts_with(path))
            .map(SharedString::new_static)
            .collect())
    }
}
