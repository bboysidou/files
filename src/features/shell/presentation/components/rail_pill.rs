use gpui::{App, Div, IntoElement, RenderOnce, Stateful, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD_SUNKEN, FG_MUTED, LINE, ON_ACCENT};
use crate::core::theme::metrics::{
    RAIL_ITEM_COLLAPSED, SEARCH_GAP, SEARCH_HEIGHT, SEARCH_PAD_LEFT, SEARCH_PAD_RIGHT, TEXT_SMALL,
};
use crate::features::shell::presentation::components::rail_label::RailLabel;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::kbd_hint::KbdHint;

const BUTTON_HOVER_OPACITY: f32 = 0.85;

pub struct RailPillSpec {
    pub id: &'static str,
    pub label_id: &'static str,
    pub icon: Icon,
    pub label_key: &'static str,
    pub keys: &'static str,
    pub button: bool,
}

pub const SEARCH_PILL: RailPillSpec = RailPillSpec {
    id: "rail-search",
    label_id: "search-label",
    icon: Icon::Search,
    label_key: "rail.search",
    keys: "f",
    button: false,
};

pub const CONNECT_PILL: RailPillSpec = RailPillSpec {
    id: "rail-connect",
    label_id: "connect-label",
    icon: Icon::Network,
    label_key: "rail.connect",
    keys: "Ctrl+K",
    button: true,
};

#[derive(IntoElement)]
pub struct RailPill {
    pub spec: RailPillSpec,
    pub motion: RailMotion,
    pub on_click: ClickHandler,
}

impl RenderOnce for RailPill {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let spec = self.spec;
        let field = div()
            .id(spec.id)
            .flex()
            .flex_shrink_0()
            .items_center()
            .pl(px(SEARCH_PAD_LEFT))
            .rounded_full()
            .map(|field| styled(field, spec.button))
            .on_click({
                let on_click = self.on_click.clone();
                move |_, window, cx| on_click(window, cx)
            })
            .child(IconGlyph::new(
                spec.icon,
                if spec.button { ON_ACCENT } else { FG_MUTED },
            ));
        if !self.motion.open {
            return field.size(px(RAIL_ITEM_COLLAPSED));
        }
        let label = div()
            .flex()
            .flex_1()
            .min_w_0()
            .text_size(px(TEXT_SMALL))
            .child(RailLabel::new(
                spec.label_id,
                t!(spec.label_key).to_string(),
                self.motion,
            ));
        field
            .h(px(SEARCH_HEIGHT))
            .w_full()
            .gap(px(SEARCH_GAP))
            .pr(px(SEARCH_PAD_RIGHT))
            .child(label)
            .when(!spec.button, |field| field.child(KbdHint::new(spec.keys)))
    }
}

fn styled(field: Stateful<Div>, button: bool) -> Stateful<Div> {
    if button {
        return field
            .bg(rgb(ACCENT))
            .text_color(rgb(ON_ACCENT))
            .cursor_pointer()
            .hover(|style| style.opacity(BUTTON_HOVER_OPACITY));
    }
    field
        .bg(rgb(CARD_SUNKEN))
        .border_1()
        .border_color(rgb(LINE))
        .text_color(rgb(FG_MUTED))
        .cursor_text()
}
