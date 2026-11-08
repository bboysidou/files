use gpui::{Context, Div, prelude::*};

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::presentation::actions::{
    ActivateTab1, ActivateTab2, ActivateTab3, ActivateTab4, ActivateTab5, ActivateTab6,
    ActivateTab7, ActivateTab8, ActivateTab9, CloseKeymap, CloseMenu, CloseTab, CopyPath,
    CopySelection, CutSelection, EditPath, MenuConfirm, MenuNext, MenuPrevious, NewFolder, NewTab,
    NextTab, PasteClipboard, PreviousTab, QuickLook, Redo, ShowGrid, ShowKeymap, ShowList,
    StartSearch, TogglePreview, TrashSelection, Undo,
};
use crate::features::browser::presentation::browser_view::BrowserView;

impl BrowserView {
    pub(super) fn with_actions(element: Div, cx: &mut Context<Self>) -> Div {
        let element = Self::with_overlay_actions(element, cx);
        Self::with_tab_actions(Self::with_operation_actions(element, cx), cx)
    }

    fn with_overlay_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &MenuNext, _, cx| this.step_menu(1, cx)))
            .on_action(cx.listener(|this, _: &MenuPrevious, _, cx| this.step_menu(-1, cx)))
            .on_action(
                cx.listener(|this, _: &MenuConfirm, window, cx| this.confirm_menu(window, cx)),
            )
            .on_action(cx.listener(|this, _: &CloseMenu, window, cx| this.close_menu(window, cx)))
            .on_action(cx.listener(|this, _: &ShowKeymap, window, cx| this.open_keymap(window, cx)))
            .on_action(
                cx.listener(|this, _: &CloseKeymap, window, cx| this.close_keymap(window, cx)),
            )
            .on_action(cx.listener(|this, _: &CopyPath, _, cx| this.copy_paths(cx)))
    }

    fn with_operation_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &CopySelection, _, cx| {
                this.copy_to_clipboard(ClipboardMode::Copy, cx)
            }))
            .on_action(cx.listener(|this, _: &CutSelection, _, cx| {
                this.copy_to_clipboard(ClipboardMode::Cut, cx)
            }))
            .on_action(cx.listener(|this, _: &PasteClipboard, _, cx| this.paste(cx)))
            .on_action(cx.listener(|this, _: &TrashSelection, _, cx| this.trash_selection(cx)))
            .on_action(cx.listener(|this, _: &NewFolder, _, cx| this.new_folder(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(
                cx.listener(|this, _: &StartSearch, window, cx| this.start_search(window, cx)),
            )
            .on_action(cx.listener(|this, _: &EditPath, window, cx| this.edit_path(window, cx)))
            .on_action(cx.listener(|this, _: &ShowList, _, cx| this.set_view(ViewMode::List, cx)))
            .on_action(cx.listener(|this, _: &ShowGrid, _, cx| this.set_view(ViewMode::Grid, cx)))
            .on_action(cx.listener(|this, _: &TogglePreview, _, cx| this.toggle_preview_column(cx)))
            .on_action(
                cx.listener(|this, _: &QuickLook, window, cx| this.open_quick_look(window, cx)),
            )
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
    }

    fn with_tab_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &NewTab, window, cx| this.new_tab(window, cx)))
            .on_action(cx.listener(|this, _: &CloseTab, window, cx| this.close_active(window, cx)))
            .on_action(cx.listener(|this, _: &NextTab, window, cx| this.cycle(1, window, cx)))
            .on_action(cx.listener(|this, _: &PreviousTab, window, cx| this.cycle(-1, window, cx)))
            .on_action(
                cx.listener(|this, _: &ActivateTab1, window, cx| this.activate(0, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab2, window, cx| this.activate(1, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab3, window, cx| this.activate(2, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab4, window, cx| this.activate(3, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab5, window, cx| this.activate(4, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab6, window, cx| this.activate(5, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab7, window, cx| this.activate(6, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab8, window, cx| this.activate(7, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab9, window, cx| this.activate(8, window, cx)),
            )
    }
}
