#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FileKind {
    Folder,
    Image,
    Video,
    Audio,
    Markdown,
    Document,
    Pdf,
    Source,
    Config,
    Text,
    Archive,
    DiskImage,
    Spreadsheet,
    Other,
}

impl FileKind {
    pub fn classify(extension: Option<&str>, is_dir: bool) -> Self {
        if is_dir {
            return FileKind::Folder;
        }
        extension
            .map(|extension| Self::from_extension(&extension.to_ascii_lowercase()))
            .unwrap_or(FileKind::Other)
    }

    fn from_extension(extension: &str) -> Self {
        match extension {
            "png" | "jpg" | "jpeg" | "webp" | "heic" | "gif" | "avif" | "bmp" | "svg" => {
                FileKind::Image
            }
            "mp4" | "mkv" | "webm" | "mov" | "avi" => FileKind::Video,
            "opus" | "flac" | "mp3" | "ogg" | "wav" | "m4a" => FileKind::Audio,
            "md" => FileKind::Markdown,
            "odt" | "doc" | "docx" | "rtf" => FileKind::Document,
            "pdf" => FileKind::Pdf,
            "rs" | "qml" | "lua" | "js" | "ts" | "tsx" | "sh" | "py" | "c" | "h" | "cpp" | "go"
            | "dart" => FileKind::Source,
            "toml" | "json" | "conf" | "lock" | "yaml" | "yml" | "ini" => FileKind::Config,
            "txt" | "log" => FileKind::Text,
            "gz" | "zip" | "zst" | "7z" | "xz" | "tar" | "bz2" | "rar" => FileKind::Archive,
            "iso" | "img" => FileKind::DiskImage,
            "ods" | "csv" | "xlsx" | "xls" => FileKind::Spreadsheet,
            _ => FileKind::Other,
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            FileKind::Folder => "kind.folder",
            FileKind::Image => "kind.image",
            FileKind::Video => "kind.video",
            FileKind::Audio => "kind.audio",
            FileKind::Markdown => "kind.markdown",
            FileKind::Document => "kind.document",
            FileKind::Pdf => "kind.pdf",
            FileKind::Source => "kind.source",
            FileKind::Config => "kind.config",
            FileKind::Text => "kind.text",
            FileKind::Archive => "kind.archive",
            FileKind::DiskImage => "kind.disk_image",
            FileKind::Spreadsheet => "kind.spreadsheet",
            FileKind::Other => "kind.other",
        }
    }
}
