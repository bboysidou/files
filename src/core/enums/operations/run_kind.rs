#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunKind {
    Elf,
    AppImage,
    Script,
    Plain,
}

impl RunKind {
    pub fn label_key(self) -> &'static str {
        match self {
            Self::Elf => "run.kind.elf",
            Self::AppImage => "run.kind.appimage",
            Self::Script => "run.kind.script",
            Self::Plain => "run.kind.plain",
        }
    }
}
