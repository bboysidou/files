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
pub const ARCHIVE_COMMAND: &str = "bsdtar";
pub const ARCHIVE_LIST_ARGUMENTS: [&str; 2] = ["-t", "-f"];
pub const ARCHIVE_EXTRACT_ARGUMENTS: [&str; 3] = ["-x", "-v", "-f"];
pub const ARCHIVE_VERBOSE_LIST_ARGUMENTS: [&str; 3] = ["-t", "-v", "-f"];
pub const ARCHIVE_VERBOSE_MARKS: [&str; 2] = ["x", "a"];
pub const ARCHIVE_LIST_SIZE_FIELD: usize = 4;
pub const ARCHIVE_DESTINATION_FLAG: &str = "-C";
pub const ARCHIVE_CREATE_ARGUMENTS: [&str; 4] = ["-a", "-c", "-v", "-f"];
pub const ARCHIVE_SHOWN_MEMBERS: usize = 400;
pub const ARCHIVE_COUNT_LIMIT: usize = 100_000;
pub const ARCHIVE_DIR_SUFFIX: char = '/';
pub const ARCHIVE_STAGING_NAME: &str = ".files-extracting";
pub const ARCHIVE_INNER_EXTENSION: &str = "tar";
pub const ARCHIVE_EXTRA_EXTENSIONS: [&str; 1] = ["iso"];
pub const MARKDOWN_EXTENSIONS: [&str; 3] = ["md", "markdown", "mdown"];
pub const MARKDOWN_BULLET: &str = "•";
pub const MARKDOWN_TASK_DONE: &str = "[x]";
pub const MARKDOWN_TASK_OPEN: &str = "[ ]";
pub const MARKDOWN_CELL_SEPARATOR: &str = "  │  ";
pub const ARCHIVE_BROWSE_DIR: &str = "files/archives";
pub const PREVIEW_MEDIA_DIR: &str = "files/media";
pub const FFPROBE_COMMAND: &str = "ffprobe";
pub const FFPROBE_ARGUMENTS: [&str; 6] = [
    "-v",
    "error",
    "-show_entries",
    "format=duration:stream=codec_type,codec_name,width,height,sample_rate,channels",
    "-of",
    "json",
];
pub const FFMPEG_COMMAND: &str = "ffmpeg";
pub const FFMPEG_FRAME_SEEK: [&str; 4] = ["-v", "error", "-ss", "1"];
pub const FFMPEG_FRAME_OUTPUT: [&str; 5] = ["-frames:v", "1", "-vf", "scale=1280:-2", "-y"];
pub const MAGICK_COMMAND: &str = "magick";
pub const FONT_SPECIMEN_TEXT: &str =
    "AaBbCc 0123456789\nThe quick brown fox\njumps over the lazy dog";
pub const FONT_SPECIMEN_POINTS: &str = "64";
pub const FONT_SPECIMEN_BACKGROUND: &str = "#1A1A1A";
pub const FONT_SPECIMEN_INK: &str = "#E8E8E8";
pub const FC_SCAN_COMMAND: &str = "fc-scan";
pub const FC_SCAN_FAMILY: [&str; 2] = ["--format", "%{family[0]}"];
pub const PNG_EXTENSION: &str = "png";
pub const EXIV2_COMMAND: &str = "exiv2";
pub const EXIV2_PRINT: [&str; 2] = ["-q", "-Pkt"];
pub const EXIV2_KEY_FLAG: &str = "-K";
pub const EXIF_FIELDS: [(&str, &str); 7] = [
    ("Exif.Image.Model", "exif.camera"),
    ("Exif.Photo.LensModel", "exif.lens"),
    ("Exif.Photo.DateTimeOriginal", "exif.taken"),
    ("Exif.Photo.ExposureTime", "exif.exposure"),
    ("Exif.Photo.FNumber", "exif.aperture"),
    ("Exif.Photo.ISOSpeedRatings", "exif.iso"),
    ("Exif.Photo.FocalLength", "exif.focal"),
];
pub const OFFICE_PARTS: [(&str, &str, &[&str]); 4] = [
    ("docx", "word/document.xml", &["</w:p>"]),
    ("odt", "content.xml", &["</text:p>", "</text:h>"]),
    ("pptx", "ppt/slides/slide1.xml", &["</a:p>"]),
    ("odp", "content.xml", &["</text:p>", "</text:h>"]),
];
pub const OFFICE_EXTRACT_ARGUMENTS: [&str; 3] = ["-x", "-O", "-f"];
pub const OFFICE_MAX_BYTES: usize = 4 * 1024 * 1024;
pub const MILLIS_PER_SECOND: f64 = 1000.0;
pub const CACHE_MAX_AGE_SECS: u64 = 30 * 24 * 60 * 60;
pub const CACHE_MAX_BYTES: u64 = 2 * 1024 * 1024 * 1024;
