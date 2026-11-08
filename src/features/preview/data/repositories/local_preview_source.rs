use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::core::constants::preview::{
    MARKDOWN_EXTENSIONS, PDF_EXTENSION, PREVIEW_FOLDER_LIMIT, PREVIEW_MAX_LINES,
    PREVIEW_SNIFF_BYTES, PREVIEW_TAB, PREVIEW_TEXT_MAX_BYTES,
};
use crate::core::helpers::files::is_image::is_image;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::preview::data::datasources::markdown_parser::parse_markdown;
use crate::features::preview::data::datasources::pdf_renderer::PdfRenderer;
use crate::features::preview::data::datasources::syntax_highlighter::SyntaxHighlighter;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::features::preview::domain::entities::text_line::TextLine;
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

const NUL: u8 = 0;
const NO_FILTER: &str = "";

pub struct LocalPreviewSource {
    directories: Arc<dyn DirectoryRepository>,
    highlighter: SyntaxHighlighter,
    pdf: PdfRenderer,
}

impl LocalPreviewSource {
    pub fn new(directories: Arc<dyn DirectoryRepository>) -> Self {
        Self {
            directories,
            highlighter: SyntaxHighlighter::new(),
            pdf: PdfRenderer,
        }
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

    fn read_text(path: &Path) -> Result<(String, bool), PreviewContent> {
        let file = File::open(path).map_err(|_| PreviewContent::Failed)?;
        let mut bytes = Vec::new();
        file.take(PREVIEW_TEXT_MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| PreviewContent::Failed)?;
        if bytes.is_empty() {
            return Err(PreviewContent::Empty);
        }
        if bytes
            .iter()
            .take(PREVIEW_SNIFF_BYTES)
            .any(|byte| *byte == NUL)
        {
            return Err(PreviewContent::Unsupported);
        }
        let over_size = bytes.len() as u64 > PREVIEW_TEXT_MAX_BYTES;
        bytes.truncate(PREVIEW_TEXT_MAX_BYTES as usize);
        let text = String::from_utf8_lossy(&bytes).replace('\t', PREVIEW_TAB);
        Ok((text, over_size))
    }

    fn text(&self, entry: &FileEntry) -> PreviewContent {
        let (text, over_size) = match Self::read_text(&entry.path) {
            Ok(read) => read,
            Err(content) => return content,
        };
        if is_markdown(entry.extension.as_deref()) {
            return PreviewContent::Markdown {
                blocks: parse_markdown(&text),
                truncated: over_size,
            };
        }
        let plain: Vec<&str> = text.lines().collect();
        let truncated = over_size || plain.len() > PREVIEW_MAX_LINES;
        let colored = self
            .highlighter
            .highlight(&entry.path, &text)
            .unwrap_or_default();
        let lines = plain
            .into_iter()
            .take(PREVIEW_MAX_LINES)
            .enumerate()
            .map(|(index, line)| {
                colored.get(index).cloned().unwrap_or_else(|| TextLine {
                    text: line.to_string(),
                    runs: Vec::new(),
                })
            })
            .collect();
        PreviewContent::Text { lines, truncated }
    }

    fn pdf(&self, path: &Path) -> PreviewContent {
        let Some(page) = self.pdf.first_page(path) else {
            return PreviewContent::Unsupported;
        };
        let dimensions = imagesize::size(&page)
            .ok()
            .map(|size| (size.width, size.height));
        PreviewContent::Pdf {
            page,
            dimensions,
            pages: self.pdf.page_count(path),
        }
    }
}

fn is_markdown(extension: Option<&str>) -> bool {
    extension.is_some_and(|extension| {
        MARKDOWN_EXTENSIONS
            .iter()
            .any(|known| known.eq_ignore_ascii_case(extension))
    })
}

fn is_pdf(extension: Option<&str>) -> bool {
    extension.is_some_and(|extension| extension.eq_ignore_ascii_case(PDF_EXTENSION))
}

impl PreviewSource for LocalPreviewSource {
    fn load(&self, entry: &FileEntry) -> PreviewContent {
        if entry.is_dir {
            self.folder(&entry.path)
        } else if is_image(entry.extension.as_deref()) {
            Self::image(&entry.path)
        } else if is_pdf(entry.extension.as_deref()) {
            self.pdf(&entry.path)
        } else {
            self.text(entry)
        }
    }
}
