use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::core::constants::preview::{
    IMAGE_EXTENSIONS, PREVIEW_FOLDER_LIMIT, PREVIEW_MAX_LINES, PREVIEW_SNIFF_BYTES, PREVIEW_TAB,
    PREVIEW_TEXT_MAX_BYTES,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

const NUL: u8 = 0;
const NO_FILTER: &str = "";

pub struct LocalPreviewSource {
    directories: Arc<dyn DirectoryRepository>,
}

impl LocalPreviewSource {
    pub fn new(directories: Arc<dyn DirectoryRepository>) -> Self {
        Self { directories }
    }

    fn is_image(entry: &FileEntry) -> bool {
        entry.extension.as_deref().is_some_and(|extension| {
            IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
        })
    }

    fn image(path: &Path) -> PreviewContent {
        let dimensions = imagesize::size(path)
            .ok()
            .map(|size| (size.width, size.height));
        PreviewContent::Image {
            path: path.to_path_buf(),
            dimensions,
        }
    }

    fn folder(&self, path: &Path) -> PreviewContent {
        let Ok(entries) = self.directories.list(path) else {
            return PreviewContent::Failed;
        };
        let visible = arrange_entries(&entries, SortOrder::by_name(), false, NO_FILTER);
        let total = visible.len();
        PreviewContent::Folder {
            entries: visible.into_iter().take(PREVIEW_FOLDER_LIMIT).collect(),
            total,
        }
    }

    fn text(path: &Path) -> PreviewContent {
        let Ok(file) = File::open(path) else {
            return PreviewContent::Failed;
        };
        let mut bytes = Vec::new();
        if file
            .take(PREVIEW_TEXT_MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .is_err()
        {
            return PreviewContent::Failed;
        }
        if bytes.is_empty() {
            return PreviewContent::Empty;
        }
        if bytes
            .iter()
            .take(PREVIEW_SNIFF_BYTES)
            .any(|byte| *byte == NUL)
        {
            return PreviewContent::Unsupported;
        }
        let over_size = bytes.len() as u64 > PREVIEW_TEXT_MAX_BYTES;
        bytes.truncate(PREVIEW_TEXT_MAX_BYTES as usize);
        let text = String::from_utf8_lossy(&bytes);
        let all_lines: Vec<&str> = text.lines().collect();
        let truncated = over_size || all_lines.len() > PREVIEW_MAX_LINES;
        let lines = all_lines
            .into_iter()
            .take(PREVIEW_MAX_LINES)
            .map(|line| line.replace('\t', PREVIEW_TAB))
            .collect();
        PreviewContent::Text { lines, truncated }
    }
}

impl PreviewSource for LocalPreviewSource {
    fn load(&self, entry: &FileEntry) -> PreviewContent {
        if entry.is_dir {
            self.folder(&entry.path)
        } else if Self::is_image(entry) {
            Self::image(&entry.path)
        } else {
            Self::text(&entry.path)
        }
    }
}
