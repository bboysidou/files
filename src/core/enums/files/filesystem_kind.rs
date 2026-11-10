#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilesystemKind {
    Fat,
    Exfat,
    Ntfs,
    Network,
    Other,
}

impl FilesystemKind {
    pub fn restricts_names(self) -> bool {
        matches!(self, Self::Fat | Self::Exfat)
    }
}
