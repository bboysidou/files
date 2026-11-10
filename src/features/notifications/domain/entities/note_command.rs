use crate::features::notifications::domain::entities::desktop_note::DesktopNote;

pub enum NoteCommand {
    Update(u64, DesktopNote),
    Finish(u64, Option<DesktopNote>),
}
