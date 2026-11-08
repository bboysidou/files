use gpui::SharedString;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeviceItem {
    pub label: SharedString,
    pub mounted: bool,
    pub active: bool,
}
