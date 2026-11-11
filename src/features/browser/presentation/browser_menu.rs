use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, IntoElement, MouseButton, Pixels, Point,
    Window, anchored, deferred, div, point, prelude::*, px,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::files::is_archive::is_archive;
use crate::core::theme::metrics::{
    MENU_EDGE_MARGIN, MENU_ITEM_HEIGHT, MENU_MIN_WIDTH, MENU_PAD, MENU_SEPARATOR_MARGIN,
};
use crate::features::browser::presentation::actions::{
    CopyPath, CopySelection, CutSelection, EmptyTrash, OpenEntry, OpenWith, PasteClipboard,
    QuickLook, RenameEntry, TrashSelection,
};
use crate::features::browser::presentation::browser_menu_entries::{
    entry, favorite_entry, folder_entries, gated,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::MENU_CONTEXT;
use crate::features::browser::presentation::state::menu_state::MenuState;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::IndexHandler;
use crate::global_components::menu::menu_entry::MenuEntry;
use crate::global_components::menu::menu_panel::MenuPanel;

const MENU_PRIORITY: usize = 2;
const DOUBLE: f32 = 2.0;

fn cloned_entries(menu: &MenuState) -> Vec<MenuEntry> {
    menu.entries
        .iter()
        .map(|item| MenuEntry {
            action: item.action.boxed_clone(),
            label: item.label.clone(),
            shortcut: item.shortcut.clone(),
            ..*item
        })
        .collect()
}

impl BrowserView {
    fn trash_entries(has_entry: bool) -> Vec<MenuEntry> {
        let mut restore = entry("trash.restore", Icon::Up, "Enter", Box::new(OpenEntry));
        restore.enabled = has_entry;
        let mut delete = entry(
            "trash.delete",
            Icon::Trash,
            "Delete",
            Box::new(TrashSelection),
        );
        (delete.enabled, delete.danger) = (has_entry, true);
        let mut empty = Self::empty_trash_entry();
        empty.separated = true;
        vec![restore, delete, empty]
    }

    fn empty_trash_entry() -> MenuEntry {
        let mut empty = entry("trash.empty", Icon::Trash, "", Box::new(EmptyTrash));
        empty.danger = true;
        empty
    }

    pub fn open_trash_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.menu = Some(MenuState {
            position,
            entries: vec![Self::empty_trash_entry()],
            hot: 0,
            apps: None,
        });
        window.focus(&self.menu_focus, cx);
        cx.notify();
    }

    fn menu_entries(&self, cx: &Context<Self>) -> Vec<MenuEntry> {
        let pane = self.active_pane().read(cx);
        let (has_entry, has_directory) =
            (pane.current_entry().is_some(), pane.directory().is_some());
        let archive = pane.current_entry().is_some_and(|entry| is_archive(&entry));
        let mut copy = entry("menu.copy", Icon::Copy, "y", Box::new(CopySelection));
        copy.separated = true;
        let mut rename = entry("menu.rename", Icon::Edit, "r", Box::new(RenameEntry));
        rename.separated = true;
        let paste = entry("menu.paste", Icon::Paste, "p", Box::new(PasteClipboard));
        let copy_path = entry(
            "menu.copy_path",
            Icon::Document,
            "Ctrl+Shift+C",
            Box::new(CopyPath),
        );
        let mut entries = vec![
            gated(
                entry("menu.open", Icon::Open, "Enter", Box::new(OpenEntry)),
                has_entry,
            ),
            gated(
                entry("menu.open_with", Icon::Open, "o", Box::new(OpenWith)),
                has_entry,
            ),
            gated(
                entry("menu.quick_look", Icon::Eye, "Space", Box::new(QuickLook)),
                has_entry,
            ),
            gated(copy, has_entry),
            gated(
                entry("menu.cut", Icon::Cut, "x", Box::new(CutSelection)),
                has_entry,
            ),
            gated(paste, has_directory),
            gated(copy_path, has_entry),
            gated(rename, has_entry),
            favorite_entry(
                self.favorite_target(cx).is_some(),
                self.is_favorite_target(cx),
            ),
        ];
        entries.extend(folder_entries(has_directory, archive, has_entry));
        entries
    }

    pub(super) fn open_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let pane = self.active_pane().read(cx);
        let entries = if *pane.location() == PlaceTarget::Trash {
            Self::trash_entries(pane.current_entry().is_some())
        } else {
            self.menu_entries(cx)
        };
        let hot = entries
            .iter()
            .position(|item| item.enabled)
            .unwrap_or_default();
        self.menu = Some(MenuState {
            position,
            entries,
            hot,
            apps: None,
        });
        window.focus(&self.menu_focus, cx);
        cx.notify();
    }

    pub(super) fn close_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.menu = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn step_menu(&mut self, delta: isize, cx: &mut Context<Self>) {
        let Some(menu) = self.menu.as_mut() else {
            return;
        };
        let count = menu.entries.len() as isize;
        let mut index = menu.hot as isize;
        for _ in 0..count {
            index = (index + delta).rem_euclid(count);
            if menu.entries[index as usize].enabled {
                menu.hot = index as usize;
                break;
            }
        }
        cx.notify();
    }

    pub(super) fn confirm_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(hot) = self.menu.as_ref().map(|menu| menu.hot) {
            self.select_menu(hot, window, cx);
        }
    }

    fn select_menu(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(menu) = self.menu.take() else {
            return;
        };
        window.focus(&self.active_pane().focus_handle(cx), cx);
        if let Some(choice) = menu.apps {
            return self.launch_choice(choice, index, cx);
        }
        if let Some(chosen) = menu
            .entries
            .into_iter()
            .nth(index)
            .filter(|item| item.enabled)
        {
            window.dispatch_action(chosen.action, cx);
        }
        cx.notify();
    }

    fn menu_origin(menu: &MenuState, viewport: gpui::Size<Pixels>) -> Point<Pixels> {
        let separators = menu.entries.iter().filter(|item| item.separated).count() as f32;
        let height = menu.entries.len() as f32 * MENU_ITEM_HEIGHT
            + separators * (MENU_SEPARATOR_MARGIN * DOUBLE + 1.0)
            + MENU_PAD * DOUBLE;
        let max_x = viewport.width - px(MENU_MIN_WIDTH + MENU_EDGE_MARGIN);
        let max_y = viewport.height - px(height + MENU_EDGE_MARGIN);
        point(menu.position.x.min(max_x), menu.position.y.min(max_y))
    }

    pub(super) fn menu_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.menu.as_ref()?;
        let viewport = window.viewport_size();
        let origin = Self::menu_origin(menu, viewport);
        let entries = cloned_entries(menu);
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = view.update(cx, |this, cx| this.select_menu(index, window, cx));
        });
        let layer = div()
            .track_focus(&self.menu_focus)
            .key_context(MENU_CONTEXT)
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.close_menu(window, cx)),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, _, window, cx| this.close_menu(window, cx)),
            )
            .child(
                anchored()
                    .position_mode(AnchoredPositionMode::Window)
                    .position(origin)
                    .snap_to_window_with_margin(px(MENU_EDGE_MARGIN))
                    .child(MenuPanel {
                        entries,
                        hot: menu.hot,
                        on_select,
                    }),
            );
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(layer);
        Some(
            deferred(overlay)
                .with_priority(MENU_PRIORITY)
                .into_any_element(),
        )
    }
}
