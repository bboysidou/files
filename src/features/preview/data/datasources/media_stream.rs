use std::process::{Child, Command, Stdio};

use crate::core::constants::preview::{
    FFMPEG_COMMAND, FFMPEG_FILTER_FLAG, FFMPEG_INPUT_FLAG, FFMPEG_QUIET, FFMPEG_REALTIME,
    FFMPEG_SEEK_FLAG, FFMPEG_STREAM_OUTPUT, FFPLAY_ARGUMENTS, FFPLAY_COMMAND, MILLIS_PER_SECOND,
};
use crate::core::helpers::process::audio_sockets::audio_sockets;
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::preview::domain::entities::playback_request::PlaybackRequest;

pub fn spawn_video_stream(request: &PlaybackRequest, (width, height): (u32, u32)) -> Option<Child> {
    let access = SandboxAccess {
        read: vec![request.path.clone()],
        write: Vec::new(),
    };
    sandboxed_command(FFMPEG_COMMAND, &access)
        .args(FFMPEG_QUIET)
        .arg(FFMPEG_REALTIME)
        .args([FFMPEG_SEEK_FLAG, &seek_text(request.start_ms)])
        .arg(FFMPEG_INPUT_FLAG)
        .arg(&request.path)
        .args([FFMPEG_FILTER_FLAG, &format!("scale={width}:{height}")])
        .args(FFMPEG_STREAM_OUTPUT)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()
}

pub fn spawn_audio_stream(request: &PlaybackRequest) -> Option<Child> {
    let mut read = audio_sockets();
    if read.is_empty() {
        return None;
    }
    read.push(request.path.clone());
    let access = SandboxAccess {
        read,
        write: Vec::new(),
    };
    quiet(sandboxed_command(FFPLAY_COMMAND, &access))
        .args(FFPLAY_ARGUMENTS)
        .args([FFMPEG_SEEK_FLAG, &seek_text(request.start_ms)])
        .arg(&request.path)
        .spawn()
        .ok()
}

fn quiet(mut command: Command) -> Command {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn seek_text(millis: u64) -> String {
    format!("{:.3}", millis as f64 / MILLIS_PER_SECOND)
}
