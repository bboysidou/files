use std::ops::Range;
use std::time::SystemTime;

use gpui::{
    AnyElement, Context, SharedString, UniformListScrollHandle, div, prelude::*, px, uniform_list,
};
use rust_i18n::t;

use crate::core::constants::files::MISSING_VALUE;
use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::format::size_text::size_text;
use crate::core::helpers::time::relative_time::relative_time;
use crate::core::theme::metrics::QUICK_LOOK_STAGE_PAD;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::preview_folder::PreviewFolder;
use crate::features::browser::presentation::components::preview_image::PreviewImage;
use crate::features::browser::presentation::components::preview_line::PreviewLine;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::global_components::empty_state::EmptyState;

impl BrowserView {
    pub(super) fn preview_stage(
        &self,
        state: &PreviewState,
        surface: PreviewSurface,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let empty = |icon: Icon, title: String, body: String| {
            EmptyState {
                icon,
                title: title.into(),
                body: body.into(),
            }
            .into_any_element()
        };
        match &state.content {
            None => div().into_any_element(),
            Some(PreviewContent::Image { path, .. }) => PreviewImage {
                path: path.clone(),
                zoom: state.zoom,
            }
            .into_any_element(),
            Some(PreviewContent::Text { lines, .. }) => uniform_list(
                surface.list_id(),
                lines.len(),
                cx.processor(move |this, range: Range<usize>, _window, _cx| {
                    this.preview_lines(surface, range)
                }),
            )
            .size_full()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .track_scroll(self.preview_scroll_for(surface))
            .into_any_element(),
            Some(PreviewContent::Folder { entries, total }) => div()
                .size_full()
                .p(px(QUICK_LOOK_STAGE_PAD))
                .child(PreviewFolder {
                    entries: entries.clone(),
                    total: *total,
                })
                .into_any_element(),
            Some(PreviewContent::Empty) => empty(
                Icon::Document,
                t!("ql.empty_file").to_string(),
                t!("ql.empty_file.body").to_string(),
            ),
            Some(PreviewContent::Unsupported) => empty(
                Icon::Document,
                t!(state.entry.kind.label_key()).to_string(),
                t!("ql.unsupported.body").to_string(),
            ),
            Some(PreviewContent::Failed) => empty(
                Icon::Document,
                t!("ql.failed.title").to_string(),
                t!("ql.failed.body").to_string(),
            ),
        }
    }

    fn preview_scroll_for(&self, surface: PreviewSurface) -> &UniformListScrollHandle {
        match surface {
            PreviewSurface::QuickLook => &self.preview_scroll,
            PreviewSurface::Column => &self.column_scroll,
        }
    }

    fn preview_lines(&mut self, surface: PreviewSurface, range: Range<usize>) -> Vec<PreviewLine> {
        let state = match surface {
            PreviewSurface::QuickLook => self.quick_look.as_ref(),
            PreviewSurface::Column => self.column_preview.as_ref(),
        };
        let Some(Some(PreviewContent::Text { lines, .. })) = state.map(|state| &state.content)
        else {
            return Vec::new();
        };
        range
            .filter_map(|index| {
                Some(PreviewLine {
                    number: index + 1,
                    text: lines.get(index)?.clone().into(),
                })
            })
            .collect()
    }

    pub(super) fn preview_facts(state: &PreviewState) -> Vec<(SharedString, SharedString)> {
        let entry = &state.entry;
        let modified = entry.modified.map_or_else(
            || MISSING_VALUE.to_string(),
            |moment| relative_time(moment, SystemTime::now()),
        );
        let mut facts = vec![
            (
                t!("ql.fact.size").to_string(),
                size_text(entry.is_dir, entry.child_count, entry.size),
            ),
            (t!("ql.fact.modified").to_string(), modified),
            (
                t!("ql.fact.kind").to_string(),
                t!(entry.kind.label_key()).to_string(),
            ),
        ];
        facts.extend(state.content.as_ref().and_then(Self::content_fact));
        facts
            .into_iter()
            .map(|(label, value)| (label.into(), value.into()))
            .collect()
    }

    fn content_fact(content: &PreviewContent) -> Option<(String, String)> {
        match content {
            PreviewContent::Image {
                dimensions: Some((width, height)),
                ..
            } => Some((
                t!("ql.fact.dimensions").to_string(),
                format!("{width} × {height}"),
            )),
            PreviewContent::Text {
                lines,
                truncated: true,
            } => Some((
                t!("ql.fact.lines").to_string(),
                t!("ql.lines_truncated", count = lines.len()).to_string(),
            )),
            PreviewContent::Text {
                lines,
                truncated: false,
            } => Some((t!("ql.fact.lines").to_string(), lines.len().to_string())),
            _ => None,
        }
    }
}
