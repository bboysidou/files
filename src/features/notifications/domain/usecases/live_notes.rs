use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::features::notifications::domain::entities::desktop_note::DesktopNote;
use crate::features::notifications::domain::entities::note_command::NoteCommand;
use crate::features::notifications::domain::repositories::desktop_notifier::DesktopNotifier;

static NEXT_SLOT: AtomicU64 = AtomicU64::new(1);

pub struct LiveNotes {
    sender: Mutex<Sender<NoteCommand>>,
}

impl LiveNotes {
    pub fn new(notifier: Arc<dyn DesktopNotifier>) -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || Self::serve(notifier.as_ref(), &receiver));
        Self {
            sender: Mutex::new(sender),
        }
    }

    pub fn next_slot() -> u64 {
        NEXT_SLOT.fetch_add(1, Ordering::Relaxed)
    }

    pub fn update(&self, slot: u64, note: DesktopNote) {
        self.send(NoteCommand::Update(slot, note));
    }

    pub fn finish(&self, slot: u64, note: Option<DesktopNote>) {
        self.send(NoteCommand::Finish(slot, note));
    }

    pub fn post(&self, note: DesktopNote) {
        self.finish(Self::next_slot(), Some(note));
    }

    fn send(&self, command: NoteCommand) {
        if let Ok(sender) = self.sender.lock() {
            let _ = sender.send(command);
        }
    }

    fn serve(notifier: &dyn DesktopNotifier, receiver: &Receiver<NoteCommand>) {
        let mut live: HashMap<u64, u32> = HashMap::new();
        for command in receiver {
            match command {
                NoteCommand::Update(slot, mut note) => {
                    note.replaces = live.get(&slot).copied().unwrap_or(0);
                    if let Some(id) = notifier.show(&note) {
                        live.insert(slot, id);
                    }
                }
                NoteCommand::Finish(slot, note) => {
                    if let Some(id) = live.remove(&slot) {
                        notifier.close(id);
                    }
                    if let Some(note) = note {
                        notifier.show(&note);
                    }
                }
            }
        }
    }
}
