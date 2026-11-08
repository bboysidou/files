use std::rc::Rc;

use gpui::{AnyElement, Context, IntoElement, UniformListScrollHandle, Window};
use rust_i18n::t;

use crate::core::constants::preview::ZOOM_DEFAULT;
use crate::core::enums::assets::icon::Icon;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::features::browser::presentation::components::preview_column::PreviewColumn;
use crate::features::browser::presentation::components::preview_column_header::PreviewColumnHeader;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::features::browser::presentation::state::preview_ticket::PreviewTicket;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::global_components::empty_state::EmptyState;
use crate::global_components::handlers::ClickHandler;

impl BrowserView {
    pub(super) fn begin_preview(
        &mut self,
        entry: FileEntry,
        surface: PreviewSurface,
        cx: &mut Context<Self>,
    ) {
        let state = PreviewState {
            entry: entry.clone(),
            content: None,
            zoom: ZOOM_DEFAULT,
        };
        let generation = match surface {
            PreviewSurface::QuickLook => {
                self.quick_look = Some(state);
                self.preview_scroll = UniformListScrollHandle::new();
                self.quick_look_generation += 1;
                self.quick_look_generation
            }
            PreviewSurface::Column => {
                self.column_preview = Some(state);
                self.column_scroll = UniformListScrollHandle::new();
                self.column_generation += 1;
                self.column_generation
            }
        };
        let source = self.setup.preview.clone();
        let task = cx.spawn(async move |this, cx| {
            let content = cx
                .background_executor()
                .spawn(async move { source.load(&entry) })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.finish_preview(
                    PreviewTicket {
                        surface,
                        generation,
                    },
                    content,
                    cx,
                )
            });
        });
        match surface {
            PreviewSurface::QuickLook => self.quick_look_task = Some(task),
            PreviewSurface::Column => self.column_task = Some(task),
        }
        cx.notify();
    }

    fn finish_preview(
        &mut self,
        ticket: PreviewTicket,
        content: PreviewContent,
        cx: &mut Context<Self>,
    ) {
        let generation = ticket.generation;
        let (current, slot) = match ticket.surface {
            PreviewSurface::QuickLook => (self.quick_look_generation, &mut self.quick_look),
            PreviewSurface::Column => (self.column_generation, &mut self.column_preview),
        };
        if current != generation {
            return;
        }
        if let Some(state) = slot.as_mut() {
            state.content = Some(content);
            cx.notify();
        }
    }

    pub(super) fn sync_preview_column(&mut self, cx: &mut Context<Self>) {
        if !self.column_open {
            return;
        }
        match self.active_pane().read(cx).current_entry() {
            None => {
                self.column_preview = None;
                self.column_task = None;
            }
            Some(entry)
                if self
                    .column_preview
                    .as_ref()
                    .is_some_and(|state| state.entry == entry) => {}
            Some(entry) => self.begin_preview(entry, PreviewSurface::Column, cx),
        }
    }

    pub(super) fn toggle_preview_column(&mut self, cx: &mut Context<Self>) {
        self.column_open = !self.column_open;
        self.sync_compact(cx);
        if self.column_open {
            self.sync_preview_column(cx);
        } else {
            self.column_preview = None;
            self.column_task = None;
        }
        cx.notify();
    }

    pub(super) fn preview_column(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.column_open {
            return None;
        }
        let Some(state) = self.column_preview.clone() else {
            let body = EmptyState {
                icon: Icon::Panel,
                title: t!("preview.nothing.title").to_string().into(),
                body: t!("preview.nothing.body").to_string().into(),
            };
            return Some(
                PreviewColumn {
                    header: None,
                    body: body.into_any_element(),
                    facts: None,
                }
                .into_any_element(),
            );
        };
        let view = cx.entity().downgrade();
        let on_quick_look: ClickHandler = Rc::new(move |window: &mut Window, cx| {
            let _ = view.update(cx, |this, cx| this.open_quick_look(window, cx));
        });
        let header = PreviewColumnHeader {
            icon: EntryIcon {
                entry: state.entry.clone(),
            }
            .into_any_element(),
            title: state.entry.name.clone().into(),
            on_quick_look,
        };
        Some(
            PreviewColumn {
                header: Some(header),
                body: self.preview_stage(&state, PreviewSurface::Column, cx),
                facts: Some(QuickLookFacts {
                    facts: Self::preview_facts(&state),
                }),
            }
            .into_any_element(),
        )
    }

    pub(super) fn sync_compact(&mut self, cx: &mut Context<Self>) {
        let compact = self.column_open;
        for tab in &self.tabs {
            tab.pane
                .update(cx, |pane, cx| pane.set_compact(compact, cx));
        }
    }
}
