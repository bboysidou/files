use std::rc::Rc;

use gpui::{App, Window};

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;

pub type SortHandler = Rc<dyn Fn(SortKey, &mut Window, &mut App)>;
pub type ViewHandler = Rc<dyn Fn(ViewMode, &mut Window, &mut App)>;
