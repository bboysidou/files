#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct MediaFacts {
    pub duration_ms: Option<u64>,
    pub video: Option<VideoStream>,
    pub audio: Option<AudioStream>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VideoStream {
    pub codec: String,
    pub width: u64,
    pub height: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AudioStream {
    pub codec: String,
    pub sample_rate: u64,
    pub channels: u64,
}
