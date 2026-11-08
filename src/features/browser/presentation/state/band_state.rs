use gpui::{Pixels, Point, Task};

pub struct BandState {
    pub origin: Point<Pixels>,
    pub current: Point<Pixels>,
    pub pointer: Point<Pixels>,
    pub base: Vec<usize>,
    pub ticker: Option<Task<()>>,
}
