use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use md5::{Digest, Md5};

use crate::core::constants::preview::{
    PDFINFO_COMMAND, PDFINFO_PAGES, PDFTOPPM_COMMAND, PREVIEW_PDF_DIR, PREVIEW_PDF_SCALE,
};
use crate::core::helpers::files::file_uri::file_uri;

const PNG_SUFFIX: &str = "png";

pub struct PdfRenderer;

impl PdfRenderer {
    fn target(path: &Path) -> Option<PathBuf> {
        let modified = fs::metadata(path).and_then(|meta| meta.modified()).ok()?;
        let stamp = format!("{}:{:?}", file_uri(path), modified);
        let name = format!("{:x}", Md5::digest(stamp.as_bytes()));
        Some(dirs::cache_dir()?.join(PREVIEW_PDF_DIR).join(name))
    }

    pub fn first_page(&self, path: &Path) -> Option<PathBuf> {
        let prefix = Self::target(path)?;
        let image = prefix.with_extension(PNG_SUFFIX);
        if image.exists() {
            return Some(image);
        }
        fs::create_dir_all(prefix.parent()?).ok()?;
        let status = Command::new(PDFTOPPM_COMMAND)
            .args([
                "-png",
                "-singlefile",
                "-f",
                "1",
                "-l",
                "1",
                "-scale-to",
                PREVIEW_PDF_SCALE,
            ])
            .arg(path)
            .arg(&prefix)
            .status()
            .ok()?;
        (status.success() && image.exists()).then_some(image)
    }

    pub fn page_count(&self, path: &Path) -> Option<usize> {
        let output = Command::new(PDFINFO_COMMAND).arg(path).output().ok()?;
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find_map(|line| line.strip_prefix(PDFINFO_PAGES))
            .and_then(|count| count.trim().parse().ok())
    }
}
