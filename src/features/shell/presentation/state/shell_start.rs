use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowRole {
    Primary,
    Extra,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShellStart {
    pub launch: LaunchRequest,
    pub role: WindowRole,
    pub tabs: Option<(Vec<TabSnapshot>, usize)>,
}
