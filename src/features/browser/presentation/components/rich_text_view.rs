use gpui::{App, IntoElement, RenderOnce, StyledText, Window};

use crate::core::helpers::format::rich_highlights::rich_highlight;
use crate::features::preview::domain::entities::markdown_block::RichText;

#[derive(IntoElement)]
pub struct RichTextView {
    pub text: RichText,
}

impl RenderOnce for RichTextView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut runs = self.text.runs;
        runs.sort_by_key(|run| run.range.start);
        let highlights = runs
            .into_iter()
            .map(|run| (run.range, rich_highlight(run.style)));
        StyledText::new(self.text.text).with_highlights(highlights)
    }
}
