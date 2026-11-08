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
}
