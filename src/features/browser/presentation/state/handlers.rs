use std::rc::Rc;

use gpui::{App, Window};

use crate::core::enums::browser::sort_key::SortKey;

pub type SortHandler = Rc<dyn Fn(SortKey, &mut Window, &mut App)>;
