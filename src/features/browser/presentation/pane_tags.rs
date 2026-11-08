use gpui::{Context, Div, prelude::*};
use rust_i18n::t;

use crate::core::dependency_injection::tags::tag_store;
use crate::core::enums::tags::tag_color::TagColor;
use crate::features::browser::presentation::actions::{
    ClearTags, TagBlue, TagGreen, TagGrey, TagOrange, TagPurple, TagRed, TagYellow,
};
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::tags::domain::usecases::toggle_tag::{clear_tags, toggle_tag};

impl PaneView {
    fn apply_tags(&mut self, tag: Option<TagColor>, cx: &mut Context<Self>) {
        let targets = self.target_paths();
        if targets.is_empty() {
            return;
        }
        let store = tag_store();
        let result = match tag {
            Some(tag) => toggle_tag(store.as_ref(), &targets, tag),
            None => clear_tags(store.as_ref(), &targets),
        };
        if let Err(error) = result {
            cx.emit(PaneEvent::Message(
                t!("tags.failed", error = error).to_string(),
            ));
        }
        let focus = self.current_entry_path();
        self.reload_with_focus(focus, false, cx);
    }

    pub(super) fn with_tag_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(
                cx.listener(|this, _: &TagRed, _, cx| this.apply_tags(Some(TagColor::Red), cx)),
            )
            .on_action(
                cx.listener(|this, _: &TagOrange, _, cx| {
                    this.apply_tags(Some(TagColor::Orange), cx)
                }),
            )
            .on_action(
                cx.listener(|this, _: &TagYellow, _, cx| {
                    this.apply_tags(Some(TagColor::Yellow), cx)
                }),
            )
            .on_action(
                cx.listener(|this, _: &TagGreen, _, cx| this.apply_tags(Some(TagColor::Green), cx)),
            )
            .on_action(
                cx.listener(|this, _: &TagBlue, _, cx| this.apply_tags(Some(TagColor::Blue), cx)),
            )
            .on_action(
                cx.listener(|this, _: &TagPurple, _, cx| {
                    this.apply_tags(Some(TagColor::Purple), cx)
                }),
            )
            .on_action(
                cx.listener(|this, _: &TagGrey, _, cx| this.apply_tags(Some(TagColor::Grey), cx)),
            )
            .on_action(cx.listener(|this, _: &ClearTags, _, cx| this.apply_tags(None, cx)))
    }
}
