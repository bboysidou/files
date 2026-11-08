pub const PREVIEW_TEXT_MAX_BYTES: u64 = 256 * 1024;
pub const PREVIEW_SNIFF_BYTES: usize = 8 * 1024;
pub const PREVIEW_MAX_LINES: usize = 10_000;
pub const PREVIEW_FOLDER_LIMIT: usize = 40;
pub const PREVIEW_TAB: &str = "    ";
pub const IMAGE_EXTENSIONS: [&str; 13] = [
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "tif", "tiff", "ico", "qoi", "tga", "pnm", "svg",
];
pub const ZOOM_STEP: f32 = 1.25;
pub const ZOOM_MIN: f32 = 0.25;
pub const ZOOM_MAX: f32 = 4.0;
pub const ZOOM_DEFAULT: f32 = 1.0;
pub const PREVIEW_HIGHLIGHT_LINES: usize = 3_000;
pub const PREVIEW_SYNTAX_THEME: &str = "base16-eighties.dark";
pub const PREVIEW_PDF_DIR: &str = "files/pdf";
pub const PREVIEW_PDF_SCALE: &str = "1600";
pub const PDFTOPPM_COMMAND: &str = "pdftoppm";
pub const PDFINFO_COMMAND: &str = "pdfinfo";
pub const PDFINFO_PAGES: &str = "Pages:";
pub const PDF_EXTENSION: &str = "pdf";
pub const MARKDOWN_EXTENSIONS: [&str; 3] = ["md", "markdown", "mdown"];
pub const MARKDOWN_BULLET: &str = "•";
pub const MARKDOWN_TASK_DONE: &str = "[x]";
pub const MARKDOWN_TASK_OPEN: &str = "[ ]";
pub const MARKDOWN_CELL_SEPARATOR: &str = "  │  ";
