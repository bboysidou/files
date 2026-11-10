use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

#[derive(Clone, PartialEq, Debug)]
pub struct PreviewState {
    pub entry: FileEntry,
    pub content: Option<PreviewContent>,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}
