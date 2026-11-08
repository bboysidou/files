use std::ops::Range;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColorRun {
    pub range: Range<usize>,
    pub color: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TextLine {
    pub text: String,
    pub runs: Vec<ColorRun>,
    pub number: Option<usize>,
}
