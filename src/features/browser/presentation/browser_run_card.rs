use std::path::PathBuf;

use gpui::{Context, Window};
use rust_i18n::t;

use crate::core::constants::operations::{PERMISSION_MASK, READ_BITS, READ_TO_EXEC_SHIFT};
use crate::core::enums::operations::run_kind::RunKind;
use crate::features::browser::presentation::browser_trash::ButtonSpec;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::file_operation::{FileOperation, ModeChange};
use crate::features::operations::domain::entities::run_plan::RunPlan;
use crate::global_components::confirm_dialog::ConfirmDialog;
use crate::global_components::dialog_button::DialogButton;

const RUN_CARD_LINE_BREAK: &str = "\n";

fn body(plan: &RunPlan) -> String {
    let mut lines = vec![t!(plan.kind.label_key()).to_string()];
    if let Some(line) = &plan.interpreter {
        lines.push(t!("run.interpreter", line = line).to_string());
    }
    if let Some(url) = &plan.origin {
        lines.push(t!("run.origin", url = url).to_string());
    }
    if plan.noexec_mount {
        lines.push(t!("run.noexec").to_string());
    } else if !plan.executable {
        lines.push(t!("run.not_executable").to_string());
    }
    lines.join(RUN_CARD_LINE_BREAK)
}

impl BrowserView {
    pub(super) fn open_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let plan = self.setup.run_inspector.inspect(&path);
        if plan.kind == RunKind::Plain {
            return self.setup.launcher.launch(&path);
        }
        self.confirm = Some(ConfirmState::Run(plan));
        self.focus_confirm = true;
        cx.notify();
    }

    pub(super) fn run_dialog(&self, plan: &RunPlan, cx: &mut Context<Self>) -> ConfirmDialog {
        let mut buttons: Vec<DialogButton> = vec![self.cancel_button(cx)];
        let spec = |id, label_key, choice| ButtonSpec {
            id,
            label_key,
            danger: false,
            choice,
        };
        if plan.kind == RunKind::Script {
            buttons.push(self.button(spec("run-text", "run.open_text", Self::open_as_text), cx));
        }
        if !plan.noexec_mount && !plan.executable {
            buttons.push(self.button(
                spec("run-chmod", "run.make_executable", Self::make_executable),
                cx,
            ));
        }
        if !plan.noexec_mount && plan.executable {
            buttons.push(self.button(
                spec("run-terminal", "run.in_terminal", Self::run_in_terminal),
                cx,
            ));
            buttons.push(self.button(spec("run-now", "run.run", Self::run_now), cx));
        }
        let name = plan
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        ConfirmDialog {
            title: t!("run.title", name = name).to_string().into(),
            body: body(plan).into(),
            buttons,
        }
    }

    fn take_run_plan(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<RunPlan> {
        match self.take_confirm(window, cx)? {
            ConfirmState::Run(plan) => Some(plan),
            _ => None,
        }
    }

    fn launch_plan(&mut self, in_terminal: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(plan) = self.take_run_plan(window, cx) else {
            return;
        };
        if let Err(error) = self.setup.app_launcher.run_program(&plan.path, in_terminal) {
            let reason = format!("{error:?}");
            self.show_toast(t!("run.failed", reason = reason).to_string(), false, cx);
        }
    }

    fn run_now(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.launch_plan(false, window, cx);
    }

    fn run_in_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.launch_plan(true, window, cx);
    }

    fn open_as_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(plan) = self.take_run_plan(window, cx) {
            self.setup.launcher.launch(&plan.path);
        }
    }

    fn make_executable(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(plan) = self.take_run_plan(window, cx) else {
            return;
        };
        let from = plan.mode & PERMISSION_MASK;
        let to = from | ((from & READ_BITS) >> READ_TO_EXEC_SHIFT);
        let change = ModeChange {
            path: plan.path,
            from,
            to,
        };
        self.execute(FileOperation::SetMode(change), OperationIntent::Fresh, cx);
    }
}
