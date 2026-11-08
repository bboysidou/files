use std::rc::Rc;

use gpui::{App, ClickEvent, Pixels, Point, Window};

pub type ClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type IndexHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
pub type IndexClickHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;
pub type IndexPointHandler = Rc<dyn Fn(usize, Point<Pixels>, &mut Window, &mut App)>;
