#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerSource {
    Ssh,
    Saved,
}

impl ServerSource {
    pub fn label_key(self) -> &'static str {
        match self {
            ServerSource::Ssh => "server_edit.source_ssh",
            ServerSource::Saved => "server_edit.source_saved",
        }
    }
}
