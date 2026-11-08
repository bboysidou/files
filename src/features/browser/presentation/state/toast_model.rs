use gpui::SharedString;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ToastModel {
    pub message: SharedString,
    pub can_undo: bool,
}
