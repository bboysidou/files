pub const THUMBNAIL_ROOT: &str = "thumbnails";
pub const THUMBNAIL_LARGE_DIR: &str = "large";
pub const THUMBNAIL_FAIL_DIR: &str = "fail";
pub const THUMBNAIL_APP_DIR: &str = "files";
pub const THUMBNAIL_SIZE: u32 = 256;
pub const THUMBNAIL_EXTENSION: &str = "png";
pub const THUMBNAIL_TEMP_EXTENSION: &str = "png.tmp";
pub const THUMB_KEY_URI: &str = "Thumb::URI";
pub const THUMB_KEY_MTIME: &str = "Thumb::MTime";
pub const THUMB_KEY_SOFTWARE: &str = "Software";
pub const THUMB_SOFTWARE: &str = "files";
pub const PRIVATE_DIR_MODE: u32 = 0o700;
pub const PRIVATE_FILE_MODE: u32 = 0o600;
pub const DIRECT_IMAGE_EXTENSIONS: [&str; 1] = ["svg"];
pub const VIDEO_STILL_OUTPUT: [&str; 8] = [
    "-frames:v",
    "1",
    "-vf",
    "scale=256:256:force_original_aspect_ratio=decrease",
    "-c:v",
    "png",
    "-f",
    "image2pipe",
];
pub const VIDEO_STILL_TARGET: &str = "pipe:1";
