use std::path::PathBuf;
use std::rc::Rc;

use gpui::{App, Global, Window};

use crate::core::helpers::files::tab_marker_token::tab_marker_token;

pub type MarkerHandler = Rc<dyn Fn(u64, &mut Window, &mut App)>;

pub struct MarkerDrop(pub MarkerHandler);

impl Global for MarkerDrop {}

pub fn drop_marker(paths: &[PathBuf], window: &mut Window, cx: &mut App) -> bool {
    let Some(token) = tab_marker_token(paths) else {
        return false;
    };
    if let Some(handler) = cx.try_global::<MarkerDrop>().map(|drop| drop.0.clone()) {
        handler(token, window, cx);
    }
    true
}
