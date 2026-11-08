use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use serde_json::Value;

use crate::core::constants::preview::{
    FFMPEG_COMMAND, FFMPEG_FRAME_OUTPUT, FFMPEG_FRAME_SEEK, FFPROBE_ARGUMENTS, FFPROBE_COMMAND,
    MILLIS_PER_SECOND, PNG_EXTENSION, PREVIEW_MEDIA_DIR,
};
use crate::core::helpers::files::cache_name::cache_name;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::preview::domain::entities::media_facts::{
    AudioStream, MediaFacts, VideoStream,
};

const INPUT_FLAG: &str = "-i";

pub fn probe_media(path: &Path) -> Option<MediaFacts> {
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let output = sandboxed_command(FFPROBE_COMMAND, &access)
        .args(FFPROBE_ARGUMENTS)
        .arg(path)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let json: Value = serde_json::from_slice(&output.stdout).ok()?;
    let streams = json["streams"].as_array().cloned().unwrap_or_default();
    let of_type = |kind: &str| {
        streams
            .iter()
            .find(|stream| stream["codec_type"] == kind)
            .cloned()
    };
    Some(MediaFacts {
        duration_ms: json["format"]["duration"]
            .as_str()
            .and_then(|text| text.parse::<f64>().ok())
            .map(|seconds| (seconds * MILLIS_PER_SECOND) as u64),
        video: of_type("video").map(|stream| VideoStream {
            codec: text(&stream["codec_name"]),
            width: stream["width"].as_u64().unwrap_or_default(),
            height: stream["height"].as_u64().unwrap_or_default(),
        }),
        audio: of_type("audio").map(|stream| AudioStream {
            codec: text(&stream["codec_name"]),
            sample_rate: text(&stream["sample_rate"]).parse().unwrap_or_default(),
            channels: stream["channels"].as_u64().unwrap_or_default(),
        }),
    })
}

pub fn video_frame(path: &Path) -> Option<PathBuf> {
    let dir = dirs::cache_dir()?.join(PREVIEW_MEDIA_DIR);
    let frame = dir.join(cache_name(path)?).with_extension(PNG_EXTENSION);
    if frame.exists() {
        return Some(frame);
    }
    fs::create_dir_all(&dir).ok()?;
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: vec![dir],
    };
    let status = sandboxed_command(FFMPEG_COMMAND, &access)
        .args(FFMPEG_FRAME_SEEK)
        .arg(INPUT_FLAG)
        .arg(path)
        .args(FFMPEG_FRAME_OUTPUT)
        .arg(&frame)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    (status.success() && frame.exists()).then_some(frame)
}

fn text(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_owned()
}
