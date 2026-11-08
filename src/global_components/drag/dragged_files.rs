use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct DraggedFiles {
    pub entries: Rc<Vec<(PathBuf, bool)>>,
}

impl DraggedFiles {
    pub fn paths(&self) -> Vec<PathBuf> {
        self.entries.iter().map(|(path, _)| path.clone()).collect()
    }
}
