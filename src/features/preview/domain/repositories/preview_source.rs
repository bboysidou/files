use std::path::Path;

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

pub trait PreviewSource: Send + Sync {
    fn load(&self, entry: &FileEntry) -> PreviewContent;
    fn pdf_page(&self, path: &Path, number: usize) -> PreviewContent;
}
