use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Entity, Focusable, FontWeight, IntoElement,
    MouseButton, Subscription, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_GAP, FILTER_FIELD_PAD_X, QUICK_LOOK_MARGIN,
    QUICK_LOOK_RADIUS, SHEET_PANEL_HEIGHT, SHEET_PANEL_PAD, SHEET_PANEL_WIDTH, TEXT_EMPTY_TITLE,
    TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::key_row::KeyRow;
use crate::features::browser::presentation::key_bindings::KEYMAP_CONTEXT;
use crate::features::browser::presentation::state::keymap_rows::keymap_rows;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::modal_scrim::ModalScrim;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

const KEYMAP_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;

impl BrowserView {
    pub(super) fn create_keymap_field(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let field = cx.new(|cx| TextField::new(t!("keys.placeholder").to_string(), cx));
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => cx.notify(),
            TextFieldEvent::Submitted | TextFieldEvent::Cancelled => this.close_keymap(window, cx),
        });
        (field, events)
    }

    pub(super) fn open_keymap(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.keymap_open = true;
        self.keymap_field.update(cx, |field, cx| field.clear(cx));
        window.focus(&self.keymap_field.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn close_keymap(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.keymap_open = false;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    fn keymap_rows(&self, cx: &Context<Self>) -> Vec<AnyElement> {
        let query = self.keymap_field.read(cx).text().trim().to_lowercase();
        let rows: Vec<AnyElement> = keymap_rows(self.settings.key_preset)
            .into_iter()
            .map(|(label, keys)| (t!(label).to_string(), keys))
            .filter(|(label, keys)| {
                query.is_empty()
                    || label.to_lowercase().contains(&query)
                    || keys.to_lowercase().contains(&query)
            })
            .map(|(label, keys)| {
                KeyRow {
                    label: label.into(),
                    keys: keys.into(),
                }
                .into_any_element()
            })
            .collect();
        if !rows.is_empty() {
            return rows;
        }
        vec![
            div()
                .text_color(rgb(FG_MUTED))
                .child(t!("keys.none").to_string())
                .into_any_element(),
        ]
    }

    fn keymap_panel(&self, viewport: gpui::Size<gpui::Pixels>, cx: &Context<Self>) -> AnyElement {
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        let search = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(FILTER_FIELD_GAP))
            .h(px(CRUMBS_HEIGHT))
            .px(px(FILTER_FIELD_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(rgb(ACCENT))
            .text_color(rgb(FG_BRIGHT))
            .child(IconGlyph::new(Icon::Search, ACCENT))
            .child(self.keymap_field.clone());
        div()
            .flex()
            .flex_col()
            .gap(px(TOOLBAR_GAP))
            .w(px(SHEET_PANEL_WIDTH).min(viewport.width - margin))
            .h(px(SHEET_PANEL_HEIGHT).min(viewport.height - margin))
            .p(px(SHEET_PANEL_PAD))
            .rounded(px(QUICK_LOOK_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .shadow_lg()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .text_size(px(TEXT_EMPTY_TITLE))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(FG_BRIGHT))
                    .child(t!("keys.title").to_string()),
            )
            .child(search)
            .child(
                div()
                    .id("keymap-rows")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .children(self.keymap_rows(cx)),
            )
            .into_any_element()
    }

    pub(super) fn keymap_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.keymap_open {
            return None;
        }
        let viewport = window.viewport_size();
        let panel = self.keymap_panel(viewport, cx);
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_keymap(window, cx));
        });
        let scrim = ModalScrim {
            focus: self.keymap_focus.clone(),
            context: KEYMAP_CONTEXT,
            viewport,
            on_dismiss,
            panel,
        };
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(scrim);
        Some(
            deferred(overlay)
                .with_priority(KEYMAP_PRIORITY)
                .into_any_element(),
        )
    }
}
