use gpui::SharedString;

#[derive(Clone, PartialEq, Debug)]
pub struct JobCardModel {
    pub title: SharedString,
    pub item: SharedString,
    pub detail: SharedString,
    pub fraction: Option<f32>,
}
