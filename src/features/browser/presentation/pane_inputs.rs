use gpui::{AppContext, Context, Entity, Window};
use rust_i18n::t;

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_inputs::PaneInputs;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

type EventHandler = fn(&mut PaneView, TextFieldEvent, &mut Window, &mut Context<PaneView>);

impl PaneView {
    pub(super) fn create_inputs(window: &mut Window, cx: &mut Context<Self>) -> PaneInputs {
        let filter = cx.new(|cx| TextField::new(t!("filter.placeholder").to_string(), cx));
        let rename = cx.new(|cx| TextField::new("", cx));
        let search = cx.new(|cx| TextField::new(t!("search.placeholder").to_string(), cx));
        let path = cx.new(|cx| TextField::new(t!("path.placeholder").to_string(), cx));
        let wiring: [(&Entity<TextField>, EventHandler); 4] = [
            (&filter, Self::on_filter_event),
            (&rename, Self::on_rename_event),
            (&search, Self::on_search_event),
            (&path, Self::on_path_event),
        ];
        let subscriptions = wiring
            .into_iter()
            .map(|(field, handler)| {
                cx.subscribe_in(field, window, move |this, _, event, window, cx| {
                    handler(this, *event, window, cx)
                })
            })
            .collect();
        PaneInputs {
            filter,
            rename,
            search,
            path,
            subscriptions,
        }
    }
}
