#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreviewSurface {
    QuickLook,
    Column,
}

impl PreviewSurface {
    pub fn list_id(self) -> &'static str {
        match self {
            PreviewSurface::QuickLook => "quick-look-lines",
            PreviewSurface::Column => "column-lines",
        }
    }
}
