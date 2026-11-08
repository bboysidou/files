use gpui::Context;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::core::helpers::files::same_device::same_device;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::drop_request::DropRequest;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;

impl BrowserView {
    pub fn drop_files(&mut self, request: DropRequest, cx: &mut Context<Self>) {
        let DropRequest {
            sources,
            destination,
            copy,
        } = request;
        let sources: Vec<_> = sources
            .into_iter()
            .filter(|source| *source != destination && source.parent() != Some(&destination))
            .collect();
        let Some(first) = sources.first() else {
            return;
        };
        let mode = if copy || !same_device(first, &destination) {
            ClipboardMode::Copy
        } else {
            ClipboardMode::Cut
        };
        let request = TransferRequest {
            sources,
            destination,
            mode,
            policy: CollisionPolicy::KeepBoth,
        };
        self.start_transfer(request, cx);
    }
}
