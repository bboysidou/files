use std::rc::Rc;

use gpui::{AnyElement, Context, IntoElement};

use crate::core::dependency_injection::overview::overview_source;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::overview::presentation::components::overview_page::OverviewPage;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::IndexHandler;

impl PaneView {
    pub(super) fn start_overview(&mut self, cx: &mut Context<Self>) {
        let Some(home) = self.setup.home.clone() else {
            return;
        };
        let source = overview_source();
        self.overview.task = Some(cx.spawn(async move |this, cx| {
            let report = cx
                .background_executor()
                .spawn(async move { source.report(&home) })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.overview.report = Some(report);
                cx.notify();
            });
        }));
    }

    fn open_recent(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(path) = self
            .overview
            .report
            .as_ref()
            .and_then(|report| report.recent.get(index))
            .map(|file| file.path.clone())
        else {
            return;
        };
        let Some(folder) = path.parent().map(|parent| parent.to_path_buf()) else {
            return;
        };
        let leaving = self.current_stop();
        self.history.visit(leaving);
        self.go_to(PlaceTarget::Directory(folder), Some(path), cx);
    }

    pub(super) fn overview_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_recent: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.open_recent(index, cx));
        });
        OverviewPage {
            report: self.overview.report.clone(),
            on_recent,
        }
        .into_any_element()
    }
}
