use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, FontWeight, IntoElement, MouseButton,
    Pixels, Size, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::theme::colors::{CARD, FG_BRIGHT, LINE_STRONG};
use crate::core::theme::metrics::LIST_CREATED_WIDTH;
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SETTINGS_PANEL_HEIGHT, SETTINGS_PANEL_WIDTH,
    SHEET_PANEL_PAD, TEXT_EMPTY_TITLE, TOOLBAR_GAP,
};
use crate::features::browser::domain::entities::list_columns::ListColumns;
use crate::features::browser::presentation::browser_settings_items::setting_control;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::settings_row::SettingsRow;
use crate::features::browser::presentation::key_bindings::SETTINGS_CONTEXT;
use crate::features::browser::presentation::state::setting_change::SettingChange;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;

const SETTINGS_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;

impl BrowserView {
    pub(super) fn open_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.open = true;
        window.focus(&self.settings.focus, cx);
        cx.notify();
    }

    pub(super) fn close_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.open = false;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn change_setting(&mut self, change: SettingChange, cx: &mut Context<Self>) {
        let mut options = self.options;
        match change {
            SettingChange::ShowHidden(on) => options.show_hidden = on,
            SettingChange::HiddenLast(on) => options.sort.hidden_last = on,
            SettingChange::SortBy(key) => options.sort.key = key,
            SettingChange::Ascending(on) => options.sort.ascending = on,
            SettingChange::View(view) => options.view = view,
            SettingChange::Column(key, on) => {
                let width = ListColumns::default()
                    .width_of(key)
                    .unwrap_or(LIST_CREATED_WIDTH);
                options.columns = options.columns.with_width(key, on.then_some(width));
            }
            SettingChange::PreviewColumn => return self.toggle_preview_column(cx),
            SettingChange::FolderViews(on) => {
                self.settings.folder_views.enabled = on;
                return cx.notify();
            }
            SettingChange::RestoreTabs(on) => {
                self.settings.restore_tabs = on;
                return cx.notify();
            }
        }
        self.apply_view_options(options, cx);
    }

    fn settings_rows(&self, cx: &Context<Self>) -> Vec<AnyElement> {
        let view = cx.entity().downgrade();
        let mut items = self.list_items();
        items.extend(self.layout_items());
        items.extend(self.column_items());
        items
            .into_iter()
            .map(|item| {
                SettingsRow {
                    label: t!(item.label).to_string().into(),
                    control: setting_control(item, view.clone()),
                }
                .into_any_element()
            })
            .collect()
    }

    fn settings_panel(&self, viewport: Size<Pixels>, cx: &Context<Self>) -> AnyElement {
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        div()
            .flex()
            .flex_col()
            .gap(px(TOOLBAR_GAP))
            .w(px(SETTINGS_PANEL_WIDTH).min(viewport.width - margin))
            .max_h(px(SETTINGS_PANEL_HEIGHT).min(viewport.height - margin))
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
                    .child(t!("settings.title").to_string()),
            )
            .child(
                div()
                    .id("settings-rows")
                    .flex()
                    .flex_col()
                    .min_h_0()
                    .overflow_y_scroll()
                    .children(self.settings_rows(cx)),
            )
            .into_any_element()
    }

    pub(super) fn settings_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.settings.open {
            return None;
        }
        let viewport = window.viewport_size();
        let panel = self.settings_panel(viewport, cx);
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_settings(window, cx));
        });
        let scrim = ModalScrim {
            focus: self.settings.focus.clone(),
            context: SETTINGS_CONTEXT,
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
                .with_priority(SETTINGS_PRIORITY)
                .into_any_element(),
        )
    }
}
