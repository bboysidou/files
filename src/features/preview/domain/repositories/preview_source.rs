use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

pub trait PreviewSource: Send + Sync {
    fn load(&self, entry: &FileEntry) -> PreviewContent;
}
