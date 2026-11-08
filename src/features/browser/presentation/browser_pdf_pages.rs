use gpui::{Context, Div, InteractiveElement};

use crate::features::browser::presentation::actions::{PreviewPageDown, PreviewPageUp};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::features::browser::presentation::state::preview_ticket::PreviewTicket;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

impl BrowserView {
    pub(super) fn with_pdf_page_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &PreviewPageDown, _, cx| {
                this.turn_pdf_page(PreviewSurface::Column, 1, cx);
            }))
            .on_action(cx.listener(|this, _: &PreviewPageUp, _, cx| {
                this.turn_pdf_page(PreviewSurface::Column, -1, cx);
            }))
    }

    pub(super) fn turn_pdf_page(
        &mut self,
        surface: PreviewSurface,
        delta: isize,
        cx: &mut Context<Self>,
    ) -> bool {
        let (state, generation) = match surface {
            PreviewSurface::QuickLook => {
                (self.quick_look.state.as_ref(), self.quick_look.generation)
            }
            PreviewSurface::Column => (self.column.state.as_ref(), self.column.generation),
        };
        let Some(state) = state else {
            return false;
        };
        let Some(PreviewContent::Pdf { index, pages, .. }) = &state.content else {
            return false;
        };
        let last = pages.unwrap_or(*index) as isize;
        let target = (*index as isize + delta).clamp(1, last.max(1)) as usize;
        if target == *index {
            return true;
        }
        let (source, path) = (self.setup.preview.clone(), state.entry.path.clone());
        let ticket = PreviewTicket {
            surface,
            generation,
        };
        let task = cx.spawn(async move |this, cx| {
            let content = cx
                .background_executor()
                .spawn(async move { source.pdf_page(&path, target) })
                .await;
            let _ = this.update(cx, |this, cx| this.finish_preview(ticket, content, cx));
        });
        match surface {
            PreviewSurface::QuickLook => self.quick_look.task = Some(task),
            PreviewSurface::Column => self.column.task = Some(task),
        }
        true
    }
}
