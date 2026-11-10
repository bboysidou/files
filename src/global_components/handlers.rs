use std::rc::Rc;

use std::path::PathBuf;

use gpui::{App, ClickEvent, Pixels, Point, Window};

pub type PointHandler = Rc<dyn Fn(Point<Pixels>, &mut Window, &mut App)>;

pub type ClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type IndexHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
pub type IndexPathHandler = Rc<dyn Fn(usize, &mut Window, &mut App) -> Option<PathBuf>>;
pub type IndexClickHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;
pub type PathsHandler = Rc<dyn Fn(Vec<PathBuf>, &mut Window, &mut App)>;
pub type IndexPathsHandler = Rc<dyn Fn(usize, Vec<PathBuf>, &mut Window, &mut App)>;
pub type IndexPairHandler = Rc<dyn Fn((usize, usize), &mut Window, &mut App)>;
pub type IndexPointHandler = Rc<dyn Fn(usize, Point<Pixels>, &mut Window, &mut App)>;
