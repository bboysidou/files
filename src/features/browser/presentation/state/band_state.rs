use gpui::{Pixels, Point};

pub struct BandState {
    pub origin: Point<Pixels>,
    pub current: Point<Pixels>,
    pub base: Vec<usize>,
}
