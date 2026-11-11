use gpui::SharedString;

use crate::global_components::handlers::ClickHandler;

#[derive(Clone)]
pub enum ToastAction {
    Undo,
    Run(SharedString, ClickHandler),
}

#[derive(Clone)]
pub struct ToastModel {
    pub message: SharedString,
    pub action: Option<ToastAction>,
}
