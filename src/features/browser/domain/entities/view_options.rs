use super::list_columns::ListColumns;
use super::sort_order::SortOrder;
use crate::core::enums::view::view_mode::ViewMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ViewOptions {
    pub show_hidden: bool,
    pub sort: SortOrder,
    pub view: ViewMode,
    pub columns: ListColumns,
}
