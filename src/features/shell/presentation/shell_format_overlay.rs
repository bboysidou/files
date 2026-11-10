use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, IntoElement, KeyDownEvent, MouseButton,
    Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::view::button_tone::ButtonTone;
use crate::core::helpers::format::file_size::file_size;
use crate::core::theme::colors::{CARD, DANGER, FG_BRIGHT, FG_SOFT, LINE_STRONG};
use crate::core::theme::metrics::{
    CONFIRM_GAP, CONFIRM_PAD, CONFIRM_WIDTH, MENU_EDGE_MARGIN, QUICK_LOOK_RADIUS, TEXT_EMPTY_TITLE,
    TEXT_SMALL,
};
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::usb_format::FormatPhase;
use crate::global_components::choice_group::ChoiceGroup;
use crate::global_components::dialog_button::DialogButton;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::menu::menu_item::MenuItem;
use crate::global_components::modal_scrim::ModalScrim;

impl ShellView {
    pub(super) fn format_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let state = self.usb_format.as_ref()?;
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_format(window, cx));
        });
        let layer = if let FormatPhase::Menu(position) = state.phase {
            self.format_menu((position, on_dismiss), window, cx)
        } else {
            ModalScrim {
                focus: self.format_focus.clone(),
                context: "UsbFormat",
                viewport: window.viewport_size(),
                on_dismiss,
                panel: self.format_panel(cx)?,
            }
            .into_any_element()
        };
        let overlay = div()
            .on_key_down(cx.listener(Self::format_key))
            .child(layer);
        Some(
            deferred(
                anchored()
                    .position_mode(AnchoredPositionMode::Window)
                    .position(point(px(0.0), px(0.0)))
                    .child(overlay),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }

    fn format_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .usb_format
            .as_ref()
            .is_some_and(|state| state.label.focus_handle(cx).is_focused(window))
        {
            return;
        }
        match event.keystroke.key.as_str() {
            "escape" => self.close_format(window, cx),
            "enter"
                if matches!(
                    self.usb_format.as_ref().map(|state| &state.phase),
                    Some(FormatPhase::Menu(_))
                ) =>
            {
                self.load_format_options(cx)
            }
            "1" | "2" | "3" | "4" => {
                if let Ok(index) = event.keystroke.key.parse::<usize>() {
                    self.select_format(index - 1, cx);
                }
            }
            _ => {}
        }
        cx.stop_propagation();
    }

    fn format_menu(
        &self,
        (position, on_dismiss): (gpui::Point<gpui::Pixels>, ClickHandler),
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |_, _, cx| {
            let _ = view.update(cx, |this, cx| this.load_format_options(cx));
        });
        let dismiss_right = on_dismiss.clone();
        div()
            .track_focus(&self.format_focus)
            .key_context("UsbFormat")
            .occlude()
            .w(window.viewport_size().width)
            .h(window.viewport_size().height)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx)
            })
            .on_mouse_down(MouseButton::Right, move |_, window, cx| {
                dismiss_right(window, cx)
            })
            .child(
                anchored()
                    .position_mode(AnchoredPositionMode::Window)
                    .position(position)
                    .snap_to_window_with_margin(px(MENU_EDGE_MARGIN))
                    .child(
                        div()
                            .p(px(CONFIRM_GAP))
                            .bg(rgb(CARD))
                            .rounded(px(QUICK_LOOK_RADIUS))
                            .border_1()
                            .border_color(rgb(LINE_STRONG))
                            .shadow_lg()
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .child(MenuItem {
                                index: 0,
                                label: t!("device.format.menu").to_string().into(),
                                icon: Icon::Usb,
                                shortcut: "".into(),
                                enabled: true,
                                danger: true,
                                hot: false,
                                on_select,
                            }),
                    ),
            )
            .into_any_element()
    }

    fn format_panel(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let state = self.usb_format.as_ref()?;
        let content = match &state.phase {
            FormatPhase::Loading => div()
                .child(t!("device.format.loading").to_string())
                .into_any_element(),
            FormatPhase::Running { progress } => self.format_progress_card(*progress),
            FormatPhase::Finished(message) => div().child(message.clone()).into_any_element(),
            FormatPhase::Choose {
                options,
                selected,
                confirm,
            } => self.format_choices(options, (*selected, *confirm), cx),
            FormatPhase::Menu(_) => return None,
        };
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(CONFIRM_GAP))
                .w(px(CONFIRM_WIDTH))
                .p(px(CONFIRM_PAD))
                .rounded(px(QUICK_LOOK_RADIUS))
                .bg(rgb(CARD))
                .border_1()
                .border_color(rgb(LINE_STRONG))
                .shadow_lg()
                .text_color(rgb(FG_SOFT))
                .text_size(px(TEXT_SMALL))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .text_size(px(TEXT_EMPTY_TITLE))
                        .text_color(rgb(FG_BRIGHT))
                        .child(t!("device.format.title", label = state.device.label).to_string()),
                )
                .child(content)
                .children(self.format_buttons(cx))
                .into_any_element(),
        )
    }

    fn format_choices(
        &self,
        options: &crate::features::devices::domain::entities::format_request::FormatOptions,
        (selected, confirm): (usize, bool),
        cx: &Context<Self>,
    ) -> AnyElement {
        let mut content = div().flex().flex_col().gap(px(CONFIRM_GAP)).child(format!(
            "{} · {}",
            options.target.block.display(),
            file_size(options.target.size)
        ));
        if options.filesystems.is_empty() {
            return content
                .child(t!("device.format.no_formats").to_string())
                .into_any_element();
        }
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.select_format(index, cx));
        });
        content = content
            .child(t!("device.format.choose").to_string())
            .child(ChoiceGroup {
                id: "usb-filesystem",
                labels: options
                    .filesystems
                    .iter()
                    .map(|filesystem| filesystem.label().into())
                    .collect(),
                selected,
                on_select,
            });
        if let Some(filesystem) = options.filesystems.get(selected) {
            content = content
                .child(t!(format!("device.format.hint.{}", filesystem.id())).to_string())
                .children(self.format_label_controls(*filesystem, cx));
            if confirm {
                let label = self
                    .usb_format
                    .as_ref()
                    .map(|state| state.device.label.as_str())
                    .unwrap_or_default();
                content = content.child(
                    div().text_color(rgb(DANGER)).child(
                        t!(
                            "device.format.warning",
                            label = label,
                            block = options.target.block.display(),
                            filesystem = filesystem.label()
                        )
                        .to_string(),
                    ),
                );
            }
        }
        content.into_any_element()
    }

    fn format_buttons(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let state = self.usb_format.as_ref()?;
        if matches!(state.phase, FormatPhase::Running { .. }) {
            return None;
        }
        let view = cx.entity().downgrade();
        let cancel: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_format(window, cx));
        });
        let label = if matches!(state.phase, FormatPhase::Finished(_)) {
            "device.format.close"
        } else {
            "device.format.cancel"
        };
        let mut buttons = div()
            .flex()
            .flex_wrap()
            .justify_end()
            .gap(px(CONFIRM_GAP))
            .child(DialogButton {
                id: "usb-cancel",
                label: t!(label).to_string().into(),
                tone: ButtonTone::Plain,
                on_click: cancel,
            });
        if let FormatPhase::Choose { confirm, .. } = &state.phase
            && self.format_label_valid(cx)
        {
            let view = cx.entity().downgrade();
            buttons = buttons.child(DialogButton {
                id: "usb-confirm",
                label: t!(if *confirm {
                    "device.format.erase"
                } else {
                    "device.format.continue"
                })
                .to_string()
                .into(),
                tone: if *confirm {
                    ButtonTone::Danger
                } else {
                    ButtonTone::Accent
                },
                on_click: Rc::new(move |_, cx| {
                    let _ = view.update(cx, |this, cx| this.confirm_format(cx));
                }),
            });
        }
        Some(buttons.into_any_element())
    }
}
