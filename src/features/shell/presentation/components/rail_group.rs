use crate::core::enums::assets::icon::Icon;
use crate::features::shell::presentation::state::rail_stop::RailStop;

pub const PHONES: RailGroup = RailGroup {
    label_key: "rail.section.phones",
    id: "phone",
    icon: Icon::Phone,
    stop: RailStop::Phone,
};
pub const SSH: RailGroup = RailGroup {
    label_key: "rail.section.ssh",
    id: "ssh",
    icon: Icon::Terminal,
    stop: RailStop::Ssh,
};

pub struct RailGroup {
    pub label_key: &'static str,
    pub id: &'static str,
    pub icon: Icon,
    pub stop: fn(usize) -> RailStop,
}
