#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ArchiveMember {
    pub path: String,
    pub is_dir: bool,
}
