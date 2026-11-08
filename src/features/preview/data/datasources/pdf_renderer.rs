use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::preview::{
    PDFINFO_COMMAND, PDFINFO_PAGES, PDFTOPPM_COMMAND, PREVIEW_PDF_DIR, PREVIEW_PDF_SCALE,
};
use crate::core::helpers::files::cache_name::cache_name;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;

const PNG_SUFFIX: &str = "png";

pub struct PdfRenderer;

impl PdfRenderer {
    fn target(path: &Path) -> Option<PathBuf> {
        Some(
            dirs::cache_dir()?
                .join(PREVIEW_PDF_DIR)
                .join(cache_name(path)?),
        )
    }

    pub fn first_page(&self, path: &Path) -> Option<PathBuf> {
        let prefix = Self::target(path)?;
        let image = prefix.with_extension(PNG_SUFFIX);
        if image.exists() {
            return Some(image);
        }
        let output_dir = prefix.parent()?;
        fs::create_dir_all(output_dir).ok()?;
        let access = SandboxAccess {
            read: vec![path.to_path_buf()],
            write: vec![output_dir.to_path_buf()],
        };
        let status = sandboxed_command(PDFTOPPM_COMMAND, &access)
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
        let access = SandboxAccess {
            read: vec![path.to_path_buf()],
            write: Vec::new(),
        };
        let output = sandboxed_command(PDFINFO_COMMAND, &access)
            .arg(path)
            .output()
            .ok()?;
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find_map(|line| line.strip_prefix(PDFINFO_PAGES))
            .and_then(|count| count.trim().parse().ok())
    }
}
