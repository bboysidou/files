use std::path::Path;

use gpui::Context;
use rust_i18n::t;

use crate::core::dependency_injection::notifications::live_notes;
use crate::features::browser::presentation::state::job_on_target::JobOnTarget;
use crate::features::notifications::domain::entities::desktop_note::DesktopNote;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::open_shells::OpenShells;

impl ShellView {
    fn job_on(&self, root: &Path, cx: &Context<Self>) -> Option<JobOnTarget> {
        if let Some(job) = self.browser.read(cx).job_on(root) {
            return Some(job);
        }
        let me = cx.entity_id();
        cx.try_global::<OpenShells>()?
            .0
            .iter()
            .filter_map(|shell| shell.view.upgrade())
            .filter(|view| view.entity_id() != me)
            .find_map(|view| view.read(cx).browser.read(cx).job_on(root))
    }

    pub(super) fn refuse_busy(&mut self, label: &str, root: &Path, cx: &mut Context<Self>) -> bool {
        let Some(job) = self.job_on(root, cx) else {
            return false;
        };
        let message = t!(
            "job.target_busy",
            label = label,
            what = job.what,
            percent = job.percent
        )
        .to_string();
        live_notes().post(DesktopNote::new(message.clone(), String::new()));
        self.report(message, cx);
        true
    }
}
