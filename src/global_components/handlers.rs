use std::rc::Rc;

use gpui::{App, Window};

pub type ClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type IndexHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
