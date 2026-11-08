use std::rc::Rc;

use gpui::{
    Action, AnchoredPositionMode, AnyElement, Context, Focusable, IntoElement, MouseButton, Pixels,
    Point, SharedString, Window, anchored, deferred, div, point, prelude::*, px,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::files::is_archive::is_archive;
use crate::core::theme::metrics::{
    MENU_EDGE_MARGIN, MENU_ITEM_HEIGHT, MENU_MIN_WIDTH, MENU_PAD, MENU_SEPARATOR_MARGIN,
};
use crate::features::browser::presentation::actions::BrowseArchive;
use crate::features::browser::presentation::actions::{
    CompressTarGz, CompressZip, Duplicate, NewFile,
};
use crate::features::browser::presentation::actions::{
    CopyPath, CopySelection, CutSelection, EmptyTrash, ExtractHere, NewFolder, OpenEntry,
    OpenTerminal, OpenWith, PasteClipboard, QuickLook, RenameEntry, TrashSelection,
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

fn entry(label: &str, icon: Icon, shortcut: &'static str, action: Box<dyn Action>) -> MenuEntry {
    MenuEntry {
        label: t!(label).to_string().into(),
        icon,
        shortcut: SharedString::new_static(shortcut),
        action,
        enabled: true,
        danger: false,
        separated: false,
    }
}

fn folder_entries(has_directory: bool, archive: bool, has_entry: bool) -> [MenuEntry; 9] {
    let new_folder = entry(
        "menu.new_folder",
        Icon::NewFolder,
        "Ctrl+Shift+N",
        Box::new(NewFolder),
    );
    let new_file = entry("menu.new_file", Icon::Document, "N", Box::new(NewFile));
    let duplicate = entry("menu.duplicate", Icon::Copy, "Ctrl+D", Box::new(Duplicate));
    let terminal = entry("menu.terminal", Icon::Terminal, "S", Box::new(OpenTerminal));
    let zip = entry(
        "menu.compress_zip",
        Icon::Download,
        "c",
        Box::new(CompressZip),
    );
    let tar = entry(
        "menu.compress_tar",
        Icon::Download,
        "",
        Box::new(CompressTarGz),
    );
    let browse = entry(
        "menu.browse_archive",
        Icon::Folder,
        "B",
        Box::new(BrowseArchive),
    );
    let extract = entry("menu.extract", Icon::Open, "e", Box::new(ExtractHere));
    let mut trash = entry("menu.trash", Icon::Trash, "dd", Box::new(TrashSelection));
    (trash.danger, trash.separated) = (true, true);
    [
        gated(new_folder, has_directory),
        gated(new_file, has_directory),
        gated(duplicate, has_entry),
        gated(terminal, has_directory),
        gated(extract, archive),
        gated(browse, archive),
        gated(zip, has_entry),
        gated(tar, has_entry),
        gated(trash, has_entry),
    ]
}

fn gated(mut item: MenuEntry, enabled: bool) -> MenuEntry {
    item.enabled = enabled;
    item
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
        let mut empty = entry("trash.empty", Icon::Trash, "", Box::new(EmptyTrash));
        (empty.danger, empty.separated) = (true, true);
        vec![restore, delete, empty]
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
        let entries = menu
            .entries
            .iter()
            .map(|item| MenuEntry {
                action: item.action.boxed_clone(),
                label: item.label.clone(),
                shortcut: item.shortcut.clone(),
                ..*item
            })
            .collect();
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = view.update(cx, |this, cx| this.select_menu(index, window, cx));
        });
        let layer = div()
            .track_focus(&self.menu_focus)
            .key_context(MENU_CONTEXT)
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
                div()
                    .absolute()
                    .left(origin.x)
                    .top(origin.y)
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
