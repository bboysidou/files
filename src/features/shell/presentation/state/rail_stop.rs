#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RailStop {
    Place(usize),
    Favorite(usize),
    Device(usize),
    Phone(usize),
    Ssh(usize),
    Network(usize),
    Tag(usize),
}
