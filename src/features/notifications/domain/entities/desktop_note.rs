use crate::core::constants::notifications::NOTE_DEFAULT_TIMEOUT;
use crate::core::enums::notifications::note_urgency::NoteUrgency;

#[derive(Clone, Debug)]
pub struct DesktopNote {
    pub summary: String,
    pub body: String,
    pub progress: Option<u8>,
    pub urgency: NoteUrgency,
    pub tag: Option<String>,
    pub replaces: u32,
    pub timeout_ms: i32,
    pub resident: bool,
}

impl DesktopNote {
    pub fn new(summary: String, body: String) -> Self {
        Self {
            summary,
            body,
            progress: None,
            urgency: NoteUrgency::Normal,
            tag: None,
            replaces: 0,
            timeout_ms: NOTE_DEFAULT_TIMEOUT,
            resident: false,
        }
    }
}
