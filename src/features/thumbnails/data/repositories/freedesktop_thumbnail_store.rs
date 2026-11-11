use std::fs::{self, DirBuilder, File, Permissions};
use std::io::BufWriter;
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use md5::{Digest, Md5};

use crate::core::constants::thumbnails::{
    PRIVATE_DIR_MODE, PRIVATE_FILE_MODE, THUMB_KEY_MTIME, THUMB_KEY_SOFTWARE, THUMB_KEY_URI,
    THUMB_SOFTWARE, THUMBNAIL_APP_DIR, THUMBNAIL_EXTENSION, THUMBNAIL_FAIL_DIR,
    THUMBNAIL_LARGE_DIR, THUMBNAIL_ROOT, THUMBNAIL_SIZE, THUMBNAIL_TEMP_EXTENSION,
};
use crate::core::helpers::files::file_uri::file_uri;
use crate::features::thumbnails::data::datasources::video_still::video_still;
use crate::features::thumbnails::domain::entities::thumbnail::Thumbnail;
use crate::features::thumbnails::domain::repositories::thumbnail_store::ThumbnailStore;

const MARKER_SIDE: u32 = 1;
const RGBA_CHANNELS: usize = 4;

struct Stamp {
    uri: String,
    mtime: String,
}

pub struct FreedesktopThumbnailStore {
    root: Option<PathBuf>,
}

impl FreedesktopThumbnailStore {
    pub fn new() -> Self {
        Self {
            root: dirs::cache_dir().map(|cache| cache.join(THUMBNAIL_ROOT)),
        }
    }

    fn is_current(file: &Path, stamp: &Stamp) -> bool {
        let Ok(reader) = File::open(file).map(std::io::BufReader::new) else {
            return false;
        };
        let Ok(decoder) = png::Decoder::new(reader).read_info() else {
            return false;
        };
        let texts = &decoder.info().uncompressed_latin1_text;
        let value = |key: &str| {
            texts
                .iter()
                .find(|chunk| chunk.keyword == key)
                .map(|chunk| chunk.text.as_str())
        };
        value(THUMB_KEY_URI) == Some(stamp.uri.as_str())
            && value(THUMB_KEY_MTIME) == Some(stamp.mtime.as_str())
    }

    fn write_png(destination: &Path, size: (u32, u32), pixels: &[u8], stamp: &Stamp) -> Option<()> {
        let directory = destination.parent()?;
        DirBuilder::new()
            .recursive(true)
            .mode(PRIVATE_DIR_MODE)
            .create(directory)
            .ok()?;
        let temporary = destination.with_extension(THUMBNAIL_TEMP_EXTENSION);
        let written = (|| {
            let mut encoder = png::Encoder::new(
                BufWriter::new(File::create(&temporary).ok()?),
                size.0,
                size.1,
            );
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder
                .add_text_chunk(THUMB_KEY_URI.to_string(), stamp.uri.clone())
                .ok()?;
            encoder
                .add_text_chunk(THUMB_KEY_MTIME.to_string(), stamp.mtime.clone())
                .ok()?;
            encoder
                .add_text_chunk(THUMB_KEY_SOFTWARE.to_string(), THUMB_SOFTWARE.to_string())
                .ok()?;
            let mut writer = encoder.write_header().ok()?;
            writer.write_image_data(pixels).ok()?;
            writer.finish().ok()?;
            fs::set_permissions(&temporary, Permissions::from_mode(PRIVATE_FILE_MODE)).ok()?;
            fs::rename(&temporary, destination).ok()
        })();
        if written.is_none() {
            let _ = fs::remove_file(&temporary);
        }
        written
    }

    fn generate(source: &Path, destination: &Path, stamp: &Stamp) -> Option<()> {
        let image = Self::decode(source).or_else(|| video_still(source))?;
        let thumbnail = image.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE).to_rgba8();
        Self::write_png(
            destination,
            thumbnail.dimensions(),
            thumbnail.as_raw(),
            stamp,
        )
    }

    fn decode(source: &Path) -> Option<image::DynamicImage> {
        image::ImageReader::open(source)
            .ok()?
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()
    }

    fn mark_failed(destination: &Path, stamp: &Stamp) {
        let pixels = vec![0_u8; RGBA_CHANNELS];
        let _ = Self::write_png(destination, (MARKER_SIDE, MARKER_SIDE), &pixels, stamp);
    }
}

impl ThumbnailStore for FreedesktopThumbnailStore {
    fn thumbnail(&self, path: &Path, modified: SystemTime) -> Thumbnail {
        let Some(root) = &self.root else {
            return Thumbnail::Failed;
        };
        if path.starts_with(root) {
            return Thumbnail::Ready(path.to_path_buf());
        }
        let Ok(since_epoch) = modified.duration_since(UNIX_EPOCH) else {
            return Thumbnail::Failed;
        };
        let stamp = Stamp {
            uri: file_uri(path),
            mtime: since_epoch.as_secs().to_string(),
        };
        let name = format!(
            "{:x}.{THUMBNAIL_EXTENSION}",
            Md5::digest(stamp.uri.as_bytes())
        );
        let large = root.join(THUMBNAIL_LARGE_DIR).join(&name);
        if Self::is_current(&large, &stamp) {
            return Thumbnail::Ready(large);
        }
        let failed = root
            .join(THUMBNAIL_FAIL_DIR)
            .join(THUMBNAIL_APP_DIR)
            .join(&name);
        if Self::is_current(&failed, &stamp) {
            return Thumbnail::Failed;
        }
        match Self::generate(path, &large, &stamp) {
            Some(()) => Thumbnail::Ready(large),
            None => {
                Self::mark_failed(&failed, &stamp);
                Thumbnail::Failed
            }
        }
    }
}
