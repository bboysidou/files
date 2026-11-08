use crate::features::preview::domain::entities::text_line::{ColorRun, TextLine};
use crate::features::search::domain::entities::content_match::ContentMatch;

pub fn match_preview_lines(hit: &ContentMatch, needle: &str, color: u32) -> Vec<TextLine> {
    hit.lines
        .iter()
        .map(|line| TextLine {
            runs: needle_runs(&line.text, needle, color),
            text: line.text.clone(),
            number: Some(line.number),
        })
        .collect()
}

fn needle_runs(text: &str, needle: &str, color: u32) -> Vec<ColorRun> {
    let lowered = text.to_lowercase();
    if lowered.len() != text.len() || needle.is_empty() {
        return Vec::new();
    }
    lowered
        .match_indices(needle)
        .map(|(start, found)| ColorRun {
            range: start..start + found.len(),
            color,
        })
        .collect()
}
