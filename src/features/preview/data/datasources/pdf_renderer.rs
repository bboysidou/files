use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::core::constants::preview::{
    PDFINFO_COMMAND, PDFINFO_PAGES, PDFTOPPM_COMMAND, PREVIEW_PDF_DIR, PREVIEW_PDF_SCALE,
};
use crate::core::helpers::files::cache_name::cache_name;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;

const PNG_SUFFIX: &str = "png";

pub struct PdfRenderer;

impl PdfRenderer {
    fn target(path: &Path, number: usize) -> Option<PathBuf> {
        let name = format!("{}-{number}", cache_name(path)?);
        Some(dirs::cache_dir()?.join(PREVIEW_PDF_DIR).join(name))
    }

    pub fn page(&self, path: &Path, number: usize) -> Option<PathBuf> {
        let prefix = Self::target(path, number)?;
        let number = number.to_string();
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
                &number,
                "-l",
                &number,
                "-scale-to",
                PREVIEW_PDF_SCALE,
            ])
            .arg(path)
            .arg(&prefix)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
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
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .ok()?;
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find_map(|line| line.strip_prefix(PDFINFO_PAGES))
            .and_then(|count| count.trim().parse().ok())
    }
}
