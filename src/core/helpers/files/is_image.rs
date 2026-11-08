use crate::core::constants::preview::IMAGE_EXTENSIONS;

pub fn is_image(extension: Option<&str>) -> bool {
    extension.is_some_and(|extension| {
        IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
    })
}
