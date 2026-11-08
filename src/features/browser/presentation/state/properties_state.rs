use gpui::{FocusHandle, SharedString, Task};

use crate::features::properties::domain::entities::file_properties::FileProperties;

pub struct PropertiesState {
    pub props: FileProperties,
    pub kind: SharedString,
    pub folder_size: Option<u64>,
    pub focus: FocusHandle,
    pub _task: Option<Task<()>>,
}
