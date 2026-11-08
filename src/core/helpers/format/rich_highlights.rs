use std::ops::Range;

use gpui::{FontStyle, FontWeight, HighlightStyle, UnderlineStyle, px, rgb};

use crate::core::enums::preview::inline_style::InlineStyle;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT};

const UNDERLINE_THICKNESS: f32 = 1.0;

pub fn rich_highlight(style: InlineStyle) -> HighlightStyle {
    match style {
        InlineStyle::Strong => HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            color: Some(rgb(FG_BRIGHT).into()),
            ..Default::default()
        },
        InlineStyle::Emphasis => HighlightStyle {
            font_style: Some(FontStyle::Italic),
            ..Default::default()
        },
        InlineStyle::Code => HighlightStyle {
            background_color: Some(rgb(CARD).into()),
            color: Some(rgb(FG_BRIGHT).into()),
            ..Default::default()
        },
        InlineStyle::Link => HighlightStyle {
            color: Some(rgb(ACCENT).into()),
            underline: Some(UnderlineStyle {
                thickness: px(UNDERLINE_THICKNESS),
                ..Default::default()
            }),
            ..Default::default()
        },
    }
}

pub fn color_highlight(range: Range<usize>, color: u32) -> (Range<usize>, HighlightStyle) {
    (
        range,
        HighlightStyle {
            color: Some(rgb(color).into()),
            ..Default::default()
        },
    )
}
