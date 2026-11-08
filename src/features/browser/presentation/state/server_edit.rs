#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PendingEdit {
    pub replaces: String,
    pub name: Option<String>,
}

#[derive(Default)]
pub struct ServerEdit {
    pub pending: Option<PendingEdit>,
}
