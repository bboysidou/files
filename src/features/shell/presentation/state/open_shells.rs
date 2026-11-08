use gpui::{AnyWindowHandle, Global, WeakEntity};

use crate::features::shell::presentation::shell_view::ShellView;

pub struct OpenShell {
    pub view: WeakEntity<ShellView>,
    pub window: AnyWindowHandle,
}

#[derive(Default)]
pub struct OpenShells(pub Vec<OpenShell>);

impl Global for OpenShells {}
