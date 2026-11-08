use super::preview_surface::PreviewSurface;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PreviewTicket {
    pub surface: PreviewSurface,
    pub generation: usize,
}
