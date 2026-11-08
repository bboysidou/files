use crate::core::constants::preview::ARCHIVE_EXTRA_EXTENSIONS;
use crate::core::enums::files::file_kind::FileKind;
use crate::features::browser::domain::entities::file_entry::FileEntry;

pub fn is_archive(entry: &FileEntry) -> bool {
    !entry.is_dir
        && (entry.kind == FileKind::Archive
            || entry.extension.as_deref().is_some_and(|extension| {
                ARCHIVE_EXTRA_EXTENSIONS
                    .iter()
                    .any(|known| known.eq_ignore_ascii_case(extension))
            }))
}
