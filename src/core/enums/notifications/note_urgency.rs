#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum NoteUrgency {
    Low,
    #[default]
    Normal,
    Critical,
}

impl NoteUrgency {
    pub fn code(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::Critical => 2,
        }
    }
}
