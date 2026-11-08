use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::files::extension_badge::extension_badge;
use crate::core::helpers::files::extension_color::extension_color;
use crate::core::theme::colors::ACCENT;
use crate::core::theme::metrics::FOLDER_ICON;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::file_badge::FileBadge;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct EntryIcon {
    pub entry: FileEntry,
}

impl RenderOnce for EntryIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon: AnyElement = if self.entry.is_dir {
            IconGlyph::new(Icon::Folder, ACCENT)
                .size(FOLDER_ICON)
                .into_any_element()
        } else {
            let extension = self.entry.extension.as_deref();
            FileBadge {
                text: extension_badge(extension).into(),
                color: extension_color(extension),
            }
            .into_any_element()
        };
        icon
    }
}
