#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RailStop {
    Place(usize),
    Device(usize),
    Network(usize),
    Tag(usize),
}
