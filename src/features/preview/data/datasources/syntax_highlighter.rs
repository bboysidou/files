use std::path::Path;
use std::sync::OnceLock;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

use crate::core::constants::preview::{PREVIEW_HIGHLIGHT_LINES, PREVIEW_SYNTAX_THEME};
use crate::features::preview::domain::entities::text_line::{ColorRun, TextLine};

struct Engine {
    syntaxes: SyntaxSet,
    theme: Theme,
}

pub struct SyntaxHighlighter {
    engine: OnceLock<Option<Engine>>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            engine: OnceLock::new(),
        }
    }

    fn engine(&self) -> Option<&Engine> {
        self.engine
            .get_or_init(|| {
                let theme = ThemeSet::load_defaults()
                    .themes
                    .remove(PREVIEW_SYNTAX_THEME)?;
                Some(Engine {
                    syntaxes: SyntaxSet::load_defaults_newlines(),
                    theme,
                })
            })
            .as_ref()
    }

    fn syntax<'a>(engine: &'a Engine, path: &Path, first: &str) -> Option<&'a SyntaxReference> {
        let by_extension = path.extension().and_then(|extension| {
            engine
                .syntaxes
                .find_syntax_by_extension(&extension.to_string_lossy())
        });
        let by_name = path.file_name().and_then(|name| {
            engine
                .syntaxes
                .find_syntax_by_extension(&name.to_string_lossy())
        });
        by_extension
            .or(by_name)
            .or_else(|| engine.syntaxes.find_syntax_by_first_line(first))
            .filter(|syntax| syntax.name != engine.syntaxes.find_syntax_plain_text().name)
    }

    fn color_line(
        highlighter: &mut HighlightLines,
        line: &str,
        engine: &Engine,
    ) -> Option<TextLine> {
        let pieces = highlighter.highlight_line(line, &engine.syntaxes).ok()?;
        let mut text = String::new();
        let mut runs = Vec::new();
        for (style, piece) in pieces {
            let piece = piece.trim_end_matches(['\n', '\r']);
            let start = text.len();
            text.push_str(piece);
            let color = style.foreground;
            runs.push(ColorRun {
                range: start..text.len(),
                color: (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b),
            });
        }
        Some(TextLine { text, runs })
    }

    pub fn highlight(&self, path: &Path, text: &str) -> Option<Vec<TextLine>> {
        let engine = self.engine()?;
        let syntax = Self::syntax(engine, path, text.lines().next().unwrap_or_default())?;
        let mut highlighter = HighlightLines::new(syntax, &engine.theme);
        LinesWithEndings::from(text)
            .take(PREVIEW_HIGHLIGHT_LINES)
            .map(|line| Self::color_line(&mut highlighter, line, engine))
            .collect()
    }
}
