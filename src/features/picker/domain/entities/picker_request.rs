use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PickerMode {
    Open,
    Save,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PickerRequest {
    pub mode: PickerMode,
    pub title: String,
    pub multiple: bool,
    pub directory: bool,
    pub start_folder: Option<PathBuf>,
    pub suggested_name: Option<String>,
    pub accept_label: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PickerResult {
    Chosen(Vec<PathBuf>),
    Cancelled,
}
