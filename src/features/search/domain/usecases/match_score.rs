use crate::core::constants::search::{
    SCORE_EXACT, SCORE_PREFIX, SCORE_SUBSEQUENCE, SCORE_SUBSTRING,
};

pub fn match_score(name: &str, query: &str) -> Option<u32> {
    if name == query {
        return Some(SCORE_EXACT);
    }
    if name.starts_with(query) {
        return Some(SCORE_PREFIX);
    }
    if let Some(position) = name.find(query) {
        return Some(SCORE_SUBSTRING + position as u32);
    }
    let mut gaps = 0_u32;
    let mut remaining = query.chars().peekable();
    for character in name.chars() {
        match remaining.peek() {
            Some(wanted) if *wanted == character => {
                remaining.next();
            }
            Some(_) => gaps += 1,
            None => break,
        }
    }
    remaining
        .peek()
        .is_none()
        .then_some(SCORE_SUBSEQUENCE + gaps)
}
