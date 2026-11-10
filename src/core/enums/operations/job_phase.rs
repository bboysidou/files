#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum JobPhase {
    #[default]
    Working,
    Flushing,
}

impl JobPhase {
    pub fn code(self) -> u8 {
        match self {
            Self::Working => 0,
            Self::Flushing => 1,
        }
    }

    pub fn from_code(code: u8) -> Self {
        match code {
            1 => Self::Flushing,
            _ => Self::Working,
        }
    }
}
