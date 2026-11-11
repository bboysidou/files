use std::sync::{Arc, OnceLock};

use image::RgbaImage;

const APP_ICON_PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/logo/icons/128.png"
));

static APP_ICON: OnceLock<Option<Arc<RgbaImage>>> = OnceLock::new();

pub fn app_icon() -> Option<Arc<RgbaImage>> {
    APP_ICON
        .get_or_init(|| {
            image::load_from_memory(APP_ICON_PNG)
                .ok()
                .map(|decoded| Arc::new(decoded.into_rgba8()))
        })
        .clone()
}
