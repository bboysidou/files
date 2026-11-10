use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::ops::Range;
use std::path::Path;

use similar::{DiffTag, TextDiff};

use crate::core::constants::files::{COMPARE_TEXT_MAX_BYTES, COPY_BUFFER_BYTES};
use crate::core::enums::compare::diff_kind::DiffKind;
use crate::core::helpers::files::is_image::is_image;
use crate::features::compare::domain::entities::compare_result::CompareResult;
use crate::features::compare::domain::entities::diff_row::DiffRow;
use crate::features::compare::domain::repositories::file_comparer::FileComparer;

pub struct LocalFileComparer;

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
}

fn small_text(path: &Path) -> Option<String> {
    let size = fs::metadata(path).ok()?.len();
    if size > COMPARE_TEXT_MAX_BYTES {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    if bytes.contains(&0) {
        return None;
    }
    String::from_utf8(bytes).ok()
}

fn same_bytes(left: &Path, right: &Path) -> Option<bool> {
    let (mut a, mut b) = (
        BufReader::new(File::open(left).ok()?),
        BufReader::new(File::open(right).ok()?),
    );
    let (mut first, mut second) = (vec![0_u8; COPY_BUFFER_BYTES], vec![0_u8; COPY_BUFFER_BYTES]);
    loop {
        let read = a.read(&mut first).ok()?;
        if read == 0 {
            return Some(b.read(&mut second).ok()? == 0);
        }
        if b.read_exact(&mut second[..read]).is_err() || first[..read] != second[..read] {
            return Some(false);
        }
    }
}

fn row(kind: DiffKind, left: Option<(usize, &str)>, right: Option<(usize, &str)>) -> DiffRow {
    let own = |side: Option<(usize, &str)>| side.map(|(index, text)| (index + 1, text.to_string()));
    DiffRow {
        kind,
        left: own(left),
        right: own(right),
    }
}

fn pick<'a>(lines: &[&'a str], range: Range<usize>, offset: usize) -> Option<(usize, &'a str)> {
    let index = range.clone().nth(offset)?;
    lines.get(index).map(|text| (index, *text))
}

fn diff_rows(old: &str, new: &str) -> (Vec<DiffRow>, usize) {
    let (old_lines, new_lines): (Vec<&str>, Vec<&str>) =
        (old.lines().collect(), new.lines().collect());
    let diff = TextDiff::from_lines(old, new);
    let (mut rows, mut changes) = (Vec::new(), 0);
    for op in diff.ops() {
        let (tag, old_range, new_range) = op.as_tag_tuple();
        let length = old_range.len().max(new_range.len());
        let kind = match tag {
            DiffTag::Equal => DiffKind::Same,
            DiffTag::Delete => DiffKind::Removed,
            DiffTag::Insert => DiffKind::Added,
            DiffTag::Replace => DiffKind::Changed,
        };
        if kind != DiffKind::Same {
            changes += 1;
        }
        for offset in 0..length {
            rows.push(row(
                kind,
                pick(&old_lines, old_range.clone(), offset),
                pick(&new_lines, new_range.clone(), offset),
            ));
        }
    }
    (rows, changes)
}

impl FileComparer for LocalFileComparer {
    fn compare(&self, left: &Path, right: &Path) -> CompareResult {
        if is_image(extension(left).as_deref()) && is_image(extension(right).as_deref()) {
            return CompareResult::Images {
                left: left.to_path_buf(),
                right: right.to_path_buf(),
            };
        }
        if let (Some(old), Some(new)) = (small_text(left), small_text(right)) {
            let (rows, changes) = diff_rows(&old, &new);
            return CompareResult::Text { rows, changes };
        }
        let sizes = (fs::metadata(left), fs::metadata(right));
        let (Ok(left_meta), Ok(right_meta)) = sizes else {
            return CompareResult::Unreadable;
        };
        CompareResult::Binary {
            left_size: left_meta.len(),
            right_size: right_meta.len(),
            identical: left_meta.len() == right_meta.len()
                && same_bytes(left, right).unwrap_or(false),
        }
    }
}
