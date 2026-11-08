use std::io::Read;

use wl_clipboard_rs::{copy, paste};

use crate::core::constants::clipboard::{
    GNOME_COPY, GNOME_CUT, KDE_COPY_FLAG, KDE_CUT_FLAG, LINE_SEPARATOR, MIME_GNOME_FILES,
    MIME_KDE_CUT, MIME_TEXT, MIME_URI_LIST, URI_LIST_SEPARATOR,
};
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::helpers::files::file_uri::file_uri;
use crate::core::helpers::files::path_from_uri::path_from_uri;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::repositories::system_clipboard::SystemClipboard;

pub struct WaylandSystemClipboard;

impl WaylandSystemClipboard {
    fn source(mime: &str, text: String) -> copy::MimeSource {
        copy::MimeSource {
            source: copy::Source::Bytes(text.into_bytes().into_boxed_slice()),
            mime_type: copy::MimeType::Specific(mime.to_string()),
        }
    }

    fn fetch(mime: &str) -> Option<String> {
        let (mut pipe, _) = paste::get_contents(
            paste::ClipboardType::Regular,
            paste::Seat::Unspecified,
            paste::MimeType::Specific(mime),
        )
        .ok()?;
        let mut text = String::new();
        pipe.read_to_string(&mut text).ok()?;
        Some(text)
    }

    fn from_gnome(text: &str) -> Option<Clipboard> {
        let mut lines = text.lines();
        let mode = match lines.next()?.trim() {
            GNOME_CUT => ClipboardMode::Cut,
            GNOME_COPY => ClipboardMode::Copy,
            _ => return None,
        };
        let paths: Vec<_> = lines.filter_map(path_from_uri).collect();
        (!paths.is_empty()).then_some(Clipboard { mode, paths })
    }

    fn from_uri_list(text: &str) -> Option<Clipboard> {
        let paths: Vec<_> = text
            .lines()
            .filter(|line| !line.starts_with('#'))
            .filter_map(path_from_uri)
            .collect();
        let cut = Self::fetch(MIME_KDE_CUT).is_some_and(|flag| flag.trim() == KDE_CUT_FLAG);
        let mode = if cut {
            ClipboardMode::Cut
        } else {
            ClipboardMode::Copy
        };
        (!paths.is_empty()).then_some(Clipboard { mode, paths })
    }
}

impl SystemClipboard for WaylandSystemClipboard {
    fn write(&self, clipboard: &Clipboard) -> bool {
        let uris: Vec<String> = clipboard.paths.iter().map(|path| file_uri(path)).collect();
        let plain: Vec<String> = clipboard
            .paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();
        let (verb, kde_flag) = match clipboard.mode {
            ClipboardMode::Copy => (GNOME_COPY, KDE_COPY_FLAG),
            ClipboardMode::Cut => (GNOME_CUT, KDE_CUT_FLAG),
        };
        let gnome = std::iter::once(verb.to_string())
            .chain(uris.iter().cloned())
            .collect::<Vec<_>>();
        let sources = vec![
            Self::source(MIME_TEXT, plain.join(LINE_SEPARATOR)),
            Self::source(MIME_GNOME_FILES, gnome.join(LINE_SEPARATOR)),
            Self::source(
                MIME_URI_LIST,
                format!("{}{URI_LIST_SEPARATOR}", uris.join(URI_LIST_SEPARATOR)),
            ),
            Self::source(MIME_KDE_CUT, kde_flag.to_string()),
        ];
        let mut options = copy::Options::new();
        options.foreground(false);
        options.copy_multi(sources).is_ok()
    }

    fn read(&self) -> Option<Clipboard> {
        Self::fetch(MIME_GNOME_FILES)
            .and_then(|text| Self::from_gnome(&text))
            .or_else(|| Self::fetch(MIME_URI_LIST).and_then(|text| Self::from_uri_list(&text)))
    }

    fn clear(&self) {
        let _ = copy::clear(copy::ClipboardType::Regular, copy::Seat::All);
    }
}
