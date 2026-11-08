use super::sort_order::SortOrder;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ViewOptions {
    pub show_hidden: bool,
    pub sort: SortOrder,
}
