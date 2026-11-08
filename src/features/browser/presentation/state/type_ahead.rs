use std::time::Instant;

#[derive(Default)]
pub struct TypeAhead {
    pub typed: String,
    pub at: Option<Instant>,
}
