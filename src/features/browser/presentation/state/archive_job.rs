use gpui::{Context, Window};

use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::operations::domain::entities::job_counter::JobCounter;

pub type ArchiveWork = Box<dyn FnOnce(&JobCounter) -> Result<(), ArchiveError> + Send>;
pub type ArchiveDone = Box<dyn FnOnce(&mut BrowserView, &mut Window, &mut Context<BrowserView>)>;

pub struct ArchiveJob {
    pub title_key: &'static str,
    pub work: ArchiveWork,
    pub done: ArchiveDone,
}
