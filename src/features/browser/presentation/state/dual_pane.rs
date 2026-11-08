use crate::features::browser::presentation::state::tab::Tab;

#[derive(Default)]
pub struct DualPane {
    pub partner: Option<Tab>,
    pub focused: bool,
}
