use gpui::{
    App, Bounds, Element, ElementId, ElementInputHandler, Entity, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, PaintQuad, Pixels, ShapedLine, SharedString, Style,
    TextAlign, TextRun, UnderlineStyle, Window, fill, point, px, relative, rgb, rgba, size,
};

use super::text_field_view::TextField;
use crate::core::theme::colors::{ACCENT, FG_DIM, TEXT_SELECTION_ALPHA};
use crate::core::theme::metrics::TEXT_CURSOR_WIDTH;

const UNDERLINE_THICKNESS: f32 = 1.0;

pub struct TextFieldElement {
    pub field: Entity<TextField>,
}

pub struct TextFieldPaint {
    line: ShapedLine,
    cursor: PaintQuad,
    selection: Option<PaintQuad>,
}

impl TextFieldElement {
    fn runs(base: TextRun, total: usize, marked: Option<&std::ops::Range<usize>>) -> Vec<TextRun> {
        let Some(marked) = marked else {
            return vec![base];
        };
        let underline = UnderlineStyle {
            color: Some(base.color),
            thickness: px(UNDERLINE_THICKNESS),
            wavy: false,
        };
        [
            TextRun {
                len: marked.start,
                ..base.clone()
            },
            TextRun {
                len: marked.end - marked.start,
                underline: Some(underline),
                ..base.clone()
            },
            TextRun {
                len: total - marked.end,
                ..base
            },
        ]
        .into_iter()
        .filter(|run| run.len > 0)
        .collect()
    }

    fn quads(
        line: &ShapedLine,
        field: &TextField,
        bounds: Bounds<Pixels>,
    ) -> (PaintQuad, Option<PaintQuad>) {
        let x = |index: usize| bounds.left() + line.x_for_index(field.shown_offset(index));
        let height = bounds.bottom() - bounds.top();
        let cursor = fill(
            Bounds::new(
                point(x(field.cursor()), bounds.top()),
                size(px(TEXT_CURSOR_WIDTH), height),
            ),
            rgb(ACCENT),
        );
        let selection = (!field.selected.is_empty()).then(|| {
            fill(
                Bounds::from_corners(
                    point(x(field.selected.start), bounds.top()),
                    point(x(field.selected.end), bounds.bottom()),
                ),
                rgba((ACCENT << 8) | TEXT_SELECTION_ALPHA),
            )
        });
        (cursor, selection)
    }
}

impl IntoElement for TextFieldElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextFieldElement {
    type RequestLayoutState = ();
    type PrepaintState = TextFieldPaint;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let field = self.field.read(cx);
        let style = window.text_style();
        let empty = field.content.is_empty();
        let (text, color): (SharedString, Hsla) = if empty {
            (field.placeholder.clone(), rgb(FG_DIM).into())
        } else {
            (field.shown_text().into(), style.color)
        };
        let base = TextRun {
            len: text.len(),
            font: style.font(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let marked = if empty {
            None
        } else {
            field
                .marked
                .as_ref()
                .map(|range| field.shown_offset(range.start)..field.shown_offset(range.end))
        };
        let runs = Self::runs(base, text.len(), marked.as_ref());
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(text, font_size, &runs, None);
        let (cursor, selection) = Self::quads(&line, field, bounds);
        let cursor = if empty {
            fill(
                Bounds::new(
                    bounds.origin,
                    size(px(TEXT_CURSOR_WIDTH), bounds.size.height),
                ),
                rgb(ACCENT),
            )
        } else {
            cursor
        };
        TextFieldPaint {
            line,
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.field.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.field.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }
        let line_height = window.line_height();
        let _ = prepaint.line.paint(
            bounds.origin,
            line_height,
            TextAlign::Left,
            None,
            window,
            cx,
        );
        if focus_handle.is_focused(window) {
            window.paint_quad(prepaint.cursor.clone());
        }
        let line = prepaint.line.clone();
        let empty = self.field.read(cx).content.is_empty();
        self.field.update(cx, |field, _| {
            field.last_layout = (!empty).then_some(line);
            field.last_bounds = Some(bounds);
        });
    }
}
