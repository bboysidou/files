use crate::core::constants::files::MISSING_VALUE;
use crate::core::helpers::format::file_size::file_size;
use crate::core::helpers::format::item_count::item_count;

pub fn size_text(is_dir: bool, child_count: Option<usize>, bytes: Option<u64>) -> String {
    match (is_dir, child_count, bytes) {
        (true, _, Some(bytes)) => file_size(bytes),
        (true, Some(count), _) => item_count(count),
        (false, _, Some(bytes)) => file_size(bytes),
        _ => MISSING_VALUE.to_string(),
    }
}
