use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use crate::core::constants::preview::{
    MARKDOWN_EXTENSIONS, PDF_EXTENSION, PREVIEW_FOLDER_LIMIT, PREVIEW_MAX_LINES,
    PREVIEW_SNIFF_BYTES, PREVIEW_TAB, PREVIEW_TEXT_MAX_BYTES,
};
use crate::core::enums::files::file_kind::FileKind;
use crate::core::helpers::files::is_archive::is_archive;
use crate::core::helpers::files::is_image::is_image;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::preview::data::datasources::archive_lister::list_archive;
use crate::features::preview::data::datasources::exif_reader::read_exif;
use crate::features::preview::data::datasources::font_specimen::{font_family, font_specimen};
use crate::features::preview::data::datasources::markdown_parser::parse_markdown;
use crate::features::preview::data::datasources::media_probe::{probe_media, video_frame};
use crate::features::preview::data::datasources::office_text::office_text;
use crate::features::preview::data::datasources::pdf_renderer::PdfRenderer;
use crate::features::preview::data::datasources::syntax_highlighter::SyntaxHighlighter;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::features::preview::domain::entities::text_line::TextLine;
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

const NUL: u8 = 0;
const FIRST_PAGE: usize = 1;
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
            exif: read_exif(path),
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
                    number: None,
                })
            })
            .collect();
        PreviewContent::Text { lines, truncated }
    }

    fn pdf(&self, path: &Path) -> PreviewContent {
        self.pdf_page(path, FIRST_PAGE)
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
    fn pdf_page(&self, path: &Path, number: usize) -> PreviewContent {
        let Some(page) = self.pdf.page(path, number) else {
            return PreviewContent::Unsupported;
        };
        let dimensions = imagesize::size(&page)
            .ok()
            .map(|size| (size.width, size.height));
        PreviewContent::Pdf {
            page,
            dimensions,
            pages: self.pdf.page_count(path),
            index: number,
        }
    }

    fn load(&self, entry: &FileEntry) -> PreviewContent {
        if entry.is_dir {
            self.folder(&entry.path)
        } else if is_image(entry.extension.as_deref()) {
            Self::image(&entry.path)
        } else if matches!(entry.kind, FileKind::Video | FileKind::Audio) {
            media(entry)
        } else if entry.kind == FileKind::Font {
            font_specimen(&entry.path).map_or(PreviewContent::Unsupported, |specimen| {
                PreviewContent::Font {
                    specimen,
                    family: font_family(&entry.path),
                }
            })
        } else if let Some(text) = entry
            .extension
            .as_deref()
            .and_then(|extension| office_text(&entry.path, extension))
        {
            plain_text(&text)
        } else if is_archive(entry) {
            list_archive(&entry.path).unwrap_or(PreviewContent::Unsupported)
        } else if is_pdf(entry.extension.as_deref()) {
            self.pdf(&entry.path)
        } else {
            self.text(entry)
        }
    }
}

fn media(entry: &FileEntry) -> PreviewContent {
    let Some(facts) = probe_media(&entry.path) else {
        return PreviewContent::Unsupported;
    };
    let frame = facts.video.as_ref().and_then(|_| video_frame(&entry.path));
    PreviewContent::Media { frame, facts }
}

fn plain_text(text: &str) -> PreviewContent {
    let lines: Vec<TextLine> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(PREVIEW_MAX_LINES + 1)
        .map(|line| TextLine {
            text: line.to_owned(),
            runs: Vec::new(),
            number: None,
        })
        .collect();
    if lines.is_empty() {
        return PreviewContent::Empty;
    }
    let truncated = lines.len() > PREVIEW_MAX_LINES;
    PreviewContent::Text {
        lines: lines.into_iter().take(PREVIEW_MAX_LINES).collect(),
        truncated,
    }
}
