use std::path::PathBuf;

use gpui::{AnyWindowHandle, Global, WeakEntity};

use crate::features::browser::presentation::browser_view::BrowserView;

pub struct PendingTab {
    pub source: WeakEntity<BrowserView>,
    pub window: AnyWindowHandle,
    pub index: usize,
    pub token: u64,
    pub marker: PathBuf,
    pub landed_home: bool,
}

#[derive(Default)]
pub struct PendingTabSlot(pub Option<PendingTab>);

impl Global for PendingTabSlot {}
